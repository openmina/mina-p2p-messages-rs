use std::{future::Future, io, pin::Pin};

use async_trait::async_trait;
use binprot::{BinProtRead, BinProtWrite};
use tokio::{io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt}, task::JoinError};

use crate::rpc_kernel::{Response, RpcRawBytes};

use super::{Message, RawMessage, Tag, Ver};

#[async_trait]
pub trait RpcAsyncReadExt: AsyncRead + Unpin {
    async fn read_rpc(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut len_buf = [0; 8];
        self.read_exact(&mut len_buf).await.unwrap();
        let len = u64::from_le_bytes(len_buf);
        buf.reserve_exact(len as usize);
        // println!("<<< reading {len} bytes");
        self.take(len).read_to_end(buf).await.unwrap();
        // hexdump::hexdump(&buf);
        Ok(buf.len())
    }

    async fn read_message(&mut self) -> io::Result<RawMessage> {
        let mut buf = Vec::new();
        self.read_rpc(&mut buf).await.unwrap();
        let mut r = buf.as_slice();
        let message = match RawMessage::binprot_read(&mut r) {
            Ok(v) => v,
            Err(binprot::Error::IoError(err)) => panic!("{err}"),
            Err(err) => return Err(io::Error::new(io::ErrorKind::InvalidData, err.to_string())),
        };
        Ok(message)
    }
}

impl<R: AsyncRead + Unpin + ?Sized> RpcAsyncReadExt for R {}

#[async_trait]
pub trait RpcAsyncWriteExt: AsyncWrite + Unpin {
    async fn write_rpc(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len: u64 = buf.len().try_into().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "cannot convert len to u64")
        })?;
        // println!(">>> writing {len} bytes");
        let len_buf = len.to_le_bytes();
        self.write_all(&len_buf[..]).await?;
        self.write_all(buf).await?;
        Ok(buf.len())
    }

    async fn write_message(&mut self, message: RawMessage) -> io::Result<()> {
        let mut buf = Vec::new();
        message.binprot_write(&mut buf)?;
        self.write_rpc(&buf[..]).await?;
        Ok(())
    }
}

impl<W: AsyncWrite + Unpin + ?Sized> RpcAsyncWriteExt for W {}

#[derive(Debug, thiserror::Error)]
pub enum RpcHandshakeError {
    #[error("unexpected handshake message")]
    UnexpectedMessage,
    #[error(transparent)]
    IO(#[from] io::Error),
}

impl From<RpcHandshakeError> for io::Error {
    fn from(source: RpcHandshakeError) -> Self {
        match source {
            RpcHandshakeError::UnexpectedMessage => {
                io::Error::new(io::ErrorKind::InvalidData, source.to_string())
            }
            RpcHandshakeError::IO(err) => err,
        }
    }
}

#[async_trait]
pub trait RpcAsyncStreamExt: AsyncRead + AsyncWrite + Unpin {
    async fn rpc_handshake(&mut self) -> Result<(), RpcHandshakeError> {
        let bytes = super::HANDSHAKE_BYTES;
        let mut r_bytes = Vec::with_capacity(bytes.len());
        let _size = self.read_rpc(&mut r_bytes).await?;
        // println!(">>> handshake: received {_size} bytes");
        // hexdump::hexdump(&r_bytes);
        if !bytes.eq(&r_bytes) {
            return Err(RpcHandshakeError::UnexpectedMessage);
        }
        let _ = self.write_rpc(bytes).await?;

        Ok(())
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin + ?Sized> RpcAsyncStreamExt for S {}

pub trait RpcHandler {
    fn handle(
        self,
        tag: Tag,
        version: Ver,
        data: Vec<u8>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, super::Error>> + Send>>;
}

#[derive(Debug, thiserror::Error)]
pub enum AsyncRpcError {
    #[error(transparent)]
    Handshake(#[from] RpcHandshakeError),
    #[error("unexpected rpc query message")]
    UnexpectedQuery,
    #[error("unexpected rpc response message")]
    UnexpectedResponse,
    #[error("error executing async handler: {_0}")]
    Join(#[from] JoinError),
    #[error(transparent)]
    IO(#[from] io::Error),
}

pub async fn serve_query<S, H>(mut stream: S, handler: H) -> Result<(), AsyncRpcError>
where
    S: RpcAsyncReadExt + RpcAsyncWriteExt + Send + Unpin,
    H: RpcHandler,
{
    stream.rpc_handshake().await?;
    let query = loop {
        let incoming = stream.read_message().await?;
        match incoming {
            Message::Heartbeat => {
                stream.write_message(Message::Heartbeat.into()).await?;
            },
            Message::Response(_) => {
                return Err(AsyncRpcError::UnexpectedResponse);
            }
            Message::Query(query) => break query,
        }
    };

    let (tag, ver, id, data) = query.split();
    let fut = handler.handle(tag, ver, data.into());
    let mut join_handle = tokio::spawn(fut);
    loop {
        tokio::select! {
            incoming = stream.read_message() => {
                match incoming? {
                    Message::Heartbeat => {
                        stream.write_message(Message::Heartbeat.into()).await?;
                    },
                    Message::Response(_) => {
                        return Err(AsyncRpcError::UnexpectedResponse);
                    }
                    Message::Query(_) => {
                        return Err(AsyncRpcError::UnexpectedQuery);
                    },
                }
            }
            result = &mut join_handle => {
                let response = Response::with_result(id, result?.map(RpcRawBytes::from));
                let message = response.into();
                stream.write_message(message).await?;
                return Ok(());
            }
        }
    }
}
