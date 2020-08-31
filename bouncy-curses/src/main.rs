use pancurses::{endwin, initscr, Input, Window};

struct Frame {
    width: u32,
    height: u32,
}

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
    fn new(window: &Window) -> Result<Game, ()> {
        let (y, x) = window.get_max_yx();
        if x < 10 || y < 10 {
            return Err(())
        }

        let frame = Frame {
            width: (x - 3) as u32,
            height: (y - 2) as u32,
        };
        let ball = Ball {
            x: 2,
            y: 4,
            vertical_direction: VerticalDirection::Up,
            horizontal_direction: HorizontalDirection::Left,
        };
        Ok(Game { frame, ball })
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }

    fn draw(&self, window: &Window) {
        window.border('|', '|', '-', '-', '+', '+', '+', '+');
        window.mvaddch(self.ball.y as i32 + 1, self.ball.x as i32 + 1, 'o');
        window.mv(0, 0);
    }
}

fn main() -> Result<(), ()> {
    let window = initscr();

    let mut game = Game::new(&window)?;

    window.timeout(33);
    loop {
        window.clear();
        game.draw(&window);
        window.refresh();

        match window.getch() {
            Some(Input::Character('q')) => break,
            Some(Input::KeyResize) => game = Game::new(&window)?,
            _ => game.step(),
        }
    }

    endwin();

    Ok(())
}
