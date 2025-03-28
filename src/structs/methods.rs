#[derive(Debug)]
struct Song {
    title: String,
    artist: String,
    release_year: u32,
    duration_seconds: u32,
}

// methods are defined in this block
impl Song {
    // immutable struct value (self parameter takes ownership) (self: Song or self: Self or
    // just self)
    //  Self is a kind of alias of Song, this is more resilient to change, tomorrow I can change
    //  the struct name to Album
    fn display_song(self) {
        println!("Title: {}", self.title);
        println!("Artist: {}", self.artist);
        println!("Release Year: {}", self.release_year);
        println!("Duration in Secods: {}", self.duration_seconds);
    }

    // mutable struct value (self parameter takes ownership, has permission to mutate)
    fn double_length(mut self) {
        self.duration_seconds *= 2;
        println!("{:?}", self);
    }

    // immutable reference to the struct (no ownership moved)
    // we can also use &Self
    // we can also use &self
    fn display_song_reference(self: &Song) {
        println!("Title: {}", self.title);
        println!("Artist: {}", self.artist);
        println!("Release Year: {}", self.release_year);
        println!("Duration in Secods: {}", self.duration_seconds);
    }

    // mutable reference to the struct instance (no ownership moved, have permission to mutate)
    // the instance here is borrowed
    // self here is not a struct, is a reference to a struct
    // also can be set as self: &mut Song
    fn double_length_mutable(&mut self) {
        self.duration_seconds *= 2;
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_seconds > other.duration_seconds
    }
}

pub fn index(show: bool) {
    if show {
        let song = Song {
            title: String::from("Dream On"),
            artist: String::from("Aerosmith"),
            release_year: 1973,
            duration_seconds: 268,
        };

        song.display_song(); // song is moved here

        // with this approach song is not the owner and cannot be used
        // I cannot set println!("{}", song.title);, because self in display_song(self) is the
        // owner of the song struct because the value is moved

        let metallica_song1 = Song {
            title: String::from("If Darkness had a Son"),
            artist: String::from("Metallica"),
            release_year: 2023,
            duration_seconds: 360,
        };

        metallica_song1.double_length();

        let metallica_song2 = Song {
            title: String::from("One"),
            artist: String::from("Metallica"),
            release_year: 1988,
            duration_seconds: 447,
        };

        // automatically behind the scenes rust pass a reference &self
        metallica_song2.display_song_reference();

        // with this implementation I can continue use the struct instance
        println!("{:?}", metallica_song2.title);

        let mut metallica_song3 = Song {
            title: String::from("Master of Puppets"),
            artist: String::from("Metallica"),
            release_year: 1986,
            duration_seconds: 515,
        };

        metallica_song3.double_length_mutable();

        // now I am able to continue using the instance and its fields
        println!("{:?}", metallica_song3.title);

        if metallica_song3.is_longer_than(&metallica_song2) {
            println!(
                "{} is longer than {}",
                metallica_song3.title, metallica_song2.title,
            );
        } else {
            println!(
                "{} is shorter than {}",
                metallica_song2.title, metallica_song3.title,
            )
        }
    }
}
