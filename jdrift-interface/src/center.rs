pub mod message;
pub mod element;

use crate::center::message::Message;
use std::io;
use std::io::Cursor;
use std::net::{TcpListener, TcpStream};
use thiserror::Error;
use tungstenite::handshake::server::NoCallback;
use tungstenite::{accept, HandshakeError, ServerHandshake, WebSocket};
use xbinser::encoding::Encoded;
use crate::center::element::builder::Builder;
use crate::center::element::container::Container;
use crate::center::element::Inner;

#[derive(Debug)]
pub struct Center {
    tcp_listener: TcpListener
}

pub struct Session {
    pub socket: WebSocket<TcpStream>,
}

impl Session {
    pub const HEAD_CLASS: u32 = 0;
    pub const BODY_CLASS: u32 = 1;

    pub fn new(socket: WebSocket<TcpStream>) -> tungstenite::Result<Self> {
        let mut instance = Self {
            socket,
        };

        instance.send(Message { class: Self::BODY_CLASS, kind: message::Kind::SetText { text: "".to_string() } })?;

        let container = Container::default();
        let mut builder_root = Builder::default();

        container.build(&mut builder_root);
        container.build(&mut builder_root);

        dbg!(builder_root.get_commands());
        instance.send_builder(&mut builder_root)?;
        Ok(instance)
    }

    fn send_builder(&mut self, builder: &mut Builder) -> tungstenite::Result<()> {
        for command in builder.get_commands().iter() { self.send(command.clone())? }
        Ok(())
    }

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
        Ok(Session::new(accept(stream)?)?)
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