mod parse_args;

use parse_args::Frame;
use std::fmt::{Display, Formatter, Result};

enum VerticalDirection {
    Up,
    Down,
}

enum HorizontalDirection {
    Left,
    Right,
}

struct Ball {
    x: u32,
    y: u32,
    vertical_direction: VerticalDirection,
    horizontal_direction: HorizontalDirection,
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horizontal_direction = HorizontalDirection::Right;
        } else if self.x == frame.width - 1 {
            self.horizontal_direction = HorizontalDirection::Left;
        }

        if self.y == 0 {
            self.vertical_direction = VerticalDirection::Down;
        } else if self.y == frame.height - 1 {
            self.vertical_direction = VerticalDirection::Up;
        }
    }

    fn mv(&mut self) {
        match self.horizontal_direction {
            HorizontalDirection::Left => self.x -= 1,
            HorizontalDirection::Right => self.x += 1,
        }

        match self.vertical_direction {
            VerticalDirection::Up => self.y -= 1,
            VerticalDirection::Down => self.y += 1,
        }
    }
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new(frame: Frame) -> Game {
        let ball = Ball {
            x: 2,
            y: 4,
            vertical_direction: VerticalDirection::Up,
            horizontal_direction: HorizontalDirection::Left,
        };
        Game { frame, ball }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        let write_horizontal_line = |fmt: &mut Formatter| {
            write!(fmt, "+")?;
            for _ in 0..self.frame.width {
                write!(fmt, "-")?;
            }
            writeln!(fmt, "+")
        };
        write_horizontal_line(fmt)?;

        for row in 0..self.frame.height {
            write!(fmt, "|")?;

            for column in 0..self.frame.width {
                let c = if column == self.ball.x && row == self.ball.y {
                    'o'
                } else {
                    ' '
                };
                write!(fmt, "{}", c)?;
            }

            writeln!(fmt, "|")?;
        }

        write_horizontal_line(fmt)?;

        Ok(())
    }
}

fn main() -> std::result::Result<(), parse_args::ParseError> {
    let frame = parse_args::parse_args()?;
    let mut game = Game::new(frame);
    let sleep_duration = std::time::Duration::from_millis(33);
    loop {
        println!("{}", game);
        game.step();
        std::thread::sleep(sleep_duration)
    }
}
