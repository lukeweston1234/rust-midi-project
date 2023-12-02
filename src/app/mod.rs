use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use midir::{ MidiOutputConnection, MidiInputConnection};

mod midi;

const NOTE_ON_MSG: u8 = 0x90;
const NOTE_OFF_MSG: u8 = 0x80;
const VELOCITY: u8 = 0x64;

pub struct App {
    midi_in: MidiInputConnection<()>,
    midi_out: MidiOutputConnection
}
impl App {
    pub fn new() -> Result<App, Box<dyn Error>>{
        let midi_in: MidiInputConnection<()> = midi::build_midi_in()?;
        let midi_out: MidiOutputConnection = midi::build_midi_out()?;
        let app: App = App {
            midi_in: midi_in,
            midi_out: midi_out
        };
        Ok(app)
    }
    pub fn play_note(&mut self, note: u8, duration: u64){
        let _ = (*self).midi_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
        sleep(Duration::from_millis(duration * 150));
        let _ = (*self).midi_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
    }
    pub fn play_chord(&mut self, notes: &Vec<u8>, duration: u64){
        for note in notes {
            let _ = (*self).midi_out.send(&[NOTE_ON_MSG, *note, VELOCITY]);
        }
        sleep(Duration::from_millis(duration * 150));
        for note in notes {
            let _ = (*self).midi_out.send(&[NOTE_OFF_MSG, *note, VELOCITY]);
        }
    }
}