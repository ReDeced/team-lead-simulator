use macroquad::prelude::*;

mod types;

use crate::types::load_developers;


#[macroquad::main("cocal")]
async fn main() {
    const FONT_BYTES: &[u8] = include_bytes!("../assets/verdana-font.ttf");
    let font = load_ttf_font_from_bytes(FONT_BYTES).expect("Не удалось загрузить шрифт");

    let developers = load_developers();

    loop {
        clear_background(DARKGRAY);
        for (i, developer) in developers.iter().enumerate() {
            draw_text_ex(format!("{} {} лет {:?} {:?}", developer.name, developer.age, developer.position, developer.grade),
            20.0, 40.0 + 30.0 * (i as f32),
            TextParams { font: Some(&font), font_size: 20, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE });
        }
        next_frame().await;
    }
}

