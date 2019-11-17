use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;

pub enum Event {
    Subscribe(String),
    Unsubscribe(String),
}

type Subscriptions = std::collections::BTreeSet<String>;
type SharedSubscriptions = Arc<RwLock<Subscriptions>>;

pub struct Engine {
    pub notifications: rumqtt::Receiver<rumqtt::Notification>,
    pub subscriptions: SharedSubscriptions,
    tx: Sender<Event>,
    #[allow(dead_code)]
    thread: thread::JoinHandle<()>,
}

impl Engine {
    fn listen_events(
        rx: Receiver<Event>,
        mut client: rumqtt::MqttClient,
        subscriptions: SharedSubscriptions,
    ) {
        loop {
            match rx.recv() {
                Ok(event) => match event {
                    Event::Subscribe(sub) => {
                        client
                            .subscribe(sub.as_str(), rumqtt::QoS::AtLeastOnce)
                            .unwrap();

                        subscriptions
                            .write()
                            .map(|mut subscriptions| subscriptions.insert(sub))
                            .unwrap();
                    }
                    Event::Unsubscribe(sub) => {
                        subscriptions
                            .write()
                            .map(|mut subscriptions| subscriptions.remove(sub.as_str()))
                            .unwrap();

                        // TODO client.unsubscribe(sub).unwrap();
                    }
                },
                Err(e) => panic!("{:?}", e),
            }
        }
    }
    pub fn new(
        notifications: rumqtt::Receiver<rumqtt::Notification>,
        client: rumqtt::MqttClient,
    ) -> Engine {
        let (tx, rx) = std::sync::mpsc::channel();
        let subscriptions = SharedSubscriptions::new(RwLock::new(Subscriptions::new()));
        let subscriptions2 = subscriptions.clone();
        let thread = thread::spawn(move || Self::listen_events(rx, client, subscriptions2));

        Engine {
            subscriptions,
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
