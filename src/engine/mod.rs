use std::collections::HashSet;

use rumqtt::{Notification, Receiver};

pub struct Engine {
    pub notifications: Receiver<Notification>,
    subscriptions: HashSet<String>,
}

impl Engine {
    pub fn new(notifications: Receiver<Notification>) -> Engine {
        Engine {
            subscriptions: HashSet::new(),
            notifications,
        }
    }

    pub fn subscribe(&mut self, sub: &String) {
        self.subscriptions.insert(sub.clone());
    }

    pub fn unsubscribe(&mut self, sub: &String) {
        self.subscriptions.remove(sub);
    }
}
