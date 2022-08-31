#![cfg(all(test, fuzzing))]
#![feature(type_alias_impl_trait)]
#![feature(no_coverage)]

use std::{
    fmt::{self, Debug},
    io::{BufRead, BufReader, Write},
    marker::PhantomData,
    process::{ChildStdin, ChildStdout, Command, Stdio},
    sync::mpsc::{channel, Receiver, Sender},
};

use binprot::BinProtRead;
use fuzzcheck::builder::{FuzzTestFunction, ReturnBool};

#[derive(Debug)]
enum Error {
    Leftovers,
    Decode(binprot::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Leftovers => write!(f, "Extra bytes left"),
            Error::Decode(e) => write!(f, "Decode error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<binprot::Error> for Error {
    fn from(source: binprot::Error) -> Self {
        Self::Decode(source)
    }
}

fn try_binprot_read<T>(bytes: &[u8]) -> Result<T, Error>
where
    T: BinProtRead,
{
    let mut ptr = bytes;
    let result = T::binprot_read(&mut ptr);
    if ptr.len() != 0 {
        Err(Error::Leftovers)
    } else {
        result.map_err(Error::from)
    }
}

struct MinaFuzzTestFunction<T> {
    bytes_tx: Sender<Vec<u8>>,
    res_rx: Receiver<Result<String, String>>,
    _phantom_data: PhantomData<T>,
}

impl<T> FuzzTestFunction<Vec<u8>, [u8], ReturnBool> for MinaFuzzTestFunction<T>
where
    T: BinProtRead + Debug,
{
    type NormalizedFunction = impl Fn(&Vec<u8>) -> bool;

    #[no_coverage]
    fn test_function(self) -> Self::NormalizedFunction {
        let Self {
            bytes_tx, res_rx, ..
        } = self;
        move |bytes| {
            let rust_result = try_binprot_read::<T>(bytes);
            let ocaml_result = {
                bytes_tx
                    .send(bytes.to_vec())
                    .expect("failed to send bytes over channel");
                res_rx
                    .recv()
                    .expect("failed to receive response from mina decoder channel")
            };
            match (rust_result, ocaml_result) {
                (Ok(_), Ok(_)) | (Err(_), Err(_)) => true,
                (Err(e), _) => {
                    println!("Rust decode error {e}");
                    false
                }
                (Ok(r), Err(e)) => {
                    println!("Data: {}", hex::encode(&bytes));
                    println!("Rust result {r:#?}");
                    println!("OCaml decode error {e}");
                    false
                }
            }
        }
    }
}

fn mina_decoder_ipc(
    mut mina_input: ChildStdin,
    mina_output: ChildStdout,
    bytes_rx: Receiver<Vec<u8>>,
    res_tx: Sender<Result<String, String>>,
) {
    let mut mina_output = BufReader::new(mina_output);
    while let Ok(bytes) = bytes_rx.recv() {
        mina_input
            .write_all(&(bytes.len() as u64).to_le_bytes())
            .expect("failed to write prefix to mina decoder");
        mina_input
            .write_all(&bytes)
            .expect("failed to write bytes to mina decoder");
        mina_input.flush().unwrap();
        let mut buf = String::new();
        mina_output
            .read_line(&mut buf)
            .expect("failed to read response from mina decoder");
        let response = buf.trim_end_matches('\n');
        let res = if response.starts_with("ok") {
            Ok(String::from(response))
        } else {
            Err(String::from(response))
        };
        res_tx
            .send(res)
            .expect("cannot send mina decoder result over channel");
    }
}

#[test]
fn gossip_net_message() {
    let (bytes_tx, bytes_rx) = channel::<Vec<u8>>();
    let (res_tx, res_rx) = channel();

    let mut mina_decoder = match std::env::var("MINA_DECODER") {
        Ok(s) => match s.split_once(':') {
            Some(("exec", path)) => Command::new(path),
            Some(("docker", image)) => {
                let mut cmd = Command::new("docker");
                cmd.args(["run", "--rm", "--interactive", image]);
                cmd
            }
            Some((prefix, _)) => panic!("Unknown target kind: {prefix}"),
            None => Command::new(s),
        },
        Err(_) => panic!("`MINA_DECODER` is node defined. Should be either `exec:<path>` or `docker:<image>`"),
    };


    let mut mina_decoder = mina_decoder
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to launch mina decoder process");

    let mina_input = mina_decoder
        .stdin
        .take()
        .expect("failed to grab mina decoder stdin");
    let mina_output = mina_decoder
        .stdout
        .take()
        .expect("failed to grab mina decoder stdout");

    std::thread::spawn(move || mina_decoder_ipc(mina_input, mina_output, bytes_rx, res_tx));
    std::thread::spawn(move || {
        match mina_decoder.wait() {
            Ok(s) => if !s.success() {
                panic!("Mina decoder returned non zero status `{}`", s.code().unwrap());
            }
            Err(e) => panic!("Error running mina decoder: {e}"),
        }
    });

    let try_decode: MinaFuzzTestFunction<mina_p2p_messages::GossipNetMessage> =
        MinaFuzzTestFunction {
            bytes_tx,
            res_rx,
            _phantom_data: Default::default(),
        };

    let result = fuzzcheck::fuzz_test(try_decode)
        .default_options()
        .stop_after_first_test_failure(true)
        .launch();
    assert!(!result.found_test_failure);
}
