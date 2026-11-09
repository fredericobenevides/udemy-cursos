fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("Blastoff!");
    } else {
        println!("{seconds} seconds to blastoff..");
        countdown(seconds - 1);
    }
}

fn debug() {
    countdown(5);

    countdown(5);

    countdown(5);
}
