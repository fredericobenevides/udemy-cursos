#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let message = ChatMessage {
        content: "message",
        time: String::from("time 1"),
    };
    println!("{}", message.retrieve_time());

    let message = ChatMessage {
        content: String::from("message"),
        time: String::from("time 2"),
    };
    println!("{}", message.retrieve_time());

    let message: ChatMessage<DigitalContent> = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("time 3"),
    };
    message.consume_entertainment();
    println!("{}", message.retrieve_time());
}
