use std::io::{stdin, stdout};

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

    app.play_note(66, 4);
    app.play_note(68, 4);
    app.play_note(69, 4);
    app.play_note(71, 4);

    let new_chord = vec![66,68,69,71];

    // let test_chord: chords::Chord::major("C#");
    app.play_chord(&new_chord, 8);

    let mut new_api_chord: chords::Chord = chords::Chord::major("A3".to_string());

    new_api_chord.set_extensions(&mut vec![chords::IntervalEnum::MajorSeventh]);

    app.play_chord(&new_api_chord.to_note_vec(), 8);
    // let test_chord:chords::Chord = chords::Chord::new();

    stdin().read_line(&mut String::new()).unwrap();
}

