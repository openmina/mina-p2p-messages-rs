use ocaml_interop::{ocaml, OCamlRuntime, OCamlBytes, ToOCaml, OCaml};

ocaml! {
    pub fn decode_tx_pool_diff(bytes: OCamlBytes) -> bool;
}

#[test]
fn decode_ocaml() {
    let mut cr = OCamlRuntime::init();
    let bytes: OCaml<OCamlBytes> = (&b"0x00"[..]).to_ocaml(&mut cr);
    let bytes = bytes.root();
    decode_tx_pool_diff(&mut cr, &bytes);
}
