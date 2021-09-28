use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::execute;
use std::io::{stdout, Write};
#[path = "loader.rs"] pub mod loader;
#[path = "voice.rs"] pub mod voice;

pub fn init_inputs_watches(voices: & mut Vec<voice::Voice>, loader: & mut loader::BankLoader) {
    let mut stdout = stdout();
    //going into raw mode
    enable_raw_mode().unwrap();

    //clearing the screen, going to top left corner and printing welcoming message
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), Print(r#"p to exit"#))
            .unwrap();

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
            }) => voices[0].triggerSound(0, voice::PlaybackType::OneShot),
            Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: _no_modifiers,
            }) => voices[1].triggerSound(1, voice::PlaybackType::OneShot),
            Event::Key(KeyEvent {
                code: KeyCode::Char('x'),
                modifiers: _no_modifiers,
            }) => voices[2].triggerSound(2, voice::PlaybackType::OneShot),
            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: _no_modifiers,
            }) => voices[3].triggerSound(3, voice::PlaybackType::OneShot),
            
            // Bank changes
            Event::Key(KeyEvent {
                code: KeyCode::Char('i'),
                modifiers: _no_modifiers,
            }) => loader.switch_to_previous_bank(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('o'),
                modifiers: _no_modifiers,
            }) => loader.switch_to_next_bank(),

            // Shut down
            Event::Key(KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: _no_modifiers,
            }) => break,
            _ => return
        }
    }
            
    //disabling raw mode
    disable_raw_mode().unwrap();
}