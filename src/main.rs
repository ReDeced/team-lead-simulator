use macroquad::{prelude::*};

mod types;
mod interface;

use crate::{interface::employees_rotation_menu::EmployeesRotationMenu, types::{Difficulty, Game}};


#[macroquad::main("cocal")]
async fn main() {
    const FONT_BYTES: &[u8] = include_bytes!("../assets/verdana-font.ttf");
    let font = load_ttf_font_from_bytes(FONT_BYTES).expect("Не удалось загрузить шрифт");

    macroquad::text::set_default_font(font.clone());
    
    let game = Game::new(Difficulty::Tutorial);
    
    let employees_rotation_menu = EmployeesRotationMenu::new();

    loop {
        clear_background(DARKGRAY);
       
        let mouse_pos = mouse_position();

        let chosen_employee = employees_rotation_menu.menu(&font, mouse_pos);

        if chosen_employee.is_some() {
            
        }
        
        next_frame().await;
    }
}

