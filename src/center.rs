pub mod message;
pub mod element;

use crate::center::element::builder::Builder;
use crate::center::element::container::Container;
use crate::center::element::{Element, New};
use crate::center::message::{EventMessage, Kind, Message};
use std::{io, thread};
use std::io::Cursor;
use std::net::{TcpListener, TcpStream};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, LockResult, Mutex, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread::{spawn, JoinHandle, Thread};
use std::time::Duration;
use thiserror::Error;
use tungstenite::handshake::server::NoCallback;
use tungstenite::{accept, HandshakeError, ServerHandshake, WebSocket};
use xbinser::encoding::{Decoded, Encoded};
use crate::center::element::event::Event;

#[derive(Debug)]
pub struct Center {
    tcp_listener: TcpListener
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
pub struct Session {
    socket: Arc<RwLock<WebSocket<TcpStream>>>,
    live: Arc<AtomicBool>,
    handle: JoinHandle<()>,
    root: Arc<RwLock<Container>>
}

impl Session {
    pub const HEAD_CLASS: u32 = 0;
    pub const BODY_CLASS: u32 = 1;

    /// Read and manage events until the connection is closed.
    /// todo: finish type implementation
    pub fn tick(&self) -> Result<(), ()> {
        let mut socket = self.socket.write().unwrap();
        if let Ok(tungstenite::Message::Binary(bytes)) = socket.read() {
            let decoded = message::EventMessage::decode(&mut Cursor::new(bytes)).map_err(|_| ())?;
            Self::update(&mut socket, &self.root, Some(decoded));
            Ok(())
        } else { Err(()) }
    }

    fn send(socket: &mut RwLockWriteGuard<WebSocket<TcpStream>>, message: Message) -> tungstenite::Result<()> {
        // fixme: find stream for tungstenite and manage errors
        let mut bytes = Cursor::new(vec![0u8; 0]);
        message.encode(&mut bytes).unwrap();
        socket.send(tungstenite::Message::Binary(bytes.into_inner()));
        Ok(())
    }

    fn send_builder(socket: &mut RwLockWriteGuard<WebSocket<TcpStream>>, builder: Builder) -> tungstenite::Result<()> {
        for command in builder.get_commands().iter() { Self::send(socket, command.clone())? }
        Self::send(socket, Message { class: 0, kind: Kind::Load })
    }

    fn update(socket: &mut RwLockWriteGuard<WebSocket<TcpStream>>, root: &Arc<RwLock<Container>>, event: Option<EventMessage>) -> tungstenite::Result<()> {
        // todo: errors
        let builder = Builder::new(event);
        root.read().unwrap().build(&builder);

        Self::send_builder(socket, builder); // TODO: handle error
        Ok(())
    }

    pub fn spawn(stream: WebSocket<TcpStream>) -> Session {
        let socket = Arc::new(RwLock::new(stream));
        let live = Arc::new(AtomicBool::new(false));
        let root: Arc<RwLock<Option<Arc<RwLock<Container>>>>> = Arc::new(RwLock::new(None));

        Self::send(&mut socket.write().unwrap(), Message { class: Self::BODY_CLASS, kind: message::Kind::SetText { text: "".to_string() } });

        let thread_socket = socket.clone();
        let thread_live = live.clone();
        let thread_root = root.clone();
        let handle = spawn(move || {
            thread::park();
            let shared = thread_root.read().unwrap().clone().unwrap();
            thread::park();
            while thread_live.load(Ordering::Acquire) {
                // todo: error
                Self::update(&mut thread_socket.write().unwrap(), &shared, None);
                thread::park();
            }
        });

        let shared_root = Arc::new(RwLock::new(Container::new(handle.thread().clone())));
        *root.write().unwrap() = Some(shared_root.clone());
        handle.thread().unpark();
        Self { root: shared_root, live, handle, socket }
    }
    
    pub fn join(self) -> Result<(), ()> {
        self.handle.join().unwrap(); // todo; error
        Ok(())
    }

    pub fn get_root(&self) -> Result<RwLockWriteGuard<Container>, ()> {
        Ok(self.root.write().unwrap()) // todo: handle error
    }

    pub fn start(&self) -> Result<(), ()> {
        // todo: error
        if self.live.load(Ordering::Relaxed) { return Ok(()) }

        self.live.store(true, Ordering::Relaxed);
        Self::update(&mut self.socket.write().unwrap(), &self.root, None).expect("edit");
        self.handle.thread().unpark();
        Ok(())
    }

    pub fn stop(&self) {
        self.live.store(false, Ordering::Release);
        self.handle.thread().unpark();
    }
}

impl Center {
    pub fn new(port: u16) -> io::Result<Self> {
        Ok(Self {
            tcp_listener: TcpListener::bind(format!("0.0.0.0:{port}"))?
        })
    }

    pub fn stream(&mut self) -> Result<WebSocket<TcpStream>, SessionError> {
        let stream = self.tcp_listener
            .incoming()
            .next()
            .ok_or(SessionError::NoStream)?
            .map_err(SessionError::Stream)?;
        Ok(accept(stream).map_err(SessionError::Handshake)?)
    }
}