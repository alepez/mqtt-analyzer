use clap::{App, Arg};
use rumqtt::{MqttOptions, SecurityOptions};

use crate::format::{MessageFormat, PayloadFormat};
use uuid::Uuid;

fn generate_random_client_id() -> String {
    Uuid::new_v4().to_string()
}

pub struct Options {
    pub mqtt: MqttOptions,
    pub topics: Vec<String>,
    pub format: MessageFormat,
}

pub fn parse_options() -> Options {
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
        .arg(Arg::with_name("format")
            .long("format")
            .value_name("FORMAT")
            .help("The format to use to show payload. If text is non valid utf8, it falls back to hex.")
            .takes_value(true)
            .possible_values(&["hex", "base64", "text", "escape"])
        )
        .get_matches();

    let hostname = matches.value_of("hostname").unwrap();
    let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();

    let username = matches.value_of("username");
    let password = matches.value_of("password");

    let client_id = matches
        .value_of("client_id")
        .map(str::to_string)
        .unwrap_or(generate_random_client_id());

    let topics: Vec<String> = matches
        .values_of("topic")
        .map_or(vec![], |values| values.map(|s| s.to_string()).collect());

    let security_options = if let (Some(u), Some(p)) = (username, password) {
        SecurityOptions::UsernamePassword(u.to_string(), p.to_string())
    } else {
        SecurityOptions::None
    };

    let payload_format = match matches.value_of("format") {
        Some("hex") => PayloadFormat::Hex,
        Some("text") => PayloadFormat::Text,
        Some("base64") => PayloadFormat::Base64,
        Some("escape") => PayloadFormat::Escape,
        _ => PayloadFormat::Hex,
    };

    let mut message_format = MessageFormat::default();
    message_format.payload_format = payload_format;

    Options {
        mqtt: MqttOptions::new(client_id, hostname, port).set_security_opts(security_options),
        topics,
        format: message_format,
    }
}
