#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Letter {
    fn notation(&self) -> char {
        match *self {
            Letter::A => 'A',
            Letter::B => 'B',
            Letter::C => 'C',
            Letter::D => 'D',
            Letter::E => 'E',
            Letter::F => 'F',
            Letter::G => 'G',
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Note(Letter, isize, usize);

impl Note {
    pub fn new(letter: Letter, accidental: isize, octave: usize) -> Self {
        // TODO: Proper simplification.
        if letter == Letter::B && accidental == 1 {
            return Note(Letter::C, 0, octave);
        }

        Note(letter, accidental, octave)
    }

    pub fn parse(note: &str) -> Self {
        Note::new(Letter::C, 0, 5)
    }

    fn name(&self) -> String {
        if self.1 == 0 {
            format!("{}", self.0.notation())
        } else if self.1 > 0 {
            format!("{}{}", self.0.notation(), "#".repeat(self.1.abs() as usize))
        } else {
            format!("{}{}", self.0.notation(), "♭".repeat(self.1.abs() as usize))
        }
    }

    fn octave(&self) -> usize {
        self.2
    }

    // fn sophige(&self);
    // fn frequency<Tuning>(&self);  // pitch?

    // fn normalize(&mut self) {
    //     // Letter pass.
    //     if self.0 == "C##" {
    //         self.0 = "D".to_owned();
    //     } else if self.0 == "B#" {
    //         self.0 = "C".to_owned();
    //     }

    //     // Octave pass.
    //     if self.0 == "C" {
    //         self.0 = "C5".to_owned();
    //     }
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Note::new(Letter::C, 0, 5), Note::parse("C"));
        assert_eq!(Note::new(Letter::C, 0, 4), Note::parse("C4"));
        assert_eq!(Note::new(Letter::B, 1, 5), Note::parse("B#"));
        assert_eq!(Note::new(Letter::C, 0, 5), Note::parse("B#"));
        assert_eq!(Note::new(Letter::B, 1, 5), Note::parse("B#5"));
        assert_eq!(Note::new(Letter::D, 0, 5), Note::parse("C##"));
        assert!(Note::new(Letter::D, 0, 3) != Note::new(Letter::D, 0, 4));
    }

    #[test]
    fn names() {
        let c3 = Note::new(Letter::C, 0, 3);
        let c4 = Note::new(Letter::C, 0, 4);
        let bs3 = Note::new(Letter::B, 1, 3);
        let cs5 = Note::new(Letter::C, 1, 5);
        assert_eq!(c3.name(), "C");
        assert_eq!(c3.name(), c4.name());
        assert_eq!(c3.name(), bs3.name());
        assert_eq!(cs5.name(), "C#");
    }

    #[test]
    fn parse() {
        assert_eq!(Note::new(Letter::A, 0, 5),  Note::parse("A"));
        assert_eq!(Note::new(Letter::C, 0, 5),  Note::parse("C"));
        assert_eq!(Note::new(Letter::C, -1, 5), Note::parse("Cb"));
        assert_eq!(Note::new(Letter::C, 1, 5),  Note::parse("C#"));
        assert_eq!(Note::new(Letter::C, -1, 5), Note::parse("♭C"));
        assert_eq!(Note::new(Letter::C, 1, 5),  Note::parse("♯C"));
    }
}
