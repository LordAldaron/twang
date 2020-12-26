use fon::{mono::Mono64, Audio, Sink};
use twang::Synth;

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    // Initialize audio with five seconds of silence.
    let mut audio = Audio::<Mono64>::with_silence(S_RATE, S_RATE as usize * 5);
    // Create the synthesizer.
    let mut synth =
        Synth::new(|fc| fc.freq(440.0).abs().gain(fc.freq(440.0).sine()));

    // Generate audio samples.
    audio.sink(..).stream(&mut synth);

    // Write chord to file
    wav::write(audio, "voice.wav").expect("Failed to write WAV file");
}
