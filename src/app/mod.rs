use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use midir::{ MidiOutputConnection, MidiInputConnection};

use crate::chords::{ChordProgression, Scene, NoteEnum};

use rand::{thread_rng, Rng};

mod midi;

const NOTE_ON_MSG: u8 = 0x90;
const NOTE_OFF_MSG: u8 = 0x80;
const VELOCITY: u8 = 0x64;

pub enum VoicingSystem {
    Closed,
    DropTwo,
    DropThree,
}

fn test_function(timestamp: u64, message: &[u8], _options: &mut ()){
    println!("{}:{:?}", timestamp, message)
}

pub struct App {
    #[allow(dead_code)]
    midi_in: MidiInputConnection<()>,
    midi_out: MidiOutputConnection,
    mod_val: u8,
    velocity_val: u8,
    current_clock: u64,
    bpm: u64
}
impl App {
    pub fn new() -> Result<App, Box<dyn Error>>{
        let midi_in: MidiInputConnection<()> = midi::build_midi_in(test_function)?;
        let midi_out: MidiOutputConnection = midi::build_midi_out()?;
        let app: App = App {
            midi_in: midi_in,
            midi_out: midi_out,
            mod_val: 100,
            velocity_val: 100,
            current_clock: 0,
            bpm: 60,
        };
        Ok(app)
    }
    pub fn play_note(&mut self, note: u8, duration: u64){
        let _ = (*self).midi_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
        sleep(Duration::from_millis(duration * 150));
        let _ = (*self).midi_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
    }
    pub fn play_chord(&mut self, notes: &Vec<u8>, duration: f32, voicing_system: &VoicingSystem){
        let mut new_notes: Vec<u8> = notes.clone();
        let mut rng = thread_rng();
        let random_index = rng.gen_range(0..3);
        new_notes[random_index] = new_notes[random_index] - 12;
        // match voicing_system {
        //     VoicingSystem::Closed => {
        //         new_notes.
        //     },
        //     VoicingSystem::DropTwo => (),
        //     VoicingSystem::DropThree => ()
        // }
        for note in &new_notes {
            let _ = (*self).midi_out.send(&[NOTE_ON_MSG, *note, VELOCITY]);
        }
        sleep(Duration::from_millis(duration.round() as u64));
        for note in &new_notes {
            let _ = (*self).midi_out.send(&[NOTE_OFF_MSG, *note, VELOCITY]);
        }
    }
    pub fn play_progression(&mut self, chords: ChordProgression, duration: f32, voicing_system: &VoicingSystem){
        for chord in chords.chords {
            println!("In play_progression");
            (*self).play_chord(&chord.to_note_vec(), duration, &voicing_system);
        }
        return;
    }
    pub fn play_scene(&mut self, scene: Scene, note_enum: NoteEnum, voicing_system: &VoicingSystem) {
        for progression in scene.progression{
            self.play_progression( progression, 60000 as f32 / self.bpm as f32 * note_enum.value(), voicing_system);
        }
    }
}