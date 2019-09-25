#[macro_use]
extern crate clap;
extern crate colored;
extern crate hex;

use std::io::{self, Write};

use colored::Colorize;
use rumqtt::{MqttClient, Notification, QoS};

use crate::cli::parse_options;
use crate::format::format_message;

mod cli;
mod format;

fn main() {
    let options = parse_options();

    let cli::Options {
        format: format_options,
        mqtt: mqtt_options,
        topics,
    } = options;

    let (mut client, notifications) = MqttClient::start(mqtt_options).unwrap();

    for topic in topics.iter() {
        client.subscribe(topic.as_str(), QoS::AtLeastOnce).unwrap();
    }

    for notification in notifications {
        match notification {
            Notification::Publish(msg) => {
                io::stdout()
                    .write_all(format_message(format_options, &msg).as_bytes())
                    .unwrap();
            }
            Notification::Disconnection => {
                println!("{}", "Disconnected!".red());
            }
            Notification::Reconnection => {
                println!("{}", "Connected!".green());
            }
            _ => (),
        }
    }
}
