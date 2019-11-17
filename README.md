# MQTT Analyzer

This is a command line tool that helps you analyze MQTT messages.

## Features

Now it is a simpler version of *mosquitto_pub*, with similar parameters.

```
USAGE:
    mqtt-analyzer [FLAGS] [OPTIONS]

FLAGS:
        --help       Prints help information
        --tui        Enable Text User Interface
    -V, --version    Prints version information

OPTIONS:
    -i, --id <ID>                The id to use for this client
        --format <FORMAT>        The format to use to show payload. If text is non valid utf8, it falls back to hex.
                                 [default: auto]  [possible values: hex, base64, text, escape, auto]
    -h, --host <HOSTNAME>        Specify the host to connect to [default: localhost]
        --mode <mode>            Enable Text User Interface [default: subs]  [possible values: subs, stream, retained,
                                 stats]
    -P, --pw <PASSWORD>          Provide a password to be used for authenticating with the broker. See also the
                                 --username option
    -p, --port <PORT>            Connect to the port specified [default: 1883]
    -t, --topic <TOPIC>...       The MQTT topic to subscribe to
    -u, --username <USERNAME>    Provide a username to be used for authenticating with the broker. See also the --pw
                                 argument
```

## Future developement

 - [ ] statistics (occurrences, frequency, etc...)
 - [ ] regular expression filter on payload
 - [ ] extensible custom formatting
 - [ ] tree navigation on topics