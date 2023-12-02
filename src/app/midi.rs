use std::error::Error;

use midir::{MidiOutput, Ignore, MidiOutputConnection, MidiInput, MidiInputConnection};

pub fn build_midi_in() -> Result<MidiInputConnection<()>, Box<dyn Error>> {
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    let in_port = &in_ports[0];

    let conn_in = midi_in.connect(
        &in_port,
        "midir-read-input",
        move |stamp, message, _| {
            println!("{}: {:x?} (len = {})", stamp, message, message.len());
        },
        (),
    )?;
    Ok(conn_in)
}

pub fn build_midi_out() -> Result<MidiOutputConnection, Box<dyn Error>> {
    let midi_out = MidiOutput::new("My Test Output")?;
    let out_ports = midi_out.ports();
    let out_port = &out_ports[0];
    let conn_out = midi_out.connect(out_port, "midir-test")?;
    Ok(conn_out)
}