## Features

- `Note::parse("C#4")`
- `Interval::parse("F#3-G#3")`
- `Chord::parse("m7")`
- `Scale::parse("D# minor")`, `Note::parse("D#").scale("minor")`


```rs
let _a = Note::parse("A");
let _c = Note::parse("C");
let _cb = Note::parse("♭C");  // or Note::parse("Cb");
let _cs = Note::parse("♯C");  // or Note::parse("C#")

let interval1 = Interval::parse("C-G");
assert_eq!(Interval::Fifth, interval1);
assert_eq!(Quality::Perfect, interval1.quality());
assert_eq!(interval1.steps(), 3.5);
assert_eq!(Interval::Fourth, -interval1);
assert_eq!(Quality::Perfect, -interval1.quality());

let chord1 = Chord::parse("CM6");
assert!(!chord1.is_triad());

let C = Scale::parse("C");
let am_harmonic = _a.scale("minor harmonic")
assert!(C.is_tonic(Note::parse("C")));
assert!(C.is_tonic(Note::parse("C6")));
assert_eq!(Note::parse("C"), C.tonic());
assert!(C.is_dominant(Note::parse("G")));
assert_eq!(Note::parse("G"), C.dominant());
assert!(C.is_subdominant(Note::parse("F")));
assert_eq!(Note::parse("F"), C.subdominant());
assert!(C.is_mediant(Note::parse("E")));
assert_eq!(Note::parse("E"), C.mediant());
assert!(C.is_submediant(Note::parse("A6")));
assert_eq!(Note::parse("A6"), C.submediant());
assert_eq!(Note::parse("D"), C.subtonic());
assert!(C.is_leading(...));
assert_eq!(Some(Note::parse(...)), C.leading());

let a_minor = Key::new(Note::parse("A"), ...);
assert_eq!(vec!["A", "B", "C", "D", "E", "F", "G"],
           a_minor.scale().iter().map(|n| format!("{}", n)));
assert!(a_minor.scale().contains(chord1));

let a_minor_harmonic = Key::new(Note::parse("A)), ...);
assert_eq!(vec!["A", "B", "C", "D", "E", "F", "G#"],
           a_minor_harmonic.scale().iter().map(|n| format!("{}", n)));
```

## Composition

### Harmony and Melody

Chords make up the foundation of the "vertical" aspect of music, that is,
overlapping tones, used together in **harmony**. **Melody** makes up the
corresponding "horizontal" aspect of music. These melodies are linear
successions of tones, also called **tunes** or **lines**. Counterpoint roughly
refers to the relationship between the vertical and horizontal dimensions of
music.

#### Chord Progressions

The purpose of chord progressions is to either establish, or contradict a
specific tonality, often understood as the "key" of a piece. Chord progressions
can be written with Roman Numeral, independent of the key you play them in. For
example, in the key of C, the iii chord is E minor since the root of E minor is
the third of C.

Chord progressions can be very simple, with the most basic simply alternating
between two chords, often major and minor due to the large number of shared
perfect fifths. More common however are three chord progressions, often laid
out in four parts. For example, I - IV - V - IV. Different styles of music
often use different progressions.

#### Expectations

Different notes and chords can be used together to create dissonance
(instability) or consonance (stability). When a progression moves from dissonance
to consonance it's said to have resolved. Resolution has a satisfying effect
upon the listener, while delaying resolution can give a sense of suspense. A
**cadence** is a falling configuration (either horizontal or vertical) that
gives a sense of resolution. There are many types of cadences:

- Perfect Authentic, ...
TODO: https://en.wikipedia.org/wiki/Cadence

## Translations

### Tempo

Roughly in 4/4 time, though these words (generally coming from Italian) meaning
is subject to change. For more information see [1][tempo_wiki] and
[2][tempo_goodwin].

- Larghissimo – very, very slow (24 bpm and under)
- Adagissimo – very slowly
- Grave – very slow (25–45 bpm)
- Largo – broadly (40–60 bpm)
- Lento – slowly (45–60 bpm)
- Larghetto – rather broadly (60–66 bpm)
- Adagio – slowly with great expression (66–76 bpm)
- Adagietto – slower than andante (72–76 bpm) or slightly faster than adagio (70–80 bpm)
- Andante – at a walking pace (76–108 bpm)
- Andantino – slightly faster than andante (although, in some cases, it can be taken to mean slightly slower than andante) (80–108 bpm)
- Marcia moderato – moderately, in the manner of a march (83–85 bpm)
- Andante moderato – between andante and moderato (thus the name) (92–112 bpm)
- Moderato – at a moderate speed (108–120 bpm)
- Allegretto – by the mid 19th century, moderately fast (112–120 bpm); see paragraph above for earlier usage
- Allegro moderato – close to, but not quite allegro (116–120 bpm)
- Allegro – fast, quickly, and bright (120–156 bpm) (molto allegro is slightly faster than allegro, but always in its range)
- Vivace – lively and fast (156–176 bpm)
- Vivacissimo – very fast and lively (172–176 bpm)
- Allegrissimo or Allegro vivace – very fast (172–176 bpm)
- Presto – very, very fast (168–200 bpm)
- Prestissimo – even faster than presto (200 bpm and over)
- A piacere – the performer may use thier own discretion with regard to tempo
              and rhythm; literally "at pleasure"[11]


## Resources

- http://chuck.cs.princeton.edu/
- https://wiki.archlinux.org/index.php/List_of_applications/Multimedia
- https://github.com/monadgroup/axiom/issues/152


[tempo_wiki]: https://en.wikipedia.org/wiki/Tempo
[tempo_goodwin]: http://www.goodwinshighend.com/music/classical/tempo_glossary.htm
