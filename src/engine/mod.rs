use std::collections::HashSet;

use rumqtt::{MqttClient, Notification, Receiver};
use std::sync::mpsc::Sender;

pub enum Event {
    Subscribe(String),
    Unsubscribe(String),
}

pub struct Engine {
    pub notifications: Receiver<Notification>,
    subscriptions: HashSet<String>,
    client_tx: Sender<Event>,
}

impl Engine {
    pub fn new(notifications: Receiver<Notification>, client_tx: Sender<Event>) -> Engine {
        Engine {
            subscriptions: HashSet::new(),
            notifications,
            client_tx,
        }
    }

    pub fn subscribe(&mut self, sub: &String) {
        if !self.subscriptions.contains(sub) {
            self.client_tx.send(Event::Subscribe(sub.clone()));
            self.subscriptions.insert(sub.clone());
        }
    }

    pub fn unsubscribe(&mut self, sub: &String) {
        if self.subscriptions.contains(sub) {
            self.client_tx.send(Event::Unsubscribe(sub.clone()));
            self.subscriptions.remove(sub);
        }
    }
}
