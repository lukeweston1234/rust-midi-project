use std::collections::HashMap;

// We will use this to figure out the note
pub fn initialize_midi_map() -> (HashMap<u8, Vec<String>>, HashMap<String, u8>) {
    let note_names_with_enharmonics = [
        vec!["A"],
        vec!["A#", "Bb"],
        vec!["B", "Cb"],
        vec!["C"],
        vec!["C#", "Db"],
        vec!["D"],
        vec!["D#", "Eb"],
        vec!["E", "Fb"],
        vec!["F", "E#"],
        vec!["F#", "Gb"],
        vec!["G"],
        vec!["G#", "Ab"],
    ];
    let mut midi_to_note_map: HashMap<u8, Vec<String>> = HashMap::new();
    let mut note_to_midi_map: HashMap<String, u8> = HashMap::new();

    for midi_number in 21..=108 { // Covering the range from A0 (21) to C8 (108)
        let note_index: i32 = (midi_number as i32 - 21) % 12; // Calculate note index in the octave
        let octave: i32 = ((midi_number as i32 - 12) / 12); // Calculate the octave number

        let note_names: Vec<String> = note_names_with_enharmonics[note_index as usize]
            .iter()
            .map(|&name| format!("{}{}", name, octave))
            .collect();
        
        midi_to_note_map.insert(midi_number, note_names.clone());
        for note in note_names.clone(){
            note_to_midi_map.insert(note, midi_number);
        }

    }
    (midi_to_note_map, note_to_midi_map)
}

pub type Interval = u8;

pub struct Chord {
    root_note: String,
    intervals: Vec<Interval>,
    note_to_midi_map: HashMap<u8, Vec<String>>,
    midi_to_note_map: HashMap<String, u8>
}
impl Chord {
    pub fn new(root_note: String, intervals: Vec<Interval>) -> Chord {
        let midi_touple: (HashMap<u8, Vec<String>>, HashMap<String, u8>) = initialize_midi_map();
        Chord {
            root_note: root_note,
            intervals: intervals,
            note_to_midi_map: midi_touple.0,
            midi_to_note_map: midi_touple.1
        }
    }
    pub fn major(root_note: String) -> Chord {
        return Chord::new(root_note,  vec![0, 4, 7]);
    }
    pub fn minor(root_note: String) -> Chord {
        return Chord::new(root_note,  vec![0, 3, 7]);
    }
    pub fn diminished(root_note: String) -> Chord {
        return Chord::new(root_note,  vec![0, 4, 6]);
    }
    pub fn sus(root_note: String) -> Chord {
        return Chord::new(root_note, vec![0, 5, 7]);
    }
    pub fn set_extensions(&mut self, intervals: &mut Vec<Interval>){
        (*self).intervals.append(intervals);
    }
    pub fn to_note_vec(&self) -> Vec<u8> {
        let res: Option<&u8> = (*self).midi_to_note_map.get(&(*self).root_note);
        match res {
            Some(root_midi) => {
                let note_vec: Vec<u8> = (*self).intervals.iter().map(|&x| *root_midi + x).collect();
                note_vec
            }
            None => panic!("Could not find note vec")
        }
        
    }
}
