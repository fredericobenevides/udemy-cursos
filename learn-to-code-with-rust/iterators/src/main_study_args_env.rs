use std::env;
use std::process;

#[derive(Debug)]
struct Settings {
    video_file: String,
    subtitles: bool,
    high_definition: bool,
}

fn main() {
    let settings = collect_settings();
    println!("{:?}", settings);
}

fn collect_settings() -> Settings {
    // target/debug/iterators   rust.mp4 true false nonsense nonsense nonsense
    //                       -> rust.mp4 true false nonsense
    //                       -> rust.mp4 true
    //                       -> rust.mp4
    let mut args = env::args().skip(1).take(3);

    let video_file = args.next().unwrap_or_else(|| {
        eprintln!("No video file specified!");
        process::exit(1);
    });

    let mut settings = args.map(|setting| setting.parse::<bool>().unwrap_or(false));

    let subtitles = settings.next().unwrap_or(false);
    let high_definition = settings.next().unwrap_or(false);

    Settings {
        video_file,
        subtitles,
        high_definition,
    }
}
