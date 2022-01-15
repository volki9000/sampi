use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::execute;
use std::io::{stdout, Write};
use crate::mixer;
use crate::voice;

pub fn init_inputs_watches(mixer_in: & mut mixer::Mixer, sample_buffer: & mut [(i16, i16); 1024], buffer_position: & usize) {
    let mut stdout = stdout();
    //going into raw mode
//    enable_raw_mode().unwrap();

    //clearing the screen, going to top left corner and printing welcoming message
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), Print(r#"p to exit"#))
            .unwrap();

    let mut write_buffer_position = *buffer_position;

    //key detection
    loop {
        //going to top left corner
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

        let _no_modifiers = KeyModifiers::empty();
        //matching the key
        match read().unwrap() {
            // Keys
            Event::Key(KeyEvent {
                code: KeyCode::Char('y'),
                modifiers: _no_modifiers,
                //clearing the screen and printing our message
            }) => mixer_in.voices[0].trigger_sound(0, voice::PlaybackType::OneShot),
            Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: _no_modifiers,
            }) => mixer_in.voices[1].trigger_sound(1, voice::PlaybackType::OneShot),
            Event::Key(KeyEvent {
                code: KeyCode::Char('x'),
                modifiers: _no_modifiers,
            }) => mixer_in.voices[2].trigger_sound(2, voice::PlaybackType::OneShot),
            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: _no_modifiers,
            }) => mixer_in.voices[3].trigger_sound(3, voice::PlaybackType::OneShot),
            
            // Bank changes
            Event::Key(KeyEvent {
                code: KeyCode::Char('i'),
                modifiers: _no_modifiers,
            }) => mixer_in.loader.switch_to_previous_bank(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('o'),
                modifiers: _no_modifiers,
            }) => mixer_in.loader.switch_to_next_bank(),

            // Shut down
            Event::Key(KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: _no_modifiers,
            }) => mixer_in.stop_sound(),
            _ => break
        }

        if write_buffer_position != *buffer_position
        {
            let smp = mixer_in.get_smp();
            sample_buffer[write_buffer_position] = (smp[0], smp[1]);
            write_buffer_position = (write_buffer_position + 1) % 1024;
        }
    }
            
    //disabling raw mode
    disable_raw_mode().unwrap();
}