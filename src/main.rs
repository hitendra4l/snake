use std::{
    io::{Error, Write, stdout},
    process,
    time::Duration,
};

use crossterm::{
    QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    terminal::{self, Clear, ClearType},
};

struct TerminalGuard;

impl TerminalGuard {
    fn new() -> Self {
        terminal::enable_raw_mode().unwrap();
        let mut stdout = stdout();
        stdout.queue(Hide).unwrap();
        stdout.flush().unwrap();
        TerminalGuard
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut stdout = stdout();
        let _ = terminal::disable_raw_mode();
        let _ = stdout
            .queue(Clear(ClearType::All))
            .and_then(|s| s.queue(Show))
            .and_then(|s| s.queue(crossterm::cursor::MoveTo(0, 0)))
            .and_then(|s| s.flush());
    }
}

fn main() -> Result<(), Error> {
    let _guard = TerminalGuard::new();

    let (cols, rows) = terminal::size()?;
    let mut game_state = snake::initialize_game(cols, rows)?;

    loop {
        if event::poll(Duration::from_millis(150))? {
            match event::read()? {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('a') | KeyCode::Char('h') | KeyCode::Left => {
                            game_state.change_direction(snake::Direction::Left)
                        }

                        KeyCode::Char('d') | KeyCode::Char('l') | KeyCode::Right => {
                            game_state.change_direction(snake::Direction::Right)
                        }

                        KeyCode::Char('w') | KeyCode::Char('k') | KeyCode::Up => {
                            game_state.change_direction(snake::Direction::Up)
                        }

                        KeyCode::Char('s') | KeyCode::Char('j') | KeyCode::Down => {
                            game_state.change_direction(snake::Direction::Down)
                        }
                        _ => {}
                    }
                    if event.code == KeyCode::Char('q') {
                        break;
                    }
                }
                Event::Resize(x, y) => println!("{x}, {y}"),
                _ => {}
            }
        }

        let should_quit = game_state.move_snake();
        if should_quit {
            let _ = exit_program();
        }
    }
    exit_program()
}

fn exit_program() -> Result<(), Error> {
    terminal::disable_raw_mode()?;
    let mut stdout = stdout();

    stdout
        .queue(Clear(ClearType::All))?
        .queue(MoveTo(0, 0))?
        .queue(Show)?;

    stdout.flush()?;
    process::exit(0);
}
