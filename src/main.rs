use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiOutput, MidiOutputPort, MidiOutputConnection};

fn main() {
    match run() {
        Ok(()) => (),
        Err(err) => println!("Error getting MIDI output: {}", err)
    }
}

fn run() -> Result<(),Box<dyn Error>>{
    let mut output:MidiOutputConnection = get_midi_out()?;
    play_note(&mut output, 66, 4);
    Ok(())
}

fn play_note(output: &mut MidiOutputConnection, note: u8, duration: u64){
    const NOTE_ON_MSG: u8 = 0x90;
    const NOTE_OFF_MSG: u8 = 0x80;
    const VELOCITY: u8 = 0x64;

    let _ = output.send(&[NOTE_ON_MSG, note, VELOCITY]);
    sleep(Duration::from_millis(duration * 150));
    let _ = output.send(&[NOTE_OFF_MSG, note, VELOCITY]);
}

fn get_midi_out() -> Result<MidiOutputConnection,Box<dyn Error>>  {
    let midi_out = MidiOutput::new("My Test Output")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid output port selected")?
        }
    };

    println!("\nOpening connection");
    let conn_out = midi_out.connect(out_port, "midir-test")?;
    Ok(conn_out)
}

