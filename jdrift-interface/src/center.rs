pub mod message;
pub mod element;

use crate::center::element::builder::Builder;
use crate::center::element::container::Container;
use crate::center::element::{Element, New};
use crate::center::message::{EventMessage, Message};
use std::{io, thread};
use std::io::Cursor;
use std::net::{TcpListener, TcpStream};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, LockResult, Mutex, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread::{spawn, JoinHandle, Thread};
use thiserror::Error;
use tungstenite::handshake::server::NoCallback;
use tungstenite::{accept, HandshakeError, ServerHandshake, WebSocket};
use xbinser::encoding::{Decoded, Encoded};

#[derive(Debug)]
pub struct Center {
    tcp_listener: TcpListener
}

#[derive(Debug)]
pub struct Session {
    socket: WebSocket<TcpStream>,
    pub renderer_thread: Thread,
    pub root: Container
}

impl Session {
    pub const HEAD_CLASS: u32 = 0;
    pub const BODY_CLASS: u32 = 1;

    fn new(stream: WebSocket<TcpStream>, renderer_thread: Thread) -> Self {
        Self {
            socket: stream,
            root: Container::new(renderer_thread.clone()),
            renderer_thread
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
        Ok(())
    }

    /// Read and manage events until the connection is closed.
    /// todo: finish type implementation
    fn tick(&mut self) -> Result<(), ()> {
        if let Ok(tungstenite::Message::Binary(bytes)) = self.socket.read() {
            let decoded = message::EventMessage::decode(&mut Cursor::new(bytes)).map_err(|_| ())?;
            dbg!(decoded);
            Ok(())
        } else { Err(()) }
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
    live: Arc<AtomicBool>,
    session: Arc<RwLock<Option<Session>>>,
    handle: JoinHandle<()>
}

impl Renderer {
    pub fn spawn(stream: WebSocket<TcpStream>) -> Renderer {
        let live = Arc::new(AtomicBool::new(true));
        let session: Arc<RwLock<Option<Session>>> = Arc::new(RwLock::new(None));
        
        let thread_live = live.clone();
        let thread_session = session.clone();
        let handle = spawn(move || {
            while thread_live.load(Ordering::Acquire) {
                {
                    // todo: error
                    let mut writer = thread_session.write().expect("Failed to get session");
                    let Some(session) = writer.as_mut() else { continue };
                    session.update().unwrap();
                    
                    session.tick();
                }
                thread::park();
            }
        });
        
        let new_session = Session::new(stream, handle.thread().clone());
        *session.write().unwrap() = Some(new_session);
        Self { session, live, handle }
    }
    
    pub fn get_session(&self) -> LockResult<RwLockWriteGuard<Option<Session>>> {
        self.session.write()
    }
    
    pub fn join(self) -> Result<(), ()> {
        self.handle.join().unwrap(); // todo; error
        Ok(())
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