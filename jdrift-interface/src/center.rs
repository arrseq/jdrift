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
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread::{spawn, JoinHandle};
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
    pub(super) live: Arc<AtomicBool>,
    pub update_render: Arc<AtomicBool>,
    pub root: Container
}

impl Session {
    pub const HEAD_CLASS: u32 = 0;
    pub const BODY_CLASS: u32 = 1;

    fn new(stream: WebSocket<TcpStream>) -> Self {
        let update_render = Arc::new(AtomicBool::new(false));
        Self {
            socket: stream,
            live: Arc::new(AtomicBool::new(true)),
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

    pub fn update(&mut self) -> tungstenite::Result<()> {
        self.send(Message { class: Self::BODY_CLASS, kind: message::Kind::SetText { text: "".to_string() } })?;
        
        let builder = Builder::default();
        self.root.build(&builder);

        self.send_builder(builder).expect("Failed to send builder"); // TODO: handle error
        self.update_render.store(false, Ordering::Release);
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
pub struct Renderer {
    session: Arc<RwLock<Session>>,
    live: Arc<AtomicBool>,
    update_render: Arc<AtomicBool>
}

impl Renderer {
    pub fn new(session: Session) -> Self {
        Self {
            live: session.live.clone(),
            update_render: session.update_render.clone(),
            session: Arc::new(RwLock::new(session))
        }
    }
    
    pub fn spawn(&self) -> JoinHandle<()> {
        let live = self.live.clone();
        let update_render = self.update_render.clone();
        let session = self.session.clone();
        
        spawn(move || {
            while live.load(Ordering::Acquire) {
                // todo: error
                if update_render.load(Ordering::Acquire) { session.write().expect("Failed to get session").update().unwrap() }
            }
        })
    }
    
    pub fn get_session(&self) -> Arc<RwLock<Session>> {
        self.session.clone()
    }
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