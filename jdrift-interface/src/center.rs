pub mod message;
pub mod element;

use crate::center::element::builder::Builder;
use crate::center::element::container::Container;
use crate::center::element::{Element, New};
use crate::center::message::Message;
use std::io;
use std::io::Cursor;
use std::net::{TcpListener, TcpStream};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;
use thiserror::Error;
use tungstenite::handshake::server::NoCallback;
use tungstenite::{accept, HandshakeError, ServerHandshake, WebSocket};
use xbinser::encoding::Encoded;

#[derive(Debug)]
pub struct Center {
    tcp_listener: TcpListener
}

#[derive(Debug)]
pub struct Session {
    socket: WebSocket<TcpStream>,
    update_render: Arc<AtomicBool>,
    pub root: Container
}

impl Session {
    pub const HEAD_CLASS: u32 = 0;
    pub const BODY_CLASS: u32 = 1;

    fn new(stream: WebSocket<TcpStream>) -> Self {
        let update_render = Arc::new(AtomicBool::new(false));
        Self {
            socket: stream,
            root: Container::new(update_render.clone()),
            update_render
        }
    }

    fn send_builder(&mut self, builder: Builder) -> tungstenite::Result<()> {
        for command in builder.get_commands().iter() { self.send(command.clone())? }
        Ok(())
    }

    fn send(&mut self, message: Message) -> tungstenite::Result<()> {
        // fixme: find stream for tungstenite
        let mut bytes = Cursor::new(vec![0u8; 0]);
        message.encode(&mut bytes).unwrap();
        self.socket.send(tungstenite::Message::Binary(bytes.into_inner()))
    }

    pub fn create<T: element::New>(&mut self) -> T {
        T::new(self.update_render.clone())
    }

    pub fn update(&mut self) -> tungstenite::Result<()> {
        self.send(Message { class: Self::BODY_CLASS, kind: message::Kind::SetText { text: "".to_string() } })?;
        
        let builder = Builder::default();
        self.root.build(&builder);

        self.send_builder(builder).expect("Failed to send builder"); // TODO: handle error
        Ok(())
    }

    /// Read and manage events until the connection is closed.
    /// todo: finish type implementation
    pub fn read(&mut self) -> Result<(), ()> {
        if self.socket.read().is_err() { Err(()) }
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

#[derive(Debug)]
struct Renderer {
    thread: JoinHandle<()>,
    session: Arc<RwLock<Session>>
}

impl Center {
    pub fn new(port: u16) -> io::Result<Self> {
        Ok(Self {
            tcp_listener: TcpListener::bind(format!("0.0.0.0:{port}"))?
        })
    }

    pub fn session(&mut self) -> Result<Session, SessionError> {
        let stream = self.tcp_listener
            .incoming()
            .next()
            .ok_or(SessionError::NoStream)?
            .map_err(SessionError::Stream)?;
        let mut session = Session::new(accept(stream).map_err(SessionError::Handshake)?);
        session.update().expect("Failed to update"); // todo: handle update error
        Ok(session)
    }
}