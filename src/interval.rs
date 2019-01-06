use std::ops;

pub struct Interval(Note, Note);

impl Interval {
    // fn parse(&str) -> Self;
    // fn new(a: Note, b: Note) -> Self;
    // fn from(a: Note, b: Note) -> Self;  // alias for `new`.
    // fn number(&self);
    // fn quality(&self);  // P, M, m, A, d.
    // fn semitones(&self);
    // fn transpose(&self, &Note) -> Note;
}

impl ops::Add for Interval {
    type Output = Interval;

    fn add(self, other: Interval) -> Interval {
        unimplemented!();
    }
}

impl ops::Sub for Interval {
    type Output = Interval;

    fn sub(self, other: Interval) -> Interval {
        unimplemented!();
    }
}

impl ops::Neg for Interval {
    type Output = Interval;

    fn neg(self) -> Interval {
        unimplemented!();
    }
}
