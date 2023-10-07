use quicksilver::{
    geom::Vector,
    graphics::{Color, VectorFont},
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            title: "Rust Roguelike",
            size: Vector::new(800.0, 600.0),
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let mononoki = VectorFont::load("mononoki-Regular.ttf").await?;
    let square = VectorFont::load("square.ttf").await?;
    let mut title = mononoki.to_renderer(&gfx, 72.0)?;
    let mut mononoki_font_info = mononoki.to_renderer(&gfx, 20.0)?;
    let mut square_font_info = square.to_renderer(&gfx, 20.0)?;
    gfx.clear(Color::WHITE);

    title.draw(
        &mut gfx,
        "Quicksilver Roguelike",
        Color::BLACK,
        Vector::new(window.size().x / 4.0, 40.0),
    )?;

    mononoki_font_info.draw(
        &mut gfx,
        "Mononoki font by Matthias Tellen, terms: SIL Open Font License 1.1",
        Color::BLACK,
        Vector::new(10.0, window.size().y - 60.0),
    )?;

    square_font_info.draw(
        &mut gfx,
        "Square font by Wouter Van Oortmerssen, terms: CC BY 3.0",
        Color::BLACK,
        Vector::new(10.0, window.size().y - 30.0),
    )?;

    gfx.present(&window)?;

    loop {
        while let Some(_) = input.next_event().await {}
    }
}
