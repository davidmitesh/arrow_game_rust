use std::{error::Error, time::{Duration, Instant}, sync::mpsc, thread};
use arrow_game_rust::{frame::{new_frame, self, Drawable}, render::render, player::Player};
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

    //Making render loop in a separate thread

    let (render_tx,render_rx) = mpsc::channel();//For simple purpose,mpsc channel is used, but in production level apps use crossbeam
    let render_handle = thread::spawn(move ||{
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render(&mut stdout, &last_frame, &last_frame, true);
        loop{
            let curr_frame =  match render_rx.recv(){
                Ok(x) => x,
                Err(_)=> break,
            };
            render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });
    //Game Loop
    let  mut player = Player::new();
    let mut instant = Instant::now();
    'gameloop : loop{
        //Per-frame init section
        let delta = instant.elapsed(); 
        instant = Instant::now();
        let mut curr_frame = new_frame();
        //Input
        while event::poll(Duration::default())?{
            if let Event::Key(key_event) = event::read()?{
                match key_event.code{
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Enter | KeyCode::Char(' ') =>  {
                        if player.shoot(){
                            audio.play("pew");
                        }
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {} 
                }

            }
        }

        //Updates
        player.update(delta);

        //Draw and render
        player.draw(&mut curr_frame);
       let _ =  render_tx.send(curr_frame);
       thread::sleep(Duration::from_millis(1));
    }

    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())

}
