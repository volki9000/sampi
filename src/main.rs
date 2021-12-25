use cpal;
use anyhow;
use crate::cpal::traits::DeviceTrait;
use crate::cpal::traits::HostTrait;
use crate::cpal::traits::StreamTrait;
//use std::{thread::sleep, time::Duration};

mod input;
mod mixer;
mod loader;
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
    let mut mixer = mixer::Mixer::new();

    std::thread::spawn(move || {
        input::init_inputs_watches(&mut mixer);
    });

    let host = cpal::default_host();
    let device = host.default_output_device().expect("Default output device not found");
    let config = device.default_output_config().unwrap().into();
    println!("Default device: {:?}", device.name());
    println!("Default output config: {:?}", config);

    let output_data_fn = move |output_data: &mut [i16], _: &cpal::OutputCallbackInfo| {
        let mixer_out = mixer.get_smp();
        output_data[0] = mixer_out[0];
        output_data[1] = mixer_out[1];
    };

    let stream = device.build_output_stream(
        &config,
        output_data_fn,
        err_fn,
    )?;
    stream.play()?;

    loop
    {
    }

    //drop(stream);

    //Ok(())
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err)
}
