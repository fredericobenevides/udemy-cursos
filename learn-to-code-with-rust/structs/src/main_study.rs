#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self {
            title,
            release_year,
            duration_secs,
        }
    }
}

impl TaylorSwiftSong {
    // Immutable struct value (self parameter takes ownership)
    fn display_song_info(self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }

    // Immutable reference to the struct instance (no ownership moved)
    fn display_song_info2(&self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }

    fn display_song_info3(&self) {
        println!("Title: {}", self.title);
        println!("Years Since release: {}", self.years_since_release());
        println!("Duration: {} seconds", self.duration_secs);
    }

    // Mutable struct value (self parameter takes ownership, has permission to mutate)
    fn double_length(mut self) {
        self.duration_secs *= 2;
        println!("{:?}", self);
    }

    // Mutable reference to the struct instance (no ownership moved, have permission to muted)
    fn double_length2(&mut self) {
        self.duration_secs *= 2;
        println!("{:?}", self);
    }

    fn is_longer_than(&self, other: &TaylorSwiftSong) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn years_since_release(&self) -> u32 {
        2024 - self.release_year
    }
}

#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}

// Hours, minutes
struct ShortDuration(u32, u32);

// Years, months
struct LongDuration(u32, u32);

struct Empty;

fn study() {
    let blank_space = TaylorSwiftSong::new(String::from("Blank Space"), 2014, 231);
    let all_too_well = TaylorSwiftSong::new(String::from("All Too Well"), 2012, 327);

    // song.display_song_info();
    // println!("{}", song.title);

    // song.double_length();

    // song.display_song_info2();
    // song.double_length2();
    // song.display_song_info2();

    if blank_space.is_longer_than(&all_too_well) {
        println!(
            "{} is longer than {}",
            blank_space.title, all_too_well.title
        );
    } else {
        println!(
            "{} is shorter than or equal to {}",
            blank_space.title, all_too_well.title
        );
    }

    blank_space.display_song_info3();
    all_too_well.display_song_info3();

    let mut computer = Computer::new(String::from("M3 Max"), 64, 2);
    println!("{:?}", computer);
    computer
        .upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(128)
        .upgrade_hard_drive_capacity(4);
    println!("{:#?}", computer);

    let work_shift = ShortDuration(8, 0);
    println!("{} hours {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} years {} months", era.0, era.1);

    go_to_work(work_shift);
    // go_to_work(era);

    let empty = ();
    let my_empty_struct = Empty;
}

fn go_to_work(length: ShortDuration) {
    println!("Passing time {} hours {} minutes", length.0, length.1);
}
