pub mod message;
pub mod element;

use std::io;
use std::io::Cursor;
use std::net::{TcpListener, TcpStream};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;
use thiserror::Error;
use tungstenite::{accept, HandshakeError, ServerHandshake, WebSocket};
use tungstenite::handshake::server::NoCallback;
use xbinser::encoding::Encoded;
use crate::center::element::{Container, Element};
use crate::center::message::Message;

#[derive(Debug)]
pub struct Center {
    tcp_listener: TcpListener
}

pub struct Session {
    pub socket: WebSocket<TcpStream>,
    pub root: Container
}

impl Session {
    fn send(&mut self, message: Message) -> tungstenite::Result<()> {
        // fixme: find stream for tungstenite
        let mut bytes = Cursor::new(vec![0u8; 0]);
        message.encode(&mut bytes).unwrap();
        self.socket.send(tungstenite::Message::Binary(bytes.into_inner()))
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
        Ok(Session { 
            socket: accept(stream)?,
            root: Container::new()
        })
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

impl element::Tree for Session {
    fn append_child(&mut self, child: Box<dyn Element>) {
        self.root.append_child(child)
    }

    fn get_children(&self) -> &[&dyn Element] {
        self.root.get_children()
    }

    fn get_children_mut(&mut self) {
        self.root.get_children_mut()
    }
}