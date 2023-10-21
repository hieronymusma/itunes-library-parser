use std::collections::HashMap;

extern crate plist;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
struct MusicLibrary {
    #[serde(rename = "Tracks")]
    tracks: HashMap<String, Track>,
    #[serde(rename = "Playlists")]
    playlists: Vec<Playlist>
}

impl MusicLibrary {
    fn get_track(&self, track_id: u64) -> Option<&Track> {
        self.tracks.get(&track_id.to_string())
    }
}

#[derive(Debug, Deserialize)]
struct Track {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Artist")]
    artist: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Playlist {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Playlist Items")]
    items: Option<Vec<PlaylistItem>>
}

#[derive(Debug, Deserialize)]
struct PlaylistItem {
    #[serde(rename = "Track ID")]
    track_id: u64
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    if std::env::args().len() < 2 {
        println!("Please provide the file path to the iTunes Library XML file as the first argument.");
        return Ok(());
    }

    let library_file_path = std::env::args().nth(1).unwrap();

    let library: MusicLibrary = plist::from_file(library_file_path)?;

    for playlist in &library.playlists {
        if let Some(items) = &playlist.items {
            for item in items {
                if let Some(track) = library.get_track(item.track_id) {
                    println!("Playlist: {} Title: {} (Artist: {})", playlist.name, track.name, track.artist.as_ref().unwrap_or(&"".to_string()));
                }
            }
        }
    }

    Ok(())
}