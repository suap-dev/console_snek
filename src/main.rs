use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use std::default;
use std::vec::Vec;

fn main() {
    let mut engine = Engine::from(ConsoleEngine::init_fill(8).unwrap());

    let mut snek = Snek::hatch(10, 10);

    for _ in 0..100 {
        snek.slither(&mut engine);
        snek.draw(&mut engine);
        engine.update_frame();
    }
}

#[derive(Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone)]
struct BodyPart {
    l_char: char,
    r_char: char,
    // fg_color: Color,
    color: Color,
    position: Position,
}
const L: char = '[';
const R: char = ']';
const HEAD_COLOR: Color = Color::DarkRed;
const BODY_COLOR: Color = Color::Red;
impl BodyPart {
    fn new(position: Position) -> Self {
        BodyPart {
            l_char: L,
            r_char: R,
            // fg_color,
            color: BODY_COLOR,
            position,
        }
    }

    fn color(&mut self, color: Color) {
        self.color = color;
    }

    fn pos(&mut self, position: Position) {
        self.position = position
    }
}

struct Snek {
    body: Vec<BodyPart>,
    head: Position,
    direction: Direction,
}
const INITIAL_LENGTH: i32 = 3;
impl Snek {
    fn hatch(x: i32, y: i32) -> Self {
        let mut body: Vec<BodyPart> = Vec::new();

        for _ in 0..INITIAL_LENGTH {
            let body_part = BodyPart::new(Position { x, y });
            body.push(body_part);
        }

        body[0].color(HEAD_COLOR);

        Snek {
            body,
            head: Position { x, y },
            direction: Direction::Right,
        }
    }

    fn slither(&mut self, engine: &mut Engine) {
        let mut new = self.body.pop().unwrap();

        if engine.is_key_pressed(KeyCode::Left) {
            self.direction = Direction::Left;
        }
        if engine.is_key_pressed(KeyCode::Right) {
            self.direction = Direction::Right;
        }
        if engine.is_key_pressed(KeyCode::Up) {
            self.direction = Direction::Up;
        }
        if engine.is_key_pressed(KeyCode::Down) {
            self.direction = Direction::Down;
        }

        match self.direction {
            Direction::Up => self.head.y -= 1,
            Direction::Down => self.head.y += 1,
            Direction::Right => self.head.x += 1,
            Direction::Left => self.head.x -= 1,
        }

        new.pos(self.head);
        new.color(HEAD_COLOR);
        for part in &mut self.body {
            part.color(BODY_COLOR)
        }
        self.body.insert(0, new);
    }

    fn draw(&self, engine: &mut Engine) {
        for body_part in &self.body {
            engine.draw_pixel(body_part);
        }
    }
}

struct Engine {
    c_engine: ConsoleEngine,
}
impl Engine {
    fn from(c_engine: ConsoleEngine) -> Self {
        Engine { c_engine }
    }

    fn draw_pixel(&mut self, pixel: &BodyPart) {
        self.set_pxl(pixel.position.x, pixel.position.y, &pixel);
    }

    fn set_pxl(&mut self, mut x: i32, y: i32, pixel: &BodyPart) {
        x *= 2;
        let l = console_engine::pixel::pxl_bg(pixel.l_char, pixel.color);
        let r = console_engine::pixel::pxl_bg(pixel.r_char, pixel.color);
        self.c_engine.set_pxl(x, y, l);
        self.c_engine.set_pxl(x + 1, y, r);
    }

    fn update_frame(&mut self) {
        self.c_engine.draw();
        self.c_engine.wait_frame();
        self.c_engine.clear_screen();
    }

    fn is_key_pressed(&mut self, key: KeyCode) -> bool {
        self.c_engine.is_key_pressed(key)
    }
}
