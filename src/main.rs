extern crate discord_rpc_client;
extern crate regex;

use discord_rpc_client::Client;
use discord_rpc_client::models::Activity;
use regex::Regex;

use std::{env, thread};
use std::io::{BufRead, BufReader,Error, ErrorKind, Write};
use std::os::unix::net::UnixStream;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Debug)]
enum Status {
    Playing,
    Paused,
    Stopped
}

fn main() {
    let socket_path = get_socket_path();
    let mut stream = get_unix_stream(&socket_path);
    let mut drpc = Client::new(431179120836214795);
    drpc.start();

    loop {
        match stream.write_all(b"status\n") {
            Ok(_) => (),
            Err(_) => {
                drpc.clear_activity().expect("Failed clear presence");
                stream = get_unix_stream(&socket_path);
                continue;
            }
        }
        let mut reader = BufReader::new(&stream);
        let mut output = String::new();
        // Read until an empty line
        while reader.read_line(&mut output).unwrap() != 1 {};

        let status = get_status(&get_value(&output, "status")).unwrap();

        let mut ac = Activity::new()
                        .details(format!("{:?}", status));
        if status != Status::Stopped {
            let artist = get_value(&output, "tag artist");
            let title = get_value(&output, "tag title");
            
            if artist.is_empty() || title.is_empty() {
                // Capture filename
                let file_r = Regex::new(r"(?m)^file .+/(?P<f>.+)\..+\n").unwrap();
                let file = match file_r.captures(&output) {
                    Some(v) => v["f"].to_owned(),
                    None => "".to_owned()
                };
                ac = ac.state(file);
            }
            else {
                ac = ac.state(artist + " - " + &title);
            }

            if status == Status::Playing {
                let duration = get_value(&output, "duration").parse::<u64>().unwrap();
                let position = get_value(&output, "position").parse::<u64>().unwrap();
                let sce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                ac = ac.timestamps(|t| t.end(sce + duration - position));
            }
        }

        drpc.set_activity(|_| ac).expect("Failed to set presence");

        thread::sleep(Duration::from_secs(15));
    }
}

fn get_unix_stream(socket_path: &str) -> UnixStream {
    loop {
        match UnixStream::connect(socket_path) {
            Ok(s) => return s,
            Err(_) => {
                // Try again in 15 seconds
                thread::sleep(Duration::from_secs(15));
            }
        }
    }
}

/// Get the path to the cmus socket the same way as cmus itself
fn get_socket_path() -> String
{
    match env::var("CMUS_SOCKET") {
        Ok(v) => return v,
        Err(_) => ()
    };

    match env::var("XDG_RUNTIME_DIR") {
        Ok(v) => return v + "/cmus-socket",
        Err(_) => ()
    };

    let cmus_config_dir = match env::var("XDG_CONFIG_HOME") {
        Ok(v) => v,
        Err(_) => env::var("HOME").unwrap() + "/.config"
    } + "/cmus";
 
    return cmus_config_dir + "/socket"
}

fn get_value(input: &str, key: &str) -> String {
    let re = Regex::new(&format!("(?m)^{} (?P<v>.+)$", key)).unwrap();
    match re.captures(input) {
        Some(value) => value["v"].to_owned(),
        None => "".to_owned()
    }
}

fn get_status(input: &str) -> Result<Status, Error> {
    match input {
        "playing" => Ok(Status::Playing),
        "paused" => Ok(Status::Paused),
        "stopped" => Ok(Status::Stopped),
        _ => Err(Error::new(ErrorKind::Other, "oh no!"))
    }
}
