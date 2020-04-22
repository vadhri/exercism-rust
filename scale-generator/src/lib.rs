#[derive(Debug)]
pub enum Error {
    InvalidTonic,
    InvalidInterval
}

pub struct Scale {
    seq: Vec<String>
}

static SHARPS_MAP: [&str; 14] =
    ["C", "a", "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#"];

static _FLATS_MAP: [&str; 12] =
    ["F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb"];

static SHARPS: [&str; 12] =
    ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
static FLATS: [&str; 12] =
    ["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"];

fn pick_notes(tonic: &str) -> &[&str] {
    if SHARPS_MAP.iter().any(|&a| a == tonic) {
        &SHARPS
    } else {
        &FLATS
    }
}

fn interval_size(c: char) -> usize {
    match c {
        'm' => 1,
        'M' => 2,
        'A' => 3,
        _ => 0
    }
}

fn get(tonic: &str, intervals: &str) -> Vec<String> {
    let notes = pick_notes(tonic);

    let mut pos = notes.iter().enumerate()
        .find(|(_, note)| note.to_lowercase() == tonic.to_lowercase())
        .map(|(i, _)| i).unwrap();

    intervals.chars().map(|c| {
        let s = notes[pos % notes.len()].to_string();
        pos += interval_size(c);
        s
    }).collect::<Vec<String>>()
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        Ok(Scale {
            seq: get(tonic, intervals)
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Self::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.seq.clone()
    }
}
