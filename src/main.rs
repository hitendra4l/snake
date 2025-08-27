use std::{
    io::{Error, Stdout, Write, stdout},
    process::exit,
    time::Duration,
};

use crossterm::{
    QueueableCommand,
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode},
    style::Print,
    terminal::{self, Clear, ClearType},
};

fn main() {
    terminal::enable_raw_mode().expect("Couldn't enable raw mode for terminal.");

    let (cols, rows) = terminal::size().expect("Error getting size of terminal.");
    let mut stdout = stdout();

    let _ = stdout.queue(Clear(ClearType::All));

    draw_border(&mut stdout, 0, 0, cols, rows).expect("Error drawing border.");

    stdout.flush().expect("Error drawing to terminal.");

    loop {
        if event::poll(Duration::from_millis(300)).unwrap() {
            match event::read().unwrap() {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        let _ = exit_program(&mut stdout);
                    }
                }
                Event::Resize(x, y) => println!("{x}, {y}"),
                _ => {}
            }
        }
    }
}

fn draw_border(stdout: &mut Stdout, x: u16, y: u16, width: u16, height: u16) -> Result<(), Error> {
    let horizontal = "█".repeat(width as usize);

    stdout
        .queue(cursor::MoveTo(x, y))?
        .queue(Print(&horizontal))?;

    stdout
        .queue(cursor::MoveTo(x, y + height - 1))?
        .queue(Print(&horizontal))?;

    for row in 1..height - 1 {
        stdout
            .queue(cursor::MoveTo(x, y + row))?
            .queue(Print("██"))?;

        stdout
            .queue(cursor::MoveTo(x + width - 2, y + row))?
            .queue(Print("██"))?;
    }

    Ok(())
}

fn exit_program(stdout: &mut Stdout) -> Result<(), Error> {
    stdout.queue(Clear(ClearType::All))?.queue(MoveTo(0, 0))?;
    stdout.flush().unwrap();
    terminal::disable_raw_mode().unwrap();
    exit(0);
}
