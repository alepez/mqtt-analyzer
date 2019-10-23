use std::collections::HashSet;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use rumqtt::{MqttClient, Notification};

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
    pub fn new(notifications: rumqtt::Receiver<Notification>, mut client: MqttClient) -> Engine {
        let (tx, rx) = std::sync::mpsc::channel();
        let thread = thread::spawn(move || loop {
            match rx.recv() {
                Ok(event) => match event {
                    Event::Subscribe(sub) => {
                        client
                            .subscribe(sub.as_str(), rumqtt::QoS::AtLeastOnce)
                            .unwrap();
                    }
                    Event::Unsubscribe(sub) => {
                        client.unsubscribe(sub).unwrap();
                    }
                },
                Err(e) => panic!("{:?}", e),
            };
        });

        Engine {
            notifications,
            tx,
            thread,
        }
    }

    pub fn subscribe_all(&self, subscriptions: Vec<String>) {
        subscriptions
            .into_iter()
            .for_each(|subscription| self.tx().send(Event::Subscribe(subscription)).unwrap());
    }

    pub fn tx(&self) -> Sender<Event> {
        self.tx.clone()
    }
}
