use dialoguer::{theme::ColorfulTheme, Input};
/// ICMP - https://datatracker.ietf.org/doc/html/rfc792
use ping::ping;
use std::fs;
use std::io::Write;
use std::net::IpAddr;
use std::time::{Duration, Instant};

fn main() {
    // output file
    let mut output_file = fs::File::create("output.json").unwrap();

    // set destination address
    let address: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("IP Address")
        .with_initial_text("8.8.8.8".to_string())
        .interact_text()
        .unwrap();

    // convert to IpAddr for 'ping'
    let address = address.parse::<IpAddr>().expect("Invalid IP address");

    // how many times to ping?
    let repetitions: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("How many?")
        .with_initial_text("5".to_string())
        .interact_text()
        .unwrap();

    // Configure ping parameters
    let timeout = Duration::from_secs(1);
    let ttl = 64;
    let ident = 0x1234;
    let seq_cnt = 1;
    let payload: [u8; 24] = [1; 24]; // Example payload, 24 bytes of 1s

    let mut count = 1;
    output_file.write_all(b"starts\n").expect("txt?");

    loop {
        let start = Instant::now();

        match ping(
            address,
            Some(timeout),
            Some(ttl),
            Some(ident),
            Some(seq_cnt),
            Some(&payload),
        ) {
            Ok(_) => {
                let green_tick = '\u{2705}';
                let elapsed = start.elapsed();
                println!("Ping to {} successful {}", address, green_tick);

                // Format elapsed time into a string
                let elapsed_str = format!("{:?}", elapsed);
                let success = "successful ping";

                // Write elapsed time to file
                writeln!(
                    output_file,
                    "{} Elapsed time: {} {}",
                    success, elapsed_str, green_tick
                )
                .expect("Failed to write to file");
                println!("{:?}", elapsed);
            }
            Err(e) => {
                println!("Failed to ping {}: {}", address, e);
            }
        }

        count += 1;
        let reps: u8 = repetitions.parse().expect("Failed to parse u8");
        if count > reps {
            break;
        }
    }
}

