use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::vec::Vec;

fn main() {
    let mut engine = Engine::from(ConsoleEngine::init_fill(8).unwrap());

    let mut map = Map::from_coords(Position { x: 2, y: 2 }, Position { x: 20, y: 10 });
    let mut snek = Snek::hatch(&map, 5, 10);

    loop {
        if !snek.slither(&mut map, &mut engine) { break; }

        map.draw(&mut engine);
        snek.draw(&mut engine);

        engine.update_frame();
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, Copy)]
struct Segment {
    l_char: char,
    r_char: char,
    color: Color,
    position: Position,
}
const L: char = '[';
const R: char = ']';
impl Segment {
    fn new_at(position: Position, color: Color) -> Self {
        Segment {
            l_char: L,
            r_char: R,
            // fg_color,
            color: color,
            position,
        }
    }
    fn new() -> Self {
        Segment::new_at(Position { x: 0, y: 0 }, Color::White)
    }

    fn at(&self, other: &Segment) -> bool {
        self.position == other.position
    }
}

const HEAD_COLOR: Color = Color::DarkRed;
const BODY_COLOR: Color = Color::Red;
struct Snek {
    body: Vec<Segment>,
    // head: Position,
    direction: Direction,
    growing: bool,
}
const INITIAL_LENGTH: i32 = 3;
impl Snek {
    fn hatch(map: &Map, x: i32, y: i32) -> Self {
        let mut body: Vec<Segment> = Vec::new();

        for _ in 0..INITIAL_LENGTH {
            let body_part = Segment::new_at(
                Position {
                    x: map.center_x(),
                    y: map.center_y(),
                },
                BODY_COLOR,
            );
            body.push(body_part);
        }

        body[0].color = HEAD_COLOR;

        Snek {
            body,
            direction: Direction::Right,
            growing: false,
        }
    }

    fn slither(&mut self, map: &mut Map, engine: &mut Engine) -> bool{
        // current head becomes body part
        self.body[0].color = BODY_COLOR;

        // we need to determine if snake changes direction
        if engine.is_key_pressed(KeyCode::Left) && self.direction != Direction::Right {
            self.direction = Direction::Left;
        }
        if engine.is_key_pressed(KeyCode::Right) && self.direction != Direction::Left {
            self.direction = Direction::Right;
        }
        if engine.is_key_pressed(KeyCode::Up) && self.direction != Direction::Down {
            self.direction = Direction::Up;
        }
        if engine.is_key_pressed(KeyCode::Down) && self.direction != Direction::Up {
            self.direction = Direction::Down;
        }
        // or is growing
        if self.munched(map) {
            self.grow();
        }

        // new head is being created
        let mut new_head = match self.growing {
            false => self.body.pop().unwrap(),
            true => {
                self.growing = false;
                Segment::new()
            }
        };
        new_head.color = HEAD_COLOR;
        new_head.position = self.body[0].position;

        // and set the new head accordingly
        match self.direction {
            Direction::Up => new_head.position.y -= 1,
            Direction::Down => new_head.position.y += 1,
            Direction::Right => new_head.position.x += 1,
            Direction::Left => new_head.position.x -= 1,
        }

        // done
        self.body.insert(0, new_head);
        
        !(self.hit_wall(map) || self.bit_tail())
    }

    fn munched(&self, map: &mut Map) -> bool {
        if self.body[0].position == map.nom_position() {
            map.new_nom();
            true
        } else {
            false
        }
    }
    fn hit_wall(&self, map: &Map) -> bool {
        if self.body[0].position.x == map.min_x()
            || self.body[0].position.x == map.max_x()
            || self.body[0].position.y == map.min_y()
            || self.body[0].position.y == map.max_y()
        {
            true
        } else {
            false
        }
    }
    fn bit_tail(&self) -> bool {
        for tail_part in self.body[1..].into_iter() {
            if self.body[0].position == tail_part.position {
                return true;
            }
        }
        false
    }

    fn grow(&mut self) {
        self.growing = true;
    }

    fn draw(&self, engine: &mut Engine) {
        for body_part in &self.body {
            engine.draw_segment(body_part);
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
    
    fn close(&mut self) {
    }

    fn draw_segment(&mut self, segment: &Segment) {
        self.set_pxl(segment.position.x, segment.position.y, &segment);
    }

    fn draw_border(&mut self, left: i32, right: i32, top: i32, bottom: i32, color: Color) {
        let wall = Segment {
            l_char: ' ',
            r_char: ' ',
            color,
            position: Position { x: left, y: top },
        };

        for x in left..=right {
            self.set_pxl(x, top, &wall);
            self.set_pxl(x, bottom, &wall);
        }
        for y in top + 1..=bottom - 1 {
            self.set_pxl(left, y, &wall);
            self.set_pxl(right, y, &wall);
        }
    }

    fn set_pxl(&mut self, mut x: i32, y: i32, pixel: &Segment) {
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

const NOM_COLOR: Color = Color::Green;
struct NomSpawner {
    rng: ThreadRng,
}
impl NomSpawner {
    fn spawn_between(&mut self, top_left_corner: Position, bot_right_corner: Position) -> Segment {
        Segment::new_at(
            Position {
                x: self.rng.gen_range(top_left_corner.x+1..bot_right_corner.x),
                y: self.rng.gen_range(top_left_corner.y+1..bot_right_corner.y),
            },
            NOM_COLOR,
        )
    }

    fn spawn(&mut self, map: &Map) -> Position {
        let x: i32 = self.rng.gen_range(map.min_x()..map.max_x());
        let y: i32 = self.rng.gen_range(map.min_y()..map.max_y());

        Position { x, y }
    }
}

struct Map {
    top_left_corner: Position,
    bot_right_corner: Position,
    nom_spawner: NomSpawner,
    nom: Segment,
}
impl Map {
    fn from_coords(top_left_corner: Position, bot_right_corner: Position) -> Self {
        let mut nom_spawner = NomSpawner {
            rng: rand::thread_rng(),
        };
        let nom = nom_spawner.spawn_between(top_left_corner, bot_right_corner);
        Map {
            top_left_corner,
            bot_right_corner,
            nom_spawner,
            nom,
        }
    }

    fn min_x(&self) -> i32 {
        self.top_left_corner.x
    }
    fn max_x(&self) -> i32 {
        self.bot_right_corner.x
    }
    fn min_y(&self) -> i32 {
        self.top_left_corner.y
    }
    fn max_y(&self) -> i32 {
        self.bot_right_corner.y
    }
    fn center_x(&self) -> i32 {
        (self.min_x() + self.max_x()) / 2
    }
    fn center_y(&self) -> i32 {
        (self.min_y() + self.max_y()) / 2
    }

    fn new_nom(&mut self) {
        self.nom = self
            .nom_spawner
            .spawn_between(self.top_left_corner, self.bot_right_corner);
    }

    // we can later generate multiple noms
    // todo: do it with Vec<...>
    fn nom_position(&self) -> Position {
        return self.nom.position;
    }

    fn draw(&self, engine: &mut Engine) {
        // fn draw(&self, engine: &mut Engine) {
        //     for body_part in &self.body {
        //         engine.draw(body_part);
        //     }
        // }
        engine.draw_border(
            self.min_x(),
            self.max_x(),
            self.min_y(),
            self.max_y(),
            Color::White,
        );
        engine.draw_segment(&self.nom);

        // while(wall.position.x != self.min_x() && wall.position.y != self.min_y()) {
        //     for
        // }
    }
}
