use std::io::stdin;


mod app;
    
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

    println!("Going to wait...");
    stdin().read_line(&mut String::new()).unwrap();
}

