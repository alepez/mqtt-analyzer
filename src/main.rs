#[macro_use]
extern crate clap;
extern crate colored;
extern crate hex;

use std::io::{self, Write};

use rumqtt::{MqttClient, Notification, QoS, Receiver};

use crate::cli::parse_options;
use crate::format::{format_notification, MessageFormat};
use crate::tui::start_tui;

mod cli;
mod format;
mod tui;

fn start_stream(
    notifications: Receiver<Notification>,
    format_options: MessageFormat,
) -> Result<(), failure::Error> {
    for notification in notifications {
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
        topics,
        tui,
    } = options;

    let (mut client, notifications) = MqttClient::start(mqtt_options).unwrap();

    for topic in topics.iter() {
        client.subscribe(topic.as_str(), QoS::AtLeastOnce).unwrap();
    }

    if tui {
        start_tui(notifications, format_options)
    } else {
        start_stream(notifications, format_options)
    }
}
