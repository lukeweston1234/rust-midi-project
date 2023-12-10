use std::error::Error;

use midir::{MidiOutput, Ignore, MidiOutputConnection, MidiInput, MidiInputConnection};

pub fn build_midi_in(callback_fn: impl FnMut(u64, &[u8], &mut ()) + Send + 'static) -> Result<MidiInputConnection<()>, Box<dyn Error>> {
    let mut midi_in: MidiInput = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    let in_ports: Vec<midir::MidiInputPort> = midi_in.ports();

    let in_port: &midir::MidiInputPort = &in_ports[0];

    let conn_in: MidiInputConnection<()> = midi_in.connect(
        &in_port,
        "midir-read-input",
        callback_fn,
        (),
    )?;
    Ok(conn_in)
}

pub fn build_midi_out() -> Result<MidiOutputConnection, Box<dyn Error>> {
    let midi_out: MidiOutput = MidiOutput::new("My Test Output")?;
    let out_ports: Vec<midir::MidiOutputPort> = midi_out.ports();
    let out_port: &midir::MidiOutputPort = &out_ports[0];
    let conn_out: MidiOutputConnection = midi_out.connect(out_port, "midir-test")?;
    Ok(conn_out)
}