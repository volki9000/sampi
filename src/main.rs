/*use cpal;
use anyhow;
use hound;
use crate::cpal::traits::DeviceTrait;
use crate::cpal::traits::HostTrait;
use crate::cpal::traits::StreamTrait;*/
//use std::{thread::sleep, time::Duration};

mod input;
mod mixer;

/* main(){
    initAudioBoilerPlate();
    initInputWatches();
    initBanks();
    initBankChangeWatches();
    loop{
        outputloop(mixer());
    }
} */

fn main() {
    let mut loader = input::loader::BankLoader::new();
    let mut voices = vec![
        input::voice::Voice::new(),
        input::voice::Voice::new(),
        input::voice::Voice::new(),
        input::voice::Voice::new(),
        input::voice::Voice::new(),
        input::voice::Voice::new(),
        input::voice::Voice::new(),
        input::voice::Voice::new(),
        input::voice::Voice::new(),
        input::voice::Voice::new(),
        input::voice::Voice::new(),
        input::voice::Voice::new(),
        input::voice::Voice::new(),
    ];

    std::thread::spawn(move || {
        input::init_inputs_watches(&mut voices, &mut loader);
    });

    // loop {
    //     sleep(Duration::from_millis(100));
    // }
}

/* fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("Default output device not found");
    let config = device.default_output_config().unwrap();
    println!("Default device: {:?}", device.name());
    println!("Default output config: {:?}", config);

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into())
    }
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let channels = config.channels as usize;

    let reader = hound::WavReader::open("samples/808kick.wav").expect("Couldn't open samples/808kick.wav");
    let mut sample = reader.into_samples::<i16>();

    // Get next sample
    let mut compute_value = move || {
        let smp = sample.next();
        match smp {
            None => 0,
            Some(value) => {
                value.expect("Could not read sample data.") as i16
            }
        }
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut compute_value)
        },
        err_fn,
    )?;
    stream.play()?;
    std::thread::sleep(std::time::Duration::from_millis(2500));
    Ok(())
}

fn write_data<T, F>(output: &mut [T], channels: usize, next_sample: &mut F
)
where
    T: cpal::Sample,
    F: FnMut() -> i16
{
    for frame in output.chunks_mut(channels) {
        let input = &next_sample();
        let value: T = cpal::Sample::from::<i16>(input);
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
} */
