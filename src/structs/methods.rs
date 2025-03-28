#[derive(Debug)]
struct Song {
    title: String,
    artist: String,
    release_year: u32,
    duration_seconds: u32,
}

// methods are defined in this block
impl Song {
    fn display_song(self) {
        // immutable struct value (self parameter takes ownership) (self: Song or self: Self or
        // just self)
        //  Self is a kind of alias of Song, this is more resilient to change, tomorrow I can change
        //  the struct name to Album
        // mutable struct value (self parameter takes ownership, has permission to mutate)
        // immutable reference to the struct (no ownership moved)
        // mutable reference to the struct instance (no ownership moved, have permission to mutate)
        println!("Title: {}", self.title);
        println!("Artist: {}", self.artist);
        println!("Release Year: {}", self.release_year);
        println!("Duration in Secods: {}", self.duration_seconds);
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

        song.display_song();
    }
}
