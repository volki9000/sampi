/* voice{
    bool play;                   // play sample?
    bool loop;                   // loop sample?
    uint activeSample;           // which sample to play
    uint activeSamplePlayHead;   // current sample read position
    out(){
        if !loop && activeSamplePlayHead >= banks[currentBank][activeSample].size {
            play = false;
        }
        if !play {
            return;
        }
        return banks[currentBank][activeSample][activeSamplePlayHead++];
    }
} */

#[derive(Copy, Clone)]
pub enum PlaybackType {
    OneShot,
    Gated,
    Looped
}

#[derive(Copy, Clone)]
pub struct Voice {
    pub play: bool,                  // play sample?
    playback_type: PlaybackType,  // loop sample?
    active_sample: usize,         // which sample to play
    playhead_position: usize, // current sample read position
}

impl Voice {
    pub fn new() -> Voice {
        Voice {
            play: false,
            playback_type: PlaybackType::OneShot,
            active_sample: 0,
            playhead_position: 0,
        }
    }

     pub fn trigger_sound(&mut self, sampleIndex: usize, pbType: PlaybackType) -> () {
        self.play = true;
        self.playback_type = pbType;
        self.active_sample = sampleIndex;
        self.playhead_position = 0;
    }

    pub fn stop_sound(&mut self) -> () {
        if matches!(self.playback_type, PlaybackType::OneShot) {
            return;
        }
        self.play = false;
        self.playhead_position = 0;
    }

    pub fn out(&mut self, bank: &Vec<Vec<(u16, u16)>>) -> (u16, u16) {
        let sample_size = bank[self.active_sample].len();
        if matches!(self.playback_type, PlaybackType::Looped)
            && self.playhead_position >= sample_size
        {
            self.play = false;
        }
        if !self.play {
            return (0, 0)
        }
        let current_sample = bank[self.active_sample][self.playhead_position];
        self.playhead_position = self.playhead_position + 1;
        current_sample
    }
}
