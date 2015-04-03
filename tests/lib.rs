extern crate sdl2;

use std::path::Path;

#[test]
fn audio_spec_wav() {
    let wav = sdl2::audio::AudioSpecWAV::load_wav(&Path::new("./tests/sine.wav")).unwrap();

    assert_eq!(wav.freq, 22050);
    assert_eq!(wav.format, sdl2::audio::AUDIOS16LSB);
    assert_eq!(wav.channels, 1);

    let buffer = wav.get_buffer();
    assert_eq!(buffer.len(), 4410);
}
