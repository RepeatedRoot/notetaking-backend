use crate::utils::{Notes, Note, Clinician};

impl Notes {
    pub fn new() -> Self {
        Notes {
            id: None,
            notes: Vec::new()
        }
    }

    pub fn add_note(&mut self, note: Note) {
        println!("Adding note...");

        for i in 0..self.notes.len() {
            println!("{:?}", self.notes[i]);
        }
    }
}

