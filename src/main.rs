use std::io::{stdout, Write};
use std::time::Duration;
use std::thread;
use crossterm::{
    cursor::{Hide, MoveTo},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    Result,
};

const WIDTH: u16 = 40;
const HEIGHT: u16 = 20;

struct Snake {
    body: Vec<(u16, u16)>,
    direction: Direction,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: vec![(WIDTH / 2, HEIGHT / 2)],
            direction: Direction::Right,
        }
    }

    fn update(&mut self) {
        let mut new_head = self.body[0];
        match self.direction {
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
        }
        self.body.insert(0, new_head);
        self.body.pop();
    }

    fn render(&self) -> Result<()> {
        for &(x, y) in &self.body {
            execute!(stdout(), MoveTo(x, y), SetForegroundColor(Color::Green), Print('■'))?;
        }
        Ok(())
    }
}

struct Game {
    snake: Snake,
    food: (u16, u16),
    score: u32,
}

impl Game {
    fn new() -> Game {
        Game {
            snake: Snake::new(),
            food: (WIDTH / 2 + 5, HEIGHT / 2),
            score: 0,
        }
    }

    fn update(&mut self) {
        self.snake.update();
        let head = self.snake.body[0];
        if head == self.food {
            self.score += 1;
            self.food = (rand::random::<u16>() % WIDTH, rand::random::<u16>() % HEIGHT);
            self.snake.body.push((0, 0));
        }
    }

    fn render(&self) -> Result<()> {
        execute!(
            stdout(),
            Hide,
            Clear(ClearType::All),
            SetBackgroundColor(Color::Black),
            SetForegroundColor(Color::White),
            MoveTo(0, 0),
            Print(format!("Score: {}", self.score)),
            MoveTo(self.food.0, self.food.1),
            SetForegroundColor(Color::Red),
            Print('■'),
        )?;
        self.snake.render()?;
        stdout().flush()?;
        Ok(())
    }
}

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut game = Game::new();

    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => game.snake.direction = Direction::Up,
                    KeyCode::Down => game.snake.direction = Direction::Down,
                    KeyCode::Left => game.snake.direction = Direction::Left,
                    KeyCode::Right => game.snake.direction = Direction::Right,
                    _ => {}
                }
            }
        }

        game.update();
        game.render()?;
        thread::sleep(Duration::from_millis(100));
    }

    execute!(stdout(), ResetColor)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
