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

    let mut chord_one: chords::Chord = chords::Chord::major("F3".to_string());
    chord_one.set_extensions(&mut vec![chords::IntervalEnum::MajorSeventh, chords::IntervalEnum::MajorNinth, chords::IntervalEnum::SharpEleven]);

    let mut chord_two:chords::Chord = chords::Chord::minor("D3".to_string());
    chord_two.set_extensions(&mut vec![chords::IntervalEnum::MinorSeventh, chords::IntervalEnum::PerfectEleventh]);

    let mut chord_three:chords::Chord = chords::Chord::minor("A3".to_string());
    chord_three.set_extensions(&mut vec![chords::IntervalEnum::MinorSeventh, chords::IntervalEnum::PerfectEleventh]);

    let mut chord_four:chords::Chord = chords::Chord::minor("E3".to_string());
    chord_four.set_extensions(&mut vec![chords::IntervalEnum::MinorSeventh, chords::IntervalEnum::PerfectEleventh]);

    let chord_vec_one: Vec<chords::Chord> = vec![chord_one, chord_two, chord_three, chord_four];

    let test_chords: chords::ChordProgression = chords::ChordProgression::new(chord_vec_one, 32);

    app.play_scene(chords::Scene::new(vec![test_chords]), chords::NoteEnum::WholeNote, &app::VoicingSystem::DropTwo);

}