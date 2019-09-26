#[macro_use]
extern crate clap;
extern crate colored;
extern crate hex;

use std::io::{self, Write};

use colored::Colorize;
use rumqtt::{MqttClient, Notification, QoS, Receiver};
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::Terminal;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

use crate::cli::parse_options;
use crate::format::{format_message, MessageFormat};

mod cli;
mod format;

fn start_tui(notifications: Receiver<Notification>) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|mut f| {
        let size = f.size();
        Block::default()
            .title("MQTT Analyzer")
            .borders(Borders::ALL)
            .render(&mut f, size);
    })
}

fn start_stream(notifications: Receiver<Notification>, format_options: MessageFormat) -> Result<(), io::Error> {
    for notification in notifications {
        match notification {
            Notification::Publish(msg) => {
                let line = format_message(format_options, &msg) + "\n";
                io::stdout().write_all(line.as_bytes()).unwrap();
                io::stdout().flush().unwrap();
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
    Ok(())
}

fn main() -> Result<(), io::Error> {
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
        start_tui(notifications)
    } else {
        start_stream(notifications, format_options)
    }
}
