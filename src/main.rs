use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::vec::Vec;

fn main() {
    let mut engine = Engine::from(ConsoleEngine::init_fill(8).unwrap());

    let mut map = Map::from_coords(Position { x: 2, y: 2 }, Position { x: 10, y: 10 });
    let mut snek = Snek::hatch(&map, 10, 10);

    for _ in 0..100 {
        snek.slither(&mut map, &mut engine);

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

#[derive(Clone)]
struct Segment {
    l_char: char,
    r_char: char,
    color: Color,
    position: Position,
}
const L: char = '[';
const R: char = ']';
impl Segment {
    fn at(position: Position, color: Color) -> Self {
        Segment {
            l_char: L,
            r_char: R,
            // fg_color,
            color: color,
            position,
        }
    }
    fn new() -> Self {
        Segment::at(Position { x: 0, y: 0 }, Color::White)
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
            let body_part = Segment::at(
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

    fn slither(&mut self, map: &mut Map, engine: &mut Engine) {
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
    }

    fn munched(&self, map: &mut Map) -> bool {
        if self.body[0].position == map.nom_position() {
            map.new_nom();
            true
        } else {
            false
        }
    }

    fn grow(&mut self) {
        self.growing = true;
    }

    fn draw(&self, engine: &mut Engine) {
        for body_part in &self.body {
            engine.draw(body_part);
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

    fn draw(&mut self, segment: &Segment) {
        self.set_pxl(segment.position.x, segment.position.y, &segment);
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
        Segment::at(
            Position {
                x: self.rng.gen_range(top_left_corner.x..bot_right_corner.x),
                y: self.rng.gen_range(top_left_corner.y..bot_right_corner.y),
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
        engine.draw(&self.nom);
    }
}
