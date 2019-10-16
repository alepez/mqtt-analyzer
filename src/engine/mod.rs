use std::collections::HashSet;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use rumqtt::Notification;

pub enum Event {
    Subscribe(String),
    Unsubscribe(String),
}

pub struct Engine {
    pub notifications: rumqtt::Receiver<Notification>,
    tx: Sender<Event>,
    #[allow(dead_code)]
    thread: thread::JoinHandle<()>,
}

impl Engine {
    pub fn new(notifications: rumqtt::Receiver<Notification>, client_tx: Sender<Event>) -> Engine {
        let (tx, rx) = std::sync::mpsc::channel();
        let thread = thread::spawn(move || loop {
            match rx.recv() {
                Ok(event) => match event {
                    Event::Subscribe(_) => client_tx.send(event).unwrap(),
                    Event::Unsubscribe(_) => client_tx.send(event).unwrap(),
                },
                Err(_) => panic!("?"),
            };
        });

        Engine {
            notifications,
            tx,
            thread,
        }
    }

    pub fn tx(&self) -> Sender<Event> {
        self.tx.clone()
    }
}
