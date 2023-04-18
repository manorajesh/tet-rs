use std::{thread, io::BufReader, fs::File};

use rodio::{Sink, Decoder, OutputStream};

pub struct MusicPlayer {
    sink: Sink,
    thread_handle: Option<thread::JoinHandle<()>>
}

impl MusicPlayer {
    pub fn new() -> MusicPlayer {
        // Get a output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to get output stream");

        // Create a new sink to play the music
        let sink = Sink::try_new(&stream_handle).unwrap();

        MusicPlayer {
            sink,
            thread_handle: None,
        }
    }

    pub fn start(&mut self, path: &str) {
        // Load a sound from a file or raw bytes
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();

        // Play the sound directly on the device (async)
        self.sink.append(source);
        self.sink.detach();

        // Start music on a new thread
        let thread_handle = thread::spawn(move || {
            // Keep function running until sink is stopped
            while self.sink.empty() {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        // Store the thread handle
        self.thread_handle = Some(thread_handle);
    }

    pub fn stop(&mut self) {
        // Stop the sink
        self.sink.stop();

        // Join the thread
        if let Some(thread_handle) = self.thread_handle.take() {
            thread_handle.join().expect("Failed to stop music thread");
        }
    }
}