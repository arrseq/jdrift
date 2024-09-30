pub mod message;

use std::io;
use std::io::Cursor;
use std::net::{TcpListener, TcpStream};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;
use thiserror::Error;
use tungstenite::{accept, HandshakeError, ServerHandshake, WebSocket};
use tungstenite::handshake::server::NoCallback;
use xbinser::encoding::Encoded;
use crate::center::message::Message;

#[derive(Debug)]
pub struct Center {
    tcp_listener: TcpListener
}

#[derive(Debug)]
pub struct Session {
    pub socket: WebSocket<TcpStream>
}

impl Session {
    pub fn send(&mut self, message: Message) -> Result<(), ()> {
        // fixme: find stream for tungstenite
        let mut bytes = Cursor::new(vec![0u8; 0]);
        message.encode(&mut bytes).unwrap();
        // self.socket.send(tungstenite::Message::Binary(bytes.into_inner()));
        self.socket.send(tungstenite::Message::Binary(vec![0, 0])).map_err(|_| ())
    }
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
            tcp_listener: TcpListener::bind(format!("0.0.0.0:{port}"))?
        })
    }

    pub fn accept(&mut self, stream: TcpStream) -> Result<Session, HandshakeError<ServerHandshake<TcpStream, NoCallback>>> {
        Ok(Session { socket: accept(stream)? })
    }

    pub fn session(&mut self) -> Result<Session, SessionError> {
        let stream = self.tcp_listener
            .incoming()
            .next()
            .ok_or(SessionError::NoStream)?
            .map_err(SessionError::Stream)?;
        self.accept(stream).map_err(SessionError::Handshake)
    }
}