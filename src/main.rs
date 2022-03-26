use std::{error::Error, time::Duration};
use rusty_audio::Audio;
use std::io;
use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, cursor::{Hide, Show}, event::{self, Event, KeyCode}};
fn main() -> Result<(),Box<dyn Error>>{
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    //Terminal

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?; 
    stdout.execute(EnterAlternateScreen)?;//A new screen when you go to something like debug mode
    stdout.execute(Hide)?;


    //Game Loop
    'gameloop : loop{
        //Input
        while event::poll(Duration::default())?{
            if let Event::Key(key_event) = event::read()?{
                match key_event.code{
                    KeyCode::Char('q') | KeyCode::Esc => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {} 
                }

            }
        }
    }

    //Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())

}
