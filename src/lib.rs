use std::{
    collections::VecDeque,
    io::{Error, Stdout, Write, stdout},
};

use crossterm::{
    QueueableCommand,
    cursor::{self},
    style::Print,
    terminal::{self, Clear, ClearType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn is_opposite(self, other: Direction) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }
}

pub struct GameState {
    /// First position would be head.
    snake: VecDeque<(u16, u16)>,
    direction: Direction,
    food: (u16, u16),
}

impl GameState {
    pub fn new(terminal_size: (u16, u16)) -> Self {
        let pos = (terminal_size.0 / 2, terminal_size.1 / 2);
        let mut stdout = stdout();
        stdout
            .queue(cursor::MoveTo(pos.0, pos.1))
            .unwrap()
            .queue(Print("██"))
            .unwrap();

        let snake = VecDeque::from([pos]);

        Self {
            snake: snake.clone(),
            direction: Direction::Right,
            food: draw_food_at_random_pos(&snake).unwrap(),
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if !self.direction.is_opposite(new_direction) {
            self.direction = new_direction;
        }
    }

    pub fn move_snake(&mut self) -> bool {
        let mut stdout = stdout();
        let old_head = self.snake[0];
        let new_head = match self.direction {
            Direction::Up => (old_head.0, old_head.1 - 1),
            Direction::Down => (old_head.0, old_head.1 + 1),
            Direction::Left => (old_head.0 - 2, old_head.1),
            Direction::Right => (old_head.0 + 2, old_head.1),
        };
        self.snake.push_front(new_head);

        stdout
            .queue(cursor::MoveTo(new_head.0, new_head.1))
            .unwrap()
            .queue(Print("██"))
            .unwrap();

        if new_head == self.food {
            stdout
                .queue(cursor::MoveTo(self.food.0, self.food.1))
                .unwrap()
                .queue(Print("██"))
                .unwrap();

            self.food = draw_food_at_random_pos(&self.snake).unwrap();
        } else {
            if let Some(tail) = self.snake.pop_back() {
                stdout
                    .queue(cursor::MoveTo(tail.0, tail.1))
                    .unwrap()
                    .queue(Print("  "))
                    .unwrap();
            }
        }

        stdout.flush().unwrap();

        let terminal_size = terminal::size().unwrap();
        // Collision detection
        if new_head.0 <= 2
            || new_head.0 >= terminal_size.0 - 2
            || new_head.1 <= 0
            || new_head.1 >= terminal_size.1 - 1
            || self.snake.iter().skip(1).any(|&pos| pos == new_head)
        {
            return true;
        }

        false
    }
}

pub fn draw_border(stdout: &mut Stdout, width: u16, height: u16) -> Result<(), Error> {
    let x: u16 = 0;
    let y: u16 = 0;

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

pub fn initialize_game(width: u16, height: u16) -> Result<GameState, Error> {
    let mut stdout = stdout();
    stdout.queue(Clear(ClearType::All))?;
    draw_border(&mut stdout, width, height)?;
    let game_state = GameState::new((width, height));

    stdout.flush()?;

    Ok(game_state)
}

pub fn draw_food_at_random_pos(snake: &VecDeque<(u16, u16)>) -> Result<(u16, u16), Error> {
    let terminal_size = terminal::size().unwrap();
    let snake_head = snake[0];

    let coord = loop {
        let mut x = rand::random_range(2..=terminal_size.0 - 4);
        if x % 2 != snake_head.0 % 2 {
            x += 1;
            if x > terminal_size.0 - 4 {
                x -= 2;
            }
        }

        let coords = (x, rand::random_range(1..=terminal_size.1 - 2));

        if !snake.contains(&coords) {
            break coords;
        }
    };

    let variants = ["◉▄", "▄◉", "▀◉", "◉▀"];
    let food_idx: usize = rand::random_range(0..4);
    let mut stdout = stdout();

    stdout
        .queue(cursor::MoveTo(coord.0, coord.1))?
        .queue(Print(variants[food_idx]))?;

    stdout.flush()?;

    Ok(coord)
}
