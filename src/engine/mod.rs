use std::collections::HashSet;

use rumqtt::{MqttClient, Notification, Receiver};

pub struct Engine {
    pub notifications: Receiver<Notification>,
    subscriptions: HashSet<String>,
    client: MqttClient,
}

impl Engine {
    pub fn new(notifications: Receiver<Notification>, client: MqttClient) -> Engine {
        Engine {
            subscriptions: HashSet::new(),
            notifications,
            client,
        }
    }

    pub fn subscribe(&mut self, sub: &String) {
        if !self.subscriptions.contains(sub) {
            self.client
                .subscribe(sub.as_str(), rumqtt::QoS::AtLeastOnce)
                .unwrap();
            self.subscriptions.insert(sub.clone());
        }
    }

    pub fn unsubscribe(&mut self, sub: &String) {
        if self.subscriptions.contains(sub) {
            self.client.unsubscribe(sub).unwrap();
            self.subscriptions.remove(sub);
        }
    }
}
