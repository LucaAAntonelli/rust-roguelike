use std::collections::HashMap;

use quicksilver::{
    geom::Vector,
    graphics::{Color, Image, VectorFont},
    run, Graphics, Input, Result, Settings, Window,
};

struct Game {
    title: Image,
    mononoki_font_info: Image,
    square_font_info: Image,
    map_size: Vector,
    map: Vec<Tile>,
    entities: Vec<Entity>,
    player_id: usize,
    tileset: HashMap<char, Image>,
    tile_size_px: Vector,
}

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    pos: Vector,
    glyph: char,
    color: Color,
}
#[derive(Clone, Debug, PartialEq)]
struct Entity {
    pos: Vector,
    glyph: char,
    color: Color,
    hp: i32,
    max_hp: i32,
}

fn generate_map(size: Vector) -> Vec<Tile> {
    let width = size.x as usize;
    let height = size.y as usize;
    let mut map = Vec::with_capacity(width * height);
    for x in 0..width {
        for y in 0..height {
            let mut tile = Tile {
                pos: Vector::new(x as f32, y as f32),
                glyph: '.',
                color: Color::BLACK,
            };

            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                tile.glyph = '#';
            };
            map.push(tile);
        }
    }
    map
}

fn generate_entities() -> Vec<Entity> {
    vec![
        Entity {
            pos: Vector::new(9.0, 6.0),
            glyph: 'g',
            color: Color::RED,
            hp: 1,
            max_hp: 1,
        },
        Entity {
            pos: Vector::new(2.0, 4.0),
            glyph: 'g',
            color: Color::RED,
            hp: 1,
            max_hp: 1,
        },
        Entity {
            pos: Vector::new(7.0, 5.0),
            glyph: '%',
            color: Color::PURPLE,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(4.0, 8.0),
            glyph: '%',
            color: Color::PURPLE,
            hp: 0,
            max_hp: 0,
        },
    ]
}

// impl State for Game {
//     fn new() -> Result<Self> {
//         let font_mononoki = "mononoki-Regular.ttf";
//         let font_square = "square.ttf";

//         let title = Asset::new(Font::load(font_mononoki).and_then(|font| {
//             font.render("Quicksilver Roguelike", &FontStyle::new(72.0, Color::BLACK))
//         }));

//         let mononoki_font_info = Asset::new(Font::load(font_mononoki).and_then(|font| {
//             font.render(
//                 "Mononoki font by Matthias Tellen, terms: SIL Open Font License 1.1",
//                 &FontStyle::new(20.0, Color::BLACK),
//             )
//         }));

//         let square_font_info = Asset::new(Font::load(font_square).and_then(|font| {
//             font.render(
//                 "Square font by Wouter Van Oortmerssen, terms: CC BY 3.0",
//                 &FontStyle::new(20.0, Color::BLACK),
//             )
//         }));

//         let map_size = Vector::new(20, 15);
//         let map = generate_map(map_size);
//         let mut entities = generate_entities();

//         let player_id = entities.len();
//         entities.push(Entity {
//             pos: Vector::new(5, 3),
//             glyph: '@',
//             color: Color::BLUE,
//             hp: 3,
//             max_hp: 5,
//         });

//         let game_glyphs = "#@g.%";
//         let tile_size_px = Vector::new(24, 24);
//         let tileset = Asset::new(Font::load(font_square).and_then(move |text| {
//             let tiles = text
//                 .render(game_glyphs, &FontStyle::new(tile_size_px.y, Color::WHITE))
//                 .expect("Could not render the font tileset.");
//             let mut tileset = HashMap::new();
//             for (index, glyph) in game_glyphs.chars().enumerate() {
//                 let pos = (index as i32 * tile_size_px.x as i32, 0);
//                 let tile = tiles.subimage(Rectangle::new(pos, tile_size_px));
//                 tileset.insert(glyph, tile);
//             }
//             Ok(tileset)
//         }));

//         Ok(Self {
//             title,
//             mononoki_font_info,
//             square_font_info,
//             map_size,
//             map,
//             entities,
//             player_id,
//             tileset,
//             tile_size_px,
//         })
//     }

//     fn update(&mut self, window: &mut Window) -> Result<()> {
//         let player = &mut self.entities[self.player_id];
//         if window.keyboard()[Key::Left].is_down() {
//             player.pos.x -= 1.0;
//         }
//         if window.keyboard()[Key::Right].is_down() {
//             player.pos.x += 1.0;
//         }
//         if window.keyboard()[Key::Up].is_down() {
//             player.pos.y -= 1.0;
//         }
//         if window.keyboard()[Key::Down].is_down() {
//             player.pos.y += 1.0;
//         }
//         if window.keyboard()[Key::Escape].is_down() {
//             window.close();
//         }
//         Ok(())
//     }

//     fn draw(&mut self, window: &mut Window) -> Result<()> {
//         window.clear(Color::WHITE)?;

//         self.title.execute(|image| {
//             window.draw(
//                 &image
//                     .area()
//                     .with_center((window.screen_size().x as i32 / 2, 40)),
//                 // Img(&image),
//             );
//             Ok(())
//         })?;

//         self.mononoki_font_info.execute(|image| {
//             window.draw(
//                 &image
//                     .area()
//                     .translate((2, window.screen_size().y as i32 - 60)),
//                 // Img(&image),
//             );
//             Ok(())
//         })?;

//         self.square_font_info.execute(|image| {
//             window.draw(
//                 &image
//                     .area()
//                     .translate((2, window.screen_size().y as i32 - 30)),
//                 // Img(&image),
//             );
//             Ok(())
//         })?;

//         let tile_size_px = self.tile_size_px;
//         let offset_px = Vector::new(50, 120);
//         let (tileset, map) = (&mut self.tileset, &self.map);
//         tileset.execute(|tileset| {
//             for tile in map.iter() {
//                 if let Some(image) = tileset.get(&tile.glyph) {
//                     let pos_px = tile.pos.times(tile_size_px);
//                     window.draw(
//                         &Rectangle::new(pos_px + offset_px, image.area().size()),
//                         // Blended(&image, tile.color),
//                     );
//                 }
//             }
//             Ok(())
//         })?;

//         let (tileset, entities) = (&mut self.tileset, &self.entities);
//         tileset.execute(|tileset| {
//             for entity in entities.iter() {
//                 if let Some(image) = tileset.get(&entity.glyph) {
//                     let pos_px = offset_px + entity.pos.times(tile_size_px);
//                     window.draw(
//                         &Rectangle::new(pos_px, image.area().size()),
//                         // Blended(&image, entity.color),
//                     );
//                 }
//             }
//             Ok(())
//         })?;

//         let player = &self.entities[self.player_id];
//         let full_health_width_px = 100.0;
//         let current_health_width_px =
//             (player.hp as f32 / player.max_hp as f32) * full_health_width_px;

//         let map_size_px = self.map_size.times(tile_size_px);
//         let health_bar_pos_px = offset_px + Vector::new(map_size_px.x, 0.0);

//         // window.draw(
//         //     &Rectangle::new(health_bar_pos_px, (full_health_width_px, tile_size_px.y)),
//         //     Col(Color::RED.with_alpha(0.5)),
//         // );
//         // window.draw(
//         //     &Rectangle::new(health_bar_pos_px, (current_health_width_px, tile_size_px.y)),
//         //     Col(Color::RED),
//         // );

//         Ok(())
//     }
// }

fn main() {
    run(
        Settings {
            title: "Rust Roguelike",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let mononoki = VectorFont::load("mononoki-Regular.ttf").await?;
    let mut font = mononoki.to_renderer(&gfx, 16.0)?;
    gfx.clear(Color::WHITE);

    font.draw(
        &mut gfx,
        "Hello world!",
        Color::BLACK,
        Vector::new(100.0, 100.0),
    )?;
    gfx.present(&window)?;

    loop {
        while let Some(_) = input.next_event().await {}
    }
}
