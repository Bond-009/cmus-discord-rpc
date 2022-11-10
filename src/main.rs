use std::cmp;
use std::env;
use std::fmt::{self, Debug, Display, Formatter};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use discord_presence::Client;
use discord_presence::DiscordError;
use discord_presence::models::Activity;

use env_logger;
use log::{debug, info};
use regex::Regex;

const RATE_LIMIT: Duration = Duration::from_secs(15);

#[derive(PartialEq, Debug)]
enum Status {
    Playing,
    Paused,
    Stopped
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Debug)]
struct ParseStatusError;

impl FromStr for Status {
    type Err = ParseStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "playing" => Ok(Status::Playing),
            "paused" => Ok(Status::Paused),
            "stopped" => Ok(Status::Stopped),
            _ => Err(ParseStatusError)
        }
    }
}

fn main() {
    env_logger::init();

    info!("Starting cmus-discord-rpc...");

    let mut drpc = Client::new(431179120836214795);
    _ = drpc.start();

    let socket_path = get_socket_path();
    debug!("Using cmus socket {}", socket_path);
    let mut stream = get_unix_stream(&socket_path);

    let mut output = String::new();
    let mut fails = 0;

    loop {
        if stream.write_all(b"status\n").is_err() {
            drpc.clear_activity().expect("Failed to clear presence");
            stream = get_unix_stream(&socket_path);
            continue;
        }

        let mut reader = BufReader::new(&stream);
        output.clear();

        // Read until an empty line
        while reader.read_line(&mut output).unwrap() != 1 {};
        debug!("Received\n{}", output);

        let status = get_value(&output, "status").unwrap().parse::<Status>().unwrap();
        let mut ac = Activity::new()
                        .details(status.to_string());
        if status != Status::Stopped {
            let artist = get_value(&output, "tag artist");
            let title = get_value(&output, "tag title");

            if artist.is_none() || title.is_none() {
                // Capture filename
                let file_r = Regex::new(r"(?m)^file .+/(.+)\..+\n").unwrap();
                match file_r.captures(&output) {
                    Some(v) => ac = ac.state(v.get(1).unwrap().as_str()),
                    None => ac = ac.state("")
                }
            }
            else {
                ac = ac.state(artist.unwrap().to_owned() + " - " + title.unwrap());
            }

            if status == Status::Playing {
                let duration = get_value(&output, "duration").unwrap().parse::<u64>().unwrap();
                let position = get_value(&output, "position").unwrap().parse::<u64>().unwrap();
                let sce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                ac = ac.timestamps(|t| t.end(sce + duration - position));
            }
        }

        match drpc.set_activity(|_| ac) {
            Ok(_) => {
                info!("Successfully updated activity");
                thread::sleep(RATE_LIMIT);
                fails = 0;
            }
            Err(DiscordError::NotStarted) => {
                let timeout = cmp::min(Duration::from_millis(500 << fails), RATE_LIMIT);
                info!("Discord client isn't ready, waiting {}ms", timeout.as_millis());
                thread::sleep(timeout);
                fails += 1;
                continue;
            },
            v => {
                v.expect("Failed to set activity");
            }
        }
    }
}

fn get_unix_stream(socket_path: &str) -> UnixStream {
    loop {
        if let Ok(s) = UnixStream::connect(socket_path) {
            return s;
        }

        // Try again in 15 seconds
        thread::sleep(Duration::from_secs(15));
    }
}

/// Get the path to the cmus socket the same way as cmus itself
fn get_socket_path() -> String
{
    if let Ok(v) = env::var("CMUS_SOCKET") {
        return v;
    }

    if let Ok(v) = env::var("XDG_RUNTIME_DIR") {
        return v + "/cmus-socket";
    }

    let cmus_config_dir = match env::var("XDG_CONFIG_HOME") {
        Ok(v) => v,
        Err(_) => env::var("HOME").unwrap() + "/.config"
    } + "/cmus";

    cmus_config_dir + "/socket"
}

fn get_value<'t>(input: &'t str, key: &str) -> Option<&'t str> {
    let re = Regex::new(&format!("(?m)^{} (.+)$", key)).unwrap();

    Some(re.captures(input)?.get(1)?.as_str())
}
