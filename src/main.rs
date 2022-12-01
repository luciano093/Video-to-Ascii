use std::env;
use std::time::{Duration, Instant};

use video_to_ascii::decoding::get_video;
use video_to_ascii::print_frame;

fn main() {
    let mut args = env::args_os();

    if args.len() < 2 {
        eprintln!("Error: you must provide a path to a video.");
        return;
    }

    let start = Instant::now();

    let video = match get_video(args.nth(1).unwrap().to_str().unwrap()) {
        Ok(video) => video,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            return;
        }
    };

    println!("time: {}s", (Instant::now() - start).as_secs_f64());

    for frame in video.frames() {
        print_frame(frame);
        std::thread::sleep(Duration::from_nanos((1_000_000_000 as f64 / video.fps()) as u64));
    }
}