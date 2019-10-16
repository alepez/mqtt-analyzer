#[macro_use]
extern crate clap;
extern crate colored;
extern crate hex;

use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;

use rumqtt::MqttClient;

use crate::cli::parse_options;
use crate::engine::Engine;
use crate::format::{format_notification, MessageFormat};
use crate::tui::start_tui;

mod cli;
mod engine;
mod format;
mod tui;

fn start_stream(engine: Engine, format_options: MessageFormat) -> Result<(), failure::Error> {
    for notification in engine.notifications {
        let line = format_notification(format_options, &notification).to_color_string() + "\n";
        io::stdout().write_all(line.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }
    Ok(())
}

fn main() -> Result<(), failure::Error> {
    let options = parse_options();

    let cli::Options {
        format: format_options,
        mqtt: mqtt_options,
        subscriptions,
        tui,
    } = options;

    let (mut client, notifications) = MqttClient::start(mqtt_options).unwrap();

    let (client_tx, client_rx) = mpsc::channel();

    let engine = Engine::new(notifications, client_tx);

    thread::spawn(move || loop {
        match client_rx.recv() {
            Ok(event) => match event {
                engine::Event::Subscribe(sub) => {
                    client
                        .subscribe(sub.as_str(), rumqtt::QoS::AtLeastOnce)
                        .unwrap();
                }
                engine::Event::Unsubscribe(sub) => {
                    client.unsubscribe(sub).unwrap();
                }
            },
            Err(e) => panic!("Error"),
        }
    });

    for subscription in subscriptions.iter() {
        engine
            .tx()
            .send(engine::Event::Subscribe(subscription.clone()));
    }

    if tui {
        start_tui(engine, format_options)
    } else {
        start_stream(engine, format_options)
    }
}
