#[macro_use]
extern crate clap;
extern crate colored;
extern crate hex;

use std::io::{self, Write};

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
    let cli::Options {
        format: format_options,
        mqtt: mqtt_options,
        subscriptions,
        tui,
        mode,
    } = parse_options();

    let mqtt_options = mqtt_options
        .set_request_channel_capacity(1000)
        .set_notification_channel_capacity(1000);

    let (client, notifications) = MqttClient::start(mqtt_options).unwrap();

    let engine = Engine::new(notifications, client);

    engine.subscribe_all(subscriptions);

    if tui {
        start_tui(engine, format_options, mode)
    } else {
        start_stream(engine, format_options)
    }
}
