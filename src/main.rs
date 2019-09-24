#[macro_use]
extern crate clap;
extern crate colored;
extern crate hex;

use std::io::{self, Write};

use clap::{App, Arg};
use colored::*;
use rumqtt::{MqttClient, MqttOptions, Notification, QoS, SecurityOptions};

struct Options {
    mqtt: MqttOptions,
    topics: Vec<String>,
}

fn parse_options() -> Options {
    let matches = App::new("mqtt-analyzer")
        .version(crate_version!())
        .author("Alessandro Pezzato <alessandro@pezzato.net>")
        .about("Analyze mqtt messages")
        .arg(Arg::with_name("hostname")
            .short("h")
            .long("host")
            .value_name("HOSTNAME")
            .help("Specify the host to connect to")
            .takes_value(true)
            .default_value("localhost")
        )
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Connect to the port specified")
            .takes_value(true)
            .default_value("1883")
        )
        .arg(Arg::with_name("username")
            .short("u")
            .long("username")
            .value_name("USERNAME")
            .help("Provide a username to be used for authenticating with the broker. See also the --pw argument")
            .takes_value(true)
        )
        .arg(Arg::with_name("password")
            .short("P")
            .long("pw")
            .value_name("PASSWORD")
            .help("Provide a password to be used for authenticating with the broker. See also the --username option")
            .takes_value(true)
        )
        .arg(Arg::with_name("client_id")
            .short("i")
            .long("id")
            .value_name("ID")
            .help("The id to use for this client")
            .takes_value(true)
        )
        .arg(Arg::with_name("topic")
            .short("t")
            .long("topic")
            .value_name("TOPIC")
            .help("The MQTT topic to subscribe to")
            .takes_value(true)
            .multiple(true)
        )
        .get_matches();

    let hostname = matches.value_of("hostname").unwrap();
    let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();

    let username = matches.value_of("username");
    let password = matches.value_of("password");

    let client_id = matches.value_of("client_id").unwrap_or("FIXME");

    let topics: Vec<String> = matches.values_of("topic").map_or(vec![], |values| values.map(|s| s.to_string()).collect());

    let security_options = if let (Some(u), Some(p)) = (username, password) {
        SecurityOptions::UsernamePassword(u.to_string(), p.to_string())
    } else {
        SecurityOptions::None
    };

    Options {
        mqtt: MqttOptions::new(client_id, hostname, port).set_security_opts(security_options),
        topics,
    }
}

fn format_payload(payload: &[u8]) -> String {
    hex::encode(payload)
}

fn format_message(msg: &rumqtt::Publish) -> String {
    let payload = format_payload(msg.payload.as_ref());
    format!("{} {}\n", msg.topic_name.blue(), payload)
}

fn main() {
    let options = parse_options();

    let Options { mqtt: mqtt_options, topics } = options;
    let (mut client, notifications) = MqttClient::start(mqtt_options).unwrap();

    for topic in topics.iter() {
        client.subscribe(topic.as_str(), QoS::AtLeastOnce).unwrap();
    }

    for notification in notifications {
        match notification {
            Notification::Publish(msg) => {
                io::stdout().write_all(format_message(&msg).as_bytes()).unwrap();
            }
            Notification::Disconnection => {
                println!("{}", "Disconnected!".red());
            }
            Notification::Reconnection => {
                println!("{}", "Connected!".green());
            }
            _ => ()
        }
    }
}