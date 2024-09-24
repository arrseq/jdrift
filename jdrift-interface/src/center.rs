use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{spawn, JoinHandle};
use thiserror::Error;
use tungstenite::{accept, HandshakeError, ServerHandshake, WebSocket};
use tungstenite::handshake::server::NoCallback;

#[derive(Debug)]
pub struct Center {
    tcp_listener: TcpListener,
    connection: Option<WebSocket<TcpStream>>
}

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Failed to get stream from incoming iterator")]
    NoStream,
    #[error("The stream could not be unwrapped")]
    Stream(io::Error),
    #[error("Failed to initiate handshake for websocket client")]
    Handshake(HandshakeError<ServerHandshake<TcpStream, NoCallback>>)
}

impl Center {
    pub fn new(port: u16) -> io::Result<Self> {
        Ok(Self {
            tcp_listener: TcpListener::bind(format!("0.0.0.0:{port}"))?,
            connection: None
        })
    }

    pub fn accept(&mut self, stream: TcpStream) -> Result<JoinHandle<()>, HandshakeError<ServerHandshake<TcpStream, NoCallback>>> {
        self.connection = Some(accept(stream)?);
        Ok(spawn(move || {
            
        }))
    }
    
    pub fn session(&mut self) -> Result<JoinHandle<()>, SessionError> {
        let stream = self.tcp_listener
            .incoming()
            .next()
            .ok_or(SessionError::NoStream)?
            .map_err(SessionError::Stream)?;
        self.accept(stream).map_err(SessionError::Handshake)
    }
}