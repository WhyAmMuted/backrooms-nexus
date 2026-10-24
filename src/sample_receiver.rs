use std::sync::Arc;

mod disk;
mod network;
mod slice;

pub struct Storage {
    audio: Vec<Arc<Vec<f32>>>,
}

impl Storage {
    pub fn get(&self, audio_id: usize) -> Option<Arc<Vec<f32>>> {
        self.audio.get(audio_id).cloned()
    }
    pub fn push(&mut self, audio: Vec<f32>) -> usize {
        self.audio.push(Arc::new(audio));
        self.audio.len() - 1
    }
    pub fn exist(&self, audio_id: usize) -> bool {
        audio_id < self.audio.len()
    }
}
