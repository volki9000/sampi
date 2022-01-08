use crate::cpal::traits::DeviceTrait;
use crate::cpal::traits::HostTrait;
use crate::cpal::traits::StreamTrait;
use anyhow;
use cpal;
//use std::{thread::sleep, time::Duration};

mod input;
mod loader;
mod mixer;
mod voice;

/* main(){
    initAudioBoilerPlate();
    initInputWatches();
    initBanks();
    initBankChangeWatches();
    loop{
        outputloop(mixer());
    }
} */

fn main() -> anyhow::Result<()> {
    let mut sample_buffer: [(i16, i16); 1024] = [(0, 0); 1024];
    let mut buffer_position: usize = 0;

    std::thread::spawn(move || {
        let mut mixer = mixer::Mixer::new();
        input::init_inputs_watches(&mut mixer);
        loop {
            let smp = mixer.get_smp();
            sample_buffer[buffer_position] = (smp[0], smp[1]);
        }
    });

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Default output device not found");
    let config = device.default_output_config().unwrap().into();
    println!("Default device: {:?}", device.name());
    println!("Default output config: {:?}", config);
    let output_data_fn = move |output_data: &mut [i16], _: &cpal::OutputCallbackInfo| {
        output_data[0] = sample_buffer[buffer_position].0;
        output_data[1] = sample_buffer[buffer_position].1;
        buffer_position = (buffer_position + 1) % 1024;
    };
    let stream = device.build_output_stream(&config, output_data_fn, err_fn)?;
    stream.play()?;

    loop{}

//    drop(stream);

//    Ok(())
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err)
}
