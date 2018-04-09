extern crate discord_rpc_client;
extern crate regex;

use discord_rpc_client::Client;
use discord_rpc_client::models::Activity;
use regex::Regex;

use std::{env, thread, time};
use std::io::{BufReader,Error, ErrorKind, prelude::*};
use std::os::unix::net::UnixStream;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Debug)]
enum Status {
    Playing,
    Paused,
    Stopped
}

fn main() {

    let mut stream = get_unixstream();
    let mut drpc = Client::new(431179120836214795).expect("Failed to start client");
    drpc.start();

    loop {
        match stream.write_all(b"status\n") {
            Ok(_) => (),
            Err(_) => {
                drpc.clear_activity().expect("Failed clear presence");
                stream = get_unixstream();
                continue;
            }
        }
        let reader = BufReader::new(&mut stream);

        let mut output = String::new();
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if line == "" {
                        break;
                    }
                    output += &line;
                    output += &"\n".to_string();
                }
                Err(x) => panic!(x)
            }
        }

        let status = get_status(&get_value(&output, "status")).unwrap();

        let mut ac: Activity = Activity::new().details(format!("{:?}", status));

        if status != Status::Stopped {
            let artist = get_value(&output, "tag artist");
            let title = get_value(&output, "tag title");
            ac = ac.state(artist + " - " + &title);

            if status == Status::Playing {
                let duration = get_value(&output, "duration").parse::<u64>().unwrap();
                let position = get_value(&output, "position").parse::<u64>().unwrap();
                let sce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                ac = ac.timestamps(|t| t.end(sce + duration - position));
            }
        }

        drpc.set_activity(|_| ac).expect("Failed to set presence");

        thread::sleep(time::Duration::from_secs(15));
    }
}

fn get_unixstream() -> UnixStream {
    let rdir = env::var("XDG_RUNTIME_DIR").unwrap();
    let path = rdir + "/cmus-socket";
    loop {
        match UnixStream::connect(&path) {
            Ok(strm) => return strm,
            Err(_) => {
                // Try again in 15 seconds
                thread::sleep(time::Duration::from_secs(15));
            }
        }
    }
}

fn get_value(input: &str, key: &str) -> String {
    let re = Regex::new(&format!("(?m)^{} (?P<t>.+)$", key)).unwrap();
    match re.captures(input) {
        Some(value) => value["t"].to_string(),
        None => "".to_string()
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
