use std::collections::HashMap; 
use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

fn main() {

    //https://en.wikipedia.org/wiki/Guitar_tunings
    //let guitar_tuning: [f32; 6] = [329.63, 246.94, 196.00, 146.83, 110.00, 82.41];
    
    let mut guitar: HashMap<&str, f32> = HashMap::new(); 
    guitar.insert("1E0", 329.63); 
    guitar.insert("2B0", 246.94);
    guitar.insert("3G0", 196.00);
    guitar.insert("4D0", 146.83);
    guitar.insert("5A0", 110.00);
    guitar.insert("6E0", 82.41);
    guitar.insert("1A5", 440.00);
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = SineWave::new(*guitar.get("3G0").unwrap()).take_duration(Duration::from_secs_f32(1.0)).amplify(0.20);
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}

