extern crate tcod;

use tcod::console::*;
use tcod::colors::{self, Color};

// scale of 1 square in inches 1.5 feet or 18 inches which is about avg shoulder width
// all size values for the field should be divided by this for scaling
const SQUARE_SCALE: i32 = 18;


// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// size of field in inches

const FIELD_HEIGHT: i32 = 3600;
const END_ZONE_HEIGHT: i32 = 360;
const FIELD_WIDTH: i32 = 1920;
const FIELD_BORDER_WIDTH: i32 = 72;
const COACHING_BOX_HEIGHT: i32 = 1800;
const COACHING_BOX_WIDTH: i32 = 72;
const BENCHES_WIDTH: i32 = 72;
const BENCHES_HEIGHT: i32 = 1800;
const PADDING_WIDTH: i32 = 72;
//empty space between sideline and
const RESTRAINING_LINE_WIDTH: i32 = 72;

//this is in inches because scoring is important



// float values for physics
const BALL_WIDTH: f32 = 7.0;
const BALL_HEIGHT: f32 = 11.5;
const GOAL_POST_WIDTH: f32 = 18.5;

// field numbers are  6 feet wide and 4 feet wide

// size of the map

const MAP_WIDTH: i32 = (FIELD_WIDTH+FIELD_BORDER_WIDTH*2+RESTRAINING_LINE_WIDTH*2
                        +COACHING_BOX_WIDTH*2+BENCHES_WIDTH*2+PADDING_WIDTH*2)/SQUARE_SCALE;
const MAP_HEIGHT: i32 = (FIELD_HEIGHT+FIELD_BORDER_WIDTH*2+PADDING_WIDTH*2)/SQUARE_SCALE;

const CAMERA_WIDTH: i32 = SCREEN_WIDTH * 5 / 6 as i32;
const CAMERA_HEIGHT: i32 = SCREEN_HEIGHT;
const CAMERA_X_BOUND: i32 = CAMERA_WIDTH / 2 as i32;
const CAMERA_Y_BOUND: i32 = CAMERA_HEIGHT / 2 as i32;
const CAMERA_X_START: i32 = CAMERA_X_BOUND+1;
const CAMERA_Y_START: i32 = CAMERA_Y_BOUND+1;
type Map = Vec<Vec<Tile>>;

// 20 frames-per-second maximum
const LIMIT_FPS: i32 = 20;

// All my colors!
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 0 };
const COLOR_GRASS: Color = Color { r: 1, g: 95, b: 28 };
const COLOR_ORANGE: Color = Color { r: 255, g: 119, b: 119};

// A camera object to move the players view. pos variables set it's place in the screen
#[derive(Debug)]
struct Camera {
    x: i32,
    y: i32,
    tlx: i32,
    tly: i32,
}
impl Camera {
    pub fn new() -> Self{
        Camera{
            x: CAMERA_X_START,
            y: CAMERA_Y_START,
            tlx: (CAMERA_X_START - CAMERA_X_BOUND) as i32,
            tly: (CAMERA_Y_START - CAMERA_Y_BOUND) as i32,
        }
    }
    pub fn move_by(&mut self, dx: i32, dy: i32){
        if dx != 0 {
            if self.x - CAMERA_X_BOUND + dx <= 0 {
                    self.x = CAMERA_X_BOUND;
                    self.tlx = 0;
                }
            else if self.x + dx + CAMERA_X_BOUND >=  MAP_WIDTH {
                    self.x = MAP_WIDTH - CAMERA_X_BOUND;
                    self.tlx = MAP_WIDTH - CAMERA_WIDTH;
                }
            else {
                self.x += dx;
                self.tlx += dx;
            }
        }
        if dy != 0{
            if self.y - CAMERA_Y_BOUND + dy <= 0 {
                self.y = CAMERA_Y_BOUND;
                self.tly = 0;
            }
            else if self.y + dy + CAMERA_Y_BOUND >= MAP_HEIGHT {
                self.y = MAP_HEIGHT - CAMERA_Y_BOUND;
                self.tly = MAP_HEIGHT - CAMERA_HEIGHT;
            }
             else {

                self.y += dy;
                self.tly += dy;
            }
        }
    }
}
/// A tile of the map and its properties
#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
    char: char,
    color: Color,
}

impl Tile {
    pub fn empty() -> Self {
        Tile{blocked: false, block_sight: false, char: ' ', color: colors::BLACK}
    }

    pub fn wall() -> Self {
        Tile{blocked: true, block_sight: true, char: ' ', color: colors::DARKER_BLUE}
    }
    pub fn grass() -> Self {
        Tile{blocked: false, block_sight: false, char: ' ', color: colors::DARKEST_GREEN}
    }
    pub fn paint() -> Self {
        Tile{blocked: false, block_sight: false, char: ' ', color: colors::WHITE}
    }
}

/// This is a generic object: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen.
#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }



    /// move by the given amount, if the destination is not blocked
    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        if !map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    /// set the color and then draw the character that represents this object at its position
    pub fn draw(&self, con: &mut Console, camera: &Camera) {
        if  self.x >= camera.tlx &&
            self.x <= camera.tlx + CAMERA_WIDTH &&
            self.y >= camera.tly &&
            self.y <= camera.tly + CAMERA_HEIGHT {
                con.set_default_foreground(self.color);
                con.put_char(self.x - camera.tlx, self.y - camera.tly, self.char, BackgroundFlag::None);
            }
        }
    /// Erase the character that represents this object
    pub fn clear(&self, con: &mut Console, camera: &Camera) {
        if  self.x >= camera.tlx &&
            self.x <= camera.tlx + CAMERA_WIDTH &&
            self.y >= camera.tly &&
            self.y <= camera.tly + CAMERA_HEIGHT {
                con.put_char(self.x - camera.tlx, self.y - camera.tly, ' ', BackgroundFlag::None);
            }
        }
    }

<<<<<<< HEAD
=======
}
>>>>>>> 1ce4e58043052e37d38ed5a5b9b90a530d1da9b3

// scales inches to ascii blocks for drawing

fn map_rect(h: i32, w: i32) -> (i32, i32) {

(((h as f32 / SQUARE_SCALE as f32).round().max(1.0) as i32),

((w as f32 / SQUARE_SCALE as f32).round().max(1.0) as i32))

}
// takes a value in inches and scales it for drawing
fn map_point(x: i32, y:i32) -> (i32, i32) {
    ((x as f32 / SQUARE_SCALE as f32).round() as i32,
    (y as f32 / SQUARE_SCALE as f32).round() as i32)
}
fn make_map() -> Map {
    // fill map with grass
    let mut map = vec![vec![Tile::grass(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // fill outer wall
    for x in 0..MAP_WIDTH{
        map[x as usize][0] = Tile::wall();
        map[x as usize][(MAP_HEIGHT-1) as usize] = Tile::wall();
    }
    for y in 0..MAP_HEIGHT {
        map[0][y as usize] = Tile::wall();
        map[(MAP_WIDTH-1) as usize][y as usize] = Tile::wall();
    }
<<<<<<< HEAD
=======
    map[(MAP_WIDTH-1) as usize][(MAP_HEIGHT-1) as usize] = Tile::wall();
>>>>>>> 1ce4e58043052e37d38ed5a5b9b90a530d1da9b3

    //fill field boundary
    let mut x1 = (MAP_WIDTH/2) - (FIELD_WIDTH/2 + FIELD_BORDER_WIDTH)/SQUARE_SCALE;
    let mut y1 = (MAP_HEIGHT/2) - (FIELD_HEIGHT/2 + FIELD_BORDER_WIDTH)/SQUARE_SCALE;
    let mut x2 = ((MAP_WIDTH/2) + (FIELD_WIDTH/2 + FIELD_BORDER_WIDTH)/SQUARE_SCALE)-1;
    let mut y2 = ((MAP_HEIGHT/2) + (FIELD_HEIGHT/2 + FIELD_BORDER_WIDTH)/SQUARE_SCALE)-1;
    for x in x1..x2{
        for y in y1..y2{
            map[x as usize][y as usize] = Tile::paint();
        }
    }
    //fill grass on the field
    x1 = (MAP_WIDTH/2)-(FIELD_WIDTH)/2/SQUARE_SCALE;
    y1 = (MAP_HEIGHT/2) - (FIELD_HEIGHT)/2/SQUARE_SCALE;
    x2 = ((MAP_WIDTH/2)+(FIELD_WIDTH)/2/SQUARE_SCALE)-1;
    y2 = ((MAP_HEIGHT/2)+(FIELD_HEIGHT)/2/SQUARE_SCALE)-1;
    for x in x1..x2{
        for y in y1..y2{
            map[x as usize][y as usize] = Tile::grass();
        }
    }
    //add 10 yard markers
    for y in (y1..y2).step_by(10 as usize){
        for x in x1..x2{
            map[x as usize][y as usize] = Tile::paint();
        }
    }
    //add endzone lines
    for y in (y1+9)..(y1+11){
        for x in x1..x2 {
            map[x as usize][y as usize] = Tile::paint();
        }
    }
    //add the yard hashes
    for y in (y1+12..y2-12).step_by(2) {
        for x in x1+2..x1+7 {
            map[x as usize][y as usize] = Tile::paint();
        }
        for x in ((x2-x1)/2-(9.25/SQUARE_SCALE as f32) as i32)..((x2-x1)/2-(4.25/SQUARE_SCALE as f32) as i32) {
            map[x as usize][y as usize] = Tile::paint();
        }
        for x in ((x2-x1)/2+(5.25/SQUARE_SCALE as f32) as i32)..((x2-x1)/2+(9.25/SQUARE_SCALE as f32) as i32) {
            map[x as usize][y as usize] = Tile::paint();
        }
        for x in x2-7..x2-2 {
            map[x as usize][y as usize] = Tile::paint();
        }
    }
    map
}

fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &Map, camera: &Camera) {
    // go through all tiles, and set their background color
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            con.set_char_background(x, y, colors::LIGHT_GREY, BackgroundFlag::Set);
        }
    }

    //draw the map in the camera
    for y in 0..CAMERA_HEIGHT {
        for x in 0..CAMERA_WIDTH {
            let a_tile = map[(x + camera.tlx) as usize][(y + camera.tly) as usize];
                con.set_char_background(x, y, a_tile.color, BackgroundFlag::Set);
        }
    }

    // draw all objects in the list
    for object in objects {
        object.draw(con, &camera);
    }

    // blit the contents of "con" to the root console
    blit(con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), root, (0, 0), 1.0, 1.0);
}

fn handle_keys(root: &mut Root, player: &mut Object, map: &Map, camera: &mut Camera) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Enter, alt: true, .. } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,  // exit game

        // movement keys
        Key { code: Up, .. } => player.move_by(0, -1, map),
        Key { code: Down, .. } => player.move_by(0, 1, map),
        Key { code: Left, .. } => player.move_by(-1, 0, map),
        Key { code: Right, .. } => player.move_by(1, 0, map),
        Key { printable: 'w', .. } => camera.move_by(0, -1),
        Key { printable: 's', .. } => camera.move_by(0, 1),
        Key { printable: 'a', .. } => camera.move_by(-1, 0),
        Key { printable: 'd', .. } => camera.move_by(1, 0),
        _ => {},
    }

    false
}

fn main() {
    let mut root = Root::initializer()
        .font("terminal16x16_gs_ro.png", FontLayout::AsciiInRow)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();
    tcod::system::set_fps(LIMIT_FPS);
    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    // create object representing the player
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::DARKER_GREEN);

    // create an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', Color{r:255, g:119, b:119});

    // the list of objects
    let mut objects = [player, npc];

    // generate map (at this point it's not drawn to the screen)
    let map = make_map();
    let mut camera = Camera::new();

    while !root.window_closed() {
        // render the screen
        render_all(&mut root, &mut con, &objects, &map, &mut camera);

        root.flush();

        // erase all objects at their old locations, before they move
        for object in &objects {
            object.clear(&mut con, &camera)
        }

        // handle keys and exit game if needed
        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player, &map, &mut camera);
        if exit {
            break
        }
    }
}
