extern crate twang; // for sound generation / effects
extern crate adi; // for speaker

use adi::speaker::Speaker;
use twang::{Sound};

fn main() {
	let mut speaker = Speaker::new(0, false).unwrap();
	let mut snds = Sound::new(None, 440.0); // A4

	loop {
		speaker.update(&mut || {
			snds.next().unwrap().sqr().into()
		});
	}
}