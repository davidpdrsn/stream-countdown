use std::io;
use std::io::prelude::*;
use std::thread;
use std::time::{Duration, Instant};
use structopt::StructOpt;

/// Stream countdown
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Number of minutes until the stream starts
    min: u64,
    #[structopt(short = "t", long = "twitter")]
    twitter: Option<String>,
    #[structopt(short = "g", long = "github")]
    github: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    ctrlc::set_handler(move || {
        print!("{}", ansi_escapes::CursorTo::AbsoluteXY(0, 0));
        println!("{}", ansi_escapes::CursorShow);
        clear();
        flush();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let start_time = Instant::now() + Duration::from_secs((opt.min + 1) * 60);

    let mut i = 0;

    loop {
        clear();
        setup_cursor();
        i += 1;

        let mins_remaining = (start_time - Instant::now()).as_secs() / 60;

        if mins_remaining == 0 {
            break;
        }

        let min_text = if mins_remaining == 1 { "min" } else { "mins" };
        let emoji = if i % 2 == 0 { "⏳" } else { "⌛️" };

        print!(
            "Stream will begin in ~{} {} {}",
            mins_remaining, min_text, emoji
        );

        print_social(&opt);

        io::stdout().flush().expect("Could not flush stdout");

        thread::sleep(Duration::from_secs(1));
    }

    loop {
        clear();
        setup_cursor();
        println!("Stream will begin any minute 🎉",);
        print_social(&opt);
        flush();

        thread::sleep(Duration::from_secs(60));
    }
}

fn print_social(opt: &Opt) {
    let delta = 4;

    if let Some(twitter) = &opt.twitter {
        print!("{}", ansi_escapes::CursorTo::AbsoluteXY(X + delta, Y));
        print!("🐤 twitter.com/{}", twitter);
    }

    if let Some(github) = &opt.github {
        print!("{}", ansi_escapes::CursorTo::AbsoluteXY(X + delta + 1, Y));
        print!("🤖 github.com/{}", github);
    }
}

fn clear() {
    print!("{}[2J", 27 as char);
}

const X: u16 = 4;
const Y: u16 = 5;

fn setup_cursor() {
    print!("{}", ansi_escapes::CursorHide);
    print!("{}", ansi_escapes::CursorTo::AbsoluteXY(X, Y));
}

fn flush() {
    io::stdout().flush().expect("Could not flush stdout");
}
