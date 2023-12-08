use std::io::{stdin, stdout};

use chords::Chord;

mod app;
mod chords;
    
fn main() {
    run()
}

fn run(){
    let mut app = match app::App::new() {
        Ok(app) => app,
        Err(err) => {
            println!("Error initializing app: {}", err);
            return;
        }
    };

    // app.play_note(66, 4);
    // app.play_note(68, 4);
    // app.play_note(69, 4);
    // app.play_note(71, 4);

    // let new_chord = vec![66,68,69,71];

    // // let test_chord: chords::Chord::major("C#");
    // app.play_chord(&new_chord, 8);

    // new_api_chord.set_extensions(&mut vec![chords::IntervalEnum::MajorSeventh]);

    // app.play_chord(&new_api_chord.to_note_vec(), 8);
    // let test_chord:chords::Chord = chords::Chord::new();

    let mut chord_one: chords::Chord = chords::Chord::major("F3".to_string());
    chord_one.set_extensions(&mut vec![chords::IntervalEnum::MajorSeventh, chords::IntervalEnum::MajorNinth, chords::IntervalEnum::SharpEleven]);

    let mut chord_two:chords::Chord = chords::Chord::minor("D3".to_string());
    chord_two.set_extensions(&mut vec![chords::IntervalEnum::MinorSeventh, chords::IntervalEnum::PerfectEleventh]);

    let mut chord_three:chords::Chord = chords::Chord::minor("A3".to_string());
    chord_three.set_extensions(&mut vec![chords::IntervalEnum::MinorSeventh, chords::IntervalEnum::PerfectEleventh]);

    let mut chord_four:chords::Chord = chords::Chord::minor("E3".to_string());
    chord_four.set_extensions(&mut vec![chords::IntervalEnum::MinorSeventh, chords::IntervalEnum::PerfectEleventh]);

    let chord_progressions: Vec<chords::Chord> = vec![chord_one, chord_two, chord_three, chord_four];

    let mut test_chords: chords::ChordProgression = chords::ChordProgression::new(chord_progressions, 32);

    while true {
        app.play_progression(&mut test_chords, app::VoicingSystem::DropTwo);
    }

}