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
pub enum IntervalEnum {
    PerfectUnison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    Tritone,
    PerfectFifth,
    AugmentedFifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    PerfectOctave,
    MinorNinth,
    MajorNinth,
    SharpEleven,
    MajorThirteenth
} 

pub struct Chord {
    root_note: String,
    triad_intervals: Vec<Interval>,
    extensions: Vec<Interval>,
    note_to_midi_map: HashMap<u8, Vec<String>>,
    midi_to_note_map: HashMap<String, u8>
}
impl Chord {
    pub fn new(root_note: String, intervals: Vec<Interval>) -> Chord {
        let midi_touple: (HashMap<u8, Vec<String>>, HashMap<String, u8>) = initialize_midi_map();
        Chord {
            root_note: root_note,
            triad_intervals: intervals,
            extensions: Vec::new(),
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
    pub fn set_extensions(&mut self, intervals: &mut Vec<IntervalEnum>){
        (*self).extensions = intervals.iter().map(|i| { Chord::get_interval_from_enum(i)}).collect();
    }
    fn get_interval_from_enum(interval: &IntervalEnum) -> Interval {
        match interval {
            IntervalEnum::PerfectUnison => 0,
            IntervalEnum::MinorSecond => 1,
            IntervalEnum::MajorSecond => 2,
            IntervalEnum::MinorThird => 3,
            IntervalEnum::MajorThird => 4,
            IntervalEnum::PerfectFourth => 5,
            IntervalEnum::Tritone => 6,
            IntervalEnum::PerfectFifth => 7,
            IntervalEnum::AugmentedFifth => 8,
            IntervalEnum::MinorSixth => 8,
            IntervalEnum::MajorSixth => 9,
            IntervalEnum::MinorSeventh => 10,
            IntervalEnum::MajorSeventh => 11,
            IntervalEnum::PerfectOctave => 12,
            IntervalEnum::MinorNinth => 13,
            IntervalEnum::MajorNinth => 14,
            IntervalEnum::SharpEleven => 18,
            IntervalEnum::MajorThirteenth => 21,
        }
    }    
    pub fn to_note_vec(&self) -> Vec<u8> {
        let res: Option<&u8> = (*self).midi_to_note_map.get(&(*self).root_note);
        match res {
            Some(root_midi) => {
                let note_vec: Vec<u8> = (*self).triad_intervals.iter().chain(&(*self).extensions).map(|&x| *root_midi + x).collect();
                note_vec
            }
            None => panic!("Could not find midi map")
        }
        
    }
}
