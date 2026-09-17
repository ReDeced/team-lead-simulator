use macroquad::prelude::*;

mod types;
mod interface;

use crate::{interface::employees_rotation_menu::EmployeesRotationMenu, types::{Difficulty, Game}};


#[macroquad::main("cocal")]
async fn main() {
    const FONT_BYTES: &[u8] = include_bytes!("../assets/verdana-font.ttf");
    let font = load_ttf_font_from_bytes(FONT_BYTES).expect("Не удалось загрузить шрифт");

    macroquad::text::set_default_font(font.clone());
    
    let game = Game::new(Difficulty::Tutorial);
    
    let screen_w = screen_width();
    let screen_h = screen_height();

    let screen_size = Vec2::new(screen_w, screen_h);

    let employees_rotation_menu = EmployeesRotationMenu::new(Vec2::ZERO, screen_size);

    loop {
        clear_background(DARKGRAY);
       
        let chosen_employee = employees_rotation_menu.draw(&font);

        if chosen_employee.is_some() {
            
        }
        
        next_frame().await;
    }
}

