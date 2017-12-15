use std::path::PathBuf;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub struct Header {
    // mandatory data from headers
    pub artist: String,
    pub title: String,
    pub bpm: f32,
    pub audio_path: PathBuf,

    // optional data from headers
    pub gap: Option<f32>,
    pub cover_path: Option<PathBuf>,
    pub background_path: Option<PathBuf>,
    pub video_path: Option<PathBuf>,
    pub video_gap: Option<f32>,
    pub genre: Option<String>,
    pub edition: Option<String>,
    pub language: Option<String>,
    pub year: Option<u32>,
    /* header fields todo
    pub creator: Option<String>,  // CREATOR
    pub start: Option<f32>,       // START
    pub end: Option<i32>,         // END
    pub resolution: Option<i32>,  // RESOLUTION
    pub notes_gap: Option<i32>,   // NOTESGAP
    pub encoding: Option<String>, // ENCODING
    pub preview_start: Option<i32>, // PREVIEWSTART
    pub medley_start_beat: Option<i32>, // MEDLEYSTARTBEAT
    pub medley_end_beat: Option<i32>,   // MEDLEYENDBEAT
    pub calc_medley: Option<Bool>,      // CALCMEDLEY
    pub duet_singer_p1: Option<String>, // DUETSINGERP1 / P1
    pub duet_singer_p2: Option<String>, // DUETSINGERP2 / P2:
    */
    pub relative: Option<bool>,
    pub unknown: Option<HashMap<String, String>>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct TXTSong {
    pub header: Header,
    pub lines: Vec<Line>,
}


#[derive(PartialEq, Clone, Debug)]
pub enum Note {
    Regular {
        start: i32,
        duration: i32,
        pitch: i32,
        text: String,
    },
    Golden {
        start: i32,
        duration: i32,
        pitch: i32,
        text: String,
    },
    Freestyle {
        start: i32,
        duration: i32,
        pitch: i32, //pitch might not be needed but not including it might lose data from orig file
        text: String,
    },
    PlayerChange { player: i32 },
}

#[derive(PartialEq, Clone, Debug)]
pub struct Line {
    pub start: i32,
    pub rel: Option<i32>,
    pub notes: Vec<Note>,
}
