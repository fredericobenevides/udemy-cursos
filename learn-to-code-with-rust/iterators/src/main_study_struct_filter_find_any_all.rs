#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrammingTutorials,
}

#[derive(Debug)]
struct TvChannel {
    name: String,
    channel_type: ChannelType,
}

fn main() {
    let channels = [
        TvChannel {
            name: String::from("CBS"),
            channel_type: ChannelType::Comedy,
        },
        TvChannel {
            name: String::from("RustLive"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
        TvChannel {
            name: String::from("NBC"),
            channel_type: ChannelType::News,
        },
        TvChannel {
            name: String::from("RustTV"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
    ];

    let good_channels: Vec<&TvChannel> = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
        .collect();
    println!("{:?}", good_channels);

    let good_channels: Vec<String> = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
        .map(|channel| channel.name.clone())
        .collect();
    println!("{:?}", good_channels);

    let good_channel = channels
        .iter()
        .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);

    match good_channel {
        Some(channel) => println!("Great choice to watch {:?}", channel),
        None => println!("There was no Rust programming on the TV (literally and metaphorically)"),
    }

    let all_are_rust = channels
        .iter()
        .all(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("{}", all_are_rust);

    let any_are_rust = channels
        .iter()
        .any(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("{}", any_are_rust);
}
