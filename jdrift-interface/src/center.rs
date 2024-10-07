pub mod message;
pub mod element;

use crate::center::element::builder::Builder;
use crate::center::element::Element;
use crate::center::message::Message;
use std::cell::RefCell;
use std::io;
use std::io::Cursor;
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use thiserror::Error;
use tungstenite::handshake::server::NoCallback;
use tungstenite::{accept, HandshakeError, ServerHandshake, WebSocket};
use xbinser::encoding::Encoded;

#[derive(Debug)]
pub struct Center {
    tcp_listener: TcpListener
}

pub(super) struct Inner {
    socket: WebSocket<TcpStream>
}

impl Inner {
    pub const HEAD_CLASS: u32 = 0;
    pub const BODY_CLASS: u32 = 1;

    pub fn new(socket: WebSocket<TcpStream>) -> tungstenite::Result<Self> {
        let mut instance = Self { socket };
        instance.update()?;
        Ok(instance)
    }
    
    pub(super) fn update(&mut self) -> tungstenite::Result<()> {
        self.send(Message { class: Self::BODY_CLASS, kind: message::Kind::SetText { text: "".to_string() } })?;
        Ok(())
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

pub struct Session<RootElement> {
    root: Element<RootElement>,
    inner: Rc<RefCell<Inner>>
}

impl<RootElement> Session<RootElement> {
    pub fn new(stream: WebSocket<TcpStream>, root: RootElement) -> tungstenite::Result<Self> {
        let inner = Rc::new(RefCell::new(Inner::new(stream)?));
        
        Ok(Self {
            inner: inner.clone(),
            root: Element {
                parent: None,
                inner: Box::new(root),
                session: inner,
                is_hydrated: false
            }
        })
    }

    /// Read and manage events until the connection is closed.
    /// todo: finish type implementation
    pub fn read(&mut self) -> Result<(), ()> {
        if self.inner.borrow_mut().socket.read().is_err() { Err(()) }
        else { Ok(()) }
    }
}

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Failed to get stream from incoming iterator")]
    NoStream,
    #[error("The stream could not be unwrapped")]
    Stream(io::Error),
    #[error("Failed to initiate handshake for websocket client")]
    Handshake(HandshakeError<ServerHandshake<TcpStream, NoCallback>>),
    #[error("Stream error")]
    Tungstenite(tungstenite::Error)
}

impl Center {
    pub fn new(port: u16) -> io::Result<Self> {
        Ok(Self {
            tcp_listener: TcpListener::bind(format!("0.0.0.0:{port}"))?
        })
    }

    pub fn session<RootElement>(&mut self, root: RootElement) -> Result<Session<RootElement>, SessionError> {
        let stream = self.tcp_listener
            .incoming()
            .next()
            .ok_or(SessionError::NoStream)?
            .map_err(SessionError::Stream)?;
        let session  = Session::new(accept(stream).map_err(SessionError::Handshake)?, root)
            .map_err(SessionError::Tungstenite)?;
        Ok(session)
    }
}