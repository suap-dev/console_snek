use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use std::default;
use std::vec::Vec;

fn main() {
    let mut engine = Engine::from(ConsoleEngine::init_fill(10).unwrap());

    let mut snek = Snek::hatch(10, 10);

    for i in 0..100 {
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

#[derive(Clone)]
struct Pixel {
    l_char: char,
    r_char: char,
    // fg_color: Color,
    bg_color: Color,
    position: Position,
}
impl Pixel {
    fn new(
        l_char: char,
        r_char: char,
        /*fg_color: Color,*/ bg_color: Color,
        position: Position,
    ) -> Self {
        Pixel {
            l_char,
            r_char,
            // fg_color,
            bg_color,
            position,
        }
    }

    fn bg(&mut self, bg_color: Color) {
        self.bg_color = bg_color;
    }

    fn pos(&mut self, position: Position) {
        self.position = position
    }
}

struct Snek {
    body: Vec<Pixel>,
    head: Position,
}
impl Snek {
    fn hatch(x: i32, y: i32) -> Self {
        const HEAD_COLOR: Color = Color::Red;
        const BODY_COLOR: Color = Color::DarkRed;
        const L: char = '[';
        const R: char = ']';

        let mut body: Vec<Pixel> = Vec::new();
        let head = Pixel::new(
            L,
            R,
            // Color::Black,
            HEAD_COLOR,
            Position { x, y },
        );
        let mut body_part = head.clone();
        body_part.bg(BODY_COLOR);

        body.push(head);
        body.push(body_part);
        // body.push(Pixel::from(head, BODY_COLOR));

        Snek { body, head: Position{x,y} }
    }

    fn slither(&mut self, engine: &mut Engine) {
        let mut new = self.body.pop().unwrap();
        self.head.x+=1;
        new.pos(self.head);
        self.body.insert(0, new);

        if engine.is_key_held(KeyCode::Left) {
            
        }
        if engine.is_key_held(KeyCode::Right) {
            
        }
        if engine.is_key_held(KeyCode::Up) {
            
        }
        if engine.is_key_held(KeyCode::Down) {
            
        }
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

    fn draw_pixel(&mut self, pixel: &Pixel) {
        self.set_pxl(pixel.position.x, pixel.position.y, &pixel);
    }

    fn set_pxl(&mut self, mut x: i32, y: i32, pixel: &Pixel) {
        x *= 2;
        let l = console_engine::pixel::pxl_bg(pixel.l_char, pixel.bg_color);
        let r = console_engine::pixel::pxl_bg(pixel.r_char, pixel.bg_color);
        self.c_engine.set_pxl(x, y, l);
        self.c_engine.set_pxl(x + 1, y, r);
    }

    fn update_frame(&mut self) {
        self.c_engine.draw();
        self.c_engine.wait_frame();
        self.c_engine.clear_screen();
    }

    fn is_key_held(&mut self, key: KeyCode) -> bool {
        self.c_engine.is_key_held(key)
    }
}
