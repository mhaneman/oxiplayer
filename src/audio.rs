use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Arc<Mutex<Option<Sink>>>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;

        Ok(AudioPlayer {
            _stream: stream,
            stream_handle,
            sink: Arc::new(Mutex::new(None)),
        })
    }

    pub fn play<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        // Stop any currently playing audio
        self.stop();

        // Create a new sink
        let sink = Sink::try_new(&self.stream_handle)?;

        // Open the audio file
        let file = File::open(path.as_ref())?;
        let source = Decoder::new(BufReader::new(file))?;

        // Add the source to the sink and play
        sink.append(source);
        sink.play();

        // Store the sink
        *self.sink.lock().unwrap() = Some(sink);

        Ok(())
    }

    pub fn stop(&mut self) {
        if let Ok(mut sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.take() {
                sink.stop();
            }
        }
    }

    pub fn pause(&mut self) {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                sink.pause();
            }
        }
    }

    pub fn resume(&mut self) {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                sink.play();
            }
        }
    }

    pub fn is_paused(&self) -> bool {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                return sink.is_paused();
            }
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                return sink.empty();
            }
        }
        true
    }

    pub fn set_volume(&mut self, volume: f32) {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                sink.set_volume(volume.clamp(0.0, 1.0));
            }
        }
    }
}
