use std::ops::Deref;

struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>,
}

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.audio
    }
}

//impl Deref for Mp3 {
//    type Target = Vec<u8>;
//
//    fn deref(&self) -> &Vec<u8> {
//        &self.audio
//    }
//}

fn main() {
    let my_favorite_song = Mp3 {
        // we would read the actual audio data from an mp3 file
        audio: vec![1, 2, 3],
        artist: Some(String::from("Nirvana")),
        title: Some(String::from("Smells Like Teen Spirit")),
    };

    assert_eq!(vec![1, 2, 3], *my_favorite_song);
	assert_eq!("Nirvana".to_string(), my_favorite_song.artist.unwrap());
	assert_eq!("Smells Like Teen Spirit".to_string(), my_favorite_song.title.unwrap());
}