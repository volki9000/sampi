/* mixer(){
    for v in voices
    {
        output += v.out();
    }
    // Hold back some samples for soft clipping or use a simple wave shaper?
    return output;
} */

use crate::voice;
use crate::loader;


pub struct Mixer {
    pub voices: Vec<voice::Voice>, // voices to mix
    pub loader: loader::BankLoader // handling loading samples into banks and switching them
}

impl Mixer {
    pub fn new() -> Mixer {
        Mixer {
            voices: vec![
                voice::Voice::new(),
                voice::Voice::new(),
                voice::Voice::new(),
                voice::Voice::new(),
                voice::Voice::new(),
                voice::Voice::new(),
                voice::Voice::new(),
                voice::Voice::new(),
                voice::Voice::new(),
                voice::Voice::new(),
                voice::Voice::new(),
                voice::Voice::new(),
                voice::Voice::new(),
            ],
            loader: loader::BankLoader::new()
        }
    }

    pub fn get_smp(&mut self) -> Vec<i16> {
        let mut mix = (0f32, 0f32);
        for v in &mut self.voices
        {
            let current_smp_int = v.out(&self.loader.bank);
            let current_smp_flt = (current_smp_int.0 as f32, current_smp_int.1 as f32);
            mix.0 = mix.0 + current_smp_flt.0;
            mix.1 = mix.1 + current_smp_flt.1;
        }
        let len_flt = self.voices.len() as f32;
        mix.0 = mix.0 / len_flt;
        mix.1 = mix.1 / len_flt;
        vec![mix.0 as i16, mix.1 as i16]
    }

    pub fn stop_sound(&mut self) -> () {
        for v in &mut self.voices
        {
            v.stop_sound();
        }
    }
}
