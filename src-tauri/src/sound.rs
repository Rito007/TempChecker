use std::io::BufReader;
use std::fs::File;
use rodio::{OutputStream, Sink};


pub fn play_notification_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(2.0);

    let file = File::open("./../src-tauri/assets/notification_sound.mp3").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}