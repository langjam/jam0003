use std::io::Write;
use std::{
    fmt::Display,
    fs::File,
    io::Cursor,
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};

pub mod notes;

pub use notes::*;

use portmidi::OutputPort;

pub enum MidiMessage {
    NoteOn(Channel, Tone, Velocity),
    NoteOff(Channel, Tone),
    ProgramChange(Channel, u8),
    PitchBend(Channel, U14),
}

impl MidiMessage {
    pub fn note_on(
        channel: impl Into<Channel>,
        note: impl Into<Tone>,
        velocity: impl Into<Velocity>,
    ) -> Self {
        MidiMessage::NoteOn(channel.into(), note.into(), velocity.into())
    }
    pub fn note_off(channel: impl Into<Channel>, note: impl Into<Tone>) -> Self {
        MidiMessage::NoteOff(channel.into(), note.into())
    }

    pub fn play_accord(
        channel: impl Into<Channel>,
        notes: &[impl Into<Tone> + Clone],
    ) -> Vec<Self> {
        let mut messages = Vec::new();
        let channel = channel.into();
        for note in notes {
            messages.push(MidiMessage::note_on(channel, note.clone(), Velocity(127)));
        }
        messages
    }

    pub fn bend_pitch(channel: impl Into<Channel>, pitch: impl Into<U14>) -> Self {
        MidiMessage::PitchBend(channel.into(), pitch.into())
    }

    pub fn change_instrument(channel: impl Into<Channel>, instrument: impl Into<u8>) -> Self {
        MidiMessage::ProgramChange(channel.into(), instrument.into())
    }
}

impl Into<portmidi::MidiMessage> for MidiMessage {
    fn into(self) -> portmidi::MidiMessage {
        match self {
            MidiMessage::NoteOn(channel, note, velocity) => portmidi::MidiMessage {
                status: 0x90 + channel.0,
                data1: note as u8,
                data2: velocity.0,
                data3: 0,
            },
            MidiMessage::NoteOff(channel, note) => portmidi::MidiMessage {
                status: 0x80 + channel.0,
                data1: note as u8,
                data2: 0,
                data3: 0,
            },
            MidiMessage::ProgramChange(channel, instrument) => portmidi::MidiMessage {
                status: 0xC0 + channel.0,
                data1: instrument as u8,
                data2: 0,
                data3: 0,
            },
            MidiMessage::PitchBend(channel, U14(high, low)) => portmidi::MidiMessage {
                status: 0xE0 + channel.0,
                data1: low,
                data2: high,
                data3: 0,
            },
        }
    }
}

pub struct LiquidPiano {
    handle: Child,
    temp_dir: std::path::PathBuf,
}

fn create_temp_dir() -> std::path::PathBuf {
    let temp_dir = std::env::temp_dir();
    let temp_dir = temp_dir.join("liquid_piano");
    std::fs::create_dir_all(&temp_dir).unwrap();
    temp_dir
}

impl LiquidPiano {
    pub fn new() -> Self {
        let temp_dir = create_temp_dir();
        let file_path = temp_dir.join("soundfont.sf2");
        let mut f = File::create(file_path.clone()).unwrap();
        println!("{:?}", file_path);
        f.write_all(SOUNDFONT).unwrap();
        let handle = Command::new("fluidsynth")
            .arg("-i")
            .arg(file_path.to_str().unwrap())
            .arg("-s")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        thread::sleep(Duration::from_millis(1000));
        LiquidPiano { handle, temp_dir }
    }

    pub fn check_running(&mut self) {
        match self.handle.try_wait().unwrap() {
            Some(status) => {
                if !status.success() {
                    panic!("fluidsynth stopped running")
                }
            }
            None => {
                return;
            }
        }
    }
}

impl Drop for LiquidPiano {
    fn drop(&mut self) {
        self.handle.kill().unwrap();
        std::fs::remove_dir_all(&self.temp_dir).unwrap();
    }
}

pub struct MidiOutput<'a> {
    port: OutputPort<'a>,
}

impl From<MidiMessage> for Vec<MidiMessage> {
    fn from(message: MidiMessage) -> Self {
        vec![message]
    }
}

impl<'a> MidiOutput<'a> {
    pub fn new(port: OutputPort<'a>) -> Self {
        MidiOutput { port }
    }

    pub fn send_message(&mut self, msg: impl Into<Vec<MidiMessage>>) {
        for message in msg.into() {
            self.port.write_message(message).unwrap();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instrument {
    pub bank: u16,
    pub preset: u16,
    pub name: String,
}

impl Display for Instrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}: {}", self.bank, self.preset, self.name)
    }
}

const SOUNDFONT: &[u8] = include_bytes!("./chorium.sf2");

pub fn get_midi_output_port(ctx: &portmidi::PortMidi) -> MidiOutput {
    let fluid_id = ctx
        .devices()
        .unwrap()
        .iter()
        .find(|d| d.name().contains("Synth input port"))
        .unwrap()
        .id();

    let output = ctx
        .device(fluid_id)
        .and_then(|dev| ctx.output_port(dev, 1024))
        .unwrap();

    let mut p = MidiOutput::new(output);
    p.send_message(MidiMessage::change_instrument(0, 0));
    p
}

pub fn get_all_instruments() -> Vec<Instrument> {
    let mut f = Cursor::new(SOUNDFONT);
    let sf = soundfont::SoundFont2::load(&mut f).unwrap();
    let mut instruments = sf
        .presets
        .into_iter()
        .map(|preset| Instrument {
            bank: preset.header.bank,
            preset: preset.header.preset,
            name: preset.header.name,
        })
        // bank switching is still a mystery, so we only show bank0 instruments
        .filter(|i| i.bank == 0)
        .collect::<Vec<_>>();
    instruments.sort();
    instruments
}

// fn main() {
//     let _piano = LiquidPiano::new();
//     let ctx = portmidi::PortMidi::new().unwrap();
//     // for d in ctx.devices().unwrap() {
//     //     println!("{}", d);
//     // }
//     let fluid_id = ctx
//         .devices()
//         .unwrap()
//         .iter()
//         .find(|d| d.name().contains("Synth input port"))
//         .unwrap()
//         .id();
//
//     let output = ctx
//         .device(fluid_id)
//         .and_then(|dev| ctx.output_port(dev, 1024))
//         .unwrap();
//
//     let mut midi_output = MidiOutput::new(output);
//
//     midi_output.send_message(MidiMessage::change_instrument(0, 0));
//
//     midi_output.send_message(MidiMessage::bend_pitch(0, 0x2000));
//
//     let notes = {
//         use Tone::*;
//         vec![
//             (C4, NoteType::Quarter),
//             (C4, NoteType::Quarter),
//             (G4, NoteType::Quarter),
//             (G4, NoteType::Quarter),
//             (A4, NoteType::Quarter),
//             (A4, NoteType::Quarter),
//             (G4, NoteType::Half),
//             (F4, NoteType::Quarter),
//             (F4, NoteType::Quarter),
//             (E4, NoteType::Quarter),
//             (E4, NoteType::Quarter),
//             (D4, NoteType::Quarter),
//             (D4, NoteType::Quarter),
//             (C4, NoteType::Half),
//             (G4, NoteType::Quarter),
//             (G4, NoteType::Quarter),
//             (F4, NoteType::Quarter),
//             (F4, NoteType::Quarter),
//             (E4, NoteType::Quarter),
//             (E4, NoteType::Quarter),
//             (D4, NoteType::Half),
//             (G4, NoteType::Quarter),
//             (G4, NoteType::Quarter),
//             (F4, NoteType::Quarter),
//             (F4, NoteType::Quarter),
//             (E4, NoteType::Quarter),
//             (E4, NoteType::Quarter),
//             (D4, NoteType::Half),
//             (C4, NoteType::Quarter),
//             (C4, NoteType::Quarter),
//             (G4, NoteType::Quarter),
//             (G4, NoteType::Quarter),
//             (A4, NoteType::Quarter),
//             (A4, NoteType::Quarter),
//             (G4, NoteType::Half),
//             (F4, NoteType::Quarter),
//             (F4, NoteType::Quarter),
//             (E4, NoteType::Quarter),
//             (E4, NoteType::Quarter),
//             (D4, NoteType::Quarter),
//             (D4, NoteType::Quarter),
//             (C4, NoteType::Half),
//         ]
//     };
//
//     let bpm = NoteSpeed::new(72);
//
//     for (n, t) in notes {
//         midi_output.send_message(MidiMessage::note_on(0, n, 100));
//         thread::sleep(t.duration(&bpm));
//
//         midi_output.send_message(MidiMessage::note_off(0, n));
//     }
//     thread::sleep(Duration::from_millis(1000));
// }
