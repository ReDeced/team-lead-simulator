use macroquad::prelude::*;

mod types;
mod interface;

use crate::{interface::game_start_menu::GameStartMenu, types::{Difficulty, Game}};


#[macroquad::main("cocal")]
async fn main() {
    const FONT_BYTES: &[u8] = include_bytes!("../assets/verdana-font.ttf");
    let font = load_ttf_font_from_bytes(FONT_BYTES).expect("Не удалось загрузить шрифт");

    macroquad::text::set_default_font(font.clone());
    
    let game = Game::new(Difficulty::Tutorial);
    
    let screen_w = screen_width();
    let screen_h = screen_height();

    let screen_size = Vec2::new(screen_w, screen_h);

    let mut game_start_menu = GameStartMenu::new(screen_size / 5.0, screen_size / 5.0 * 3.0, Difficulty::Hard);

    loop {
        clear_background(DARKGRAY);
        
        game_start_menu.draw(&font);
        
        next_frame().await;
    }
}

