use macroquad::{color::{BLACK, DARKGRAY, WHITE}, input::{MouseButton, is_mouse_button_pressed, mouse_position}, math::Vec2, shapes::{draw_rectangle, draw_rectangle_lines}, text::{Font, TextParams, draw_text_ex, measure_text}};
use strum::{EnumCount, IntoEnumIterator};

use crate::types::Difficulty;


pub struct DifficultyChooseMenu {
    pos: Vec2,
    size: Vec2,
    
    max_difficulty: Difficulty,
    chosen_difficulty: Difficulty
}


impl DifficultyChooseMenu {
    pub fn new(pos: Vec2, size: Vec2, max_difficulty: Difficulty) -> Self {
        Self { pos, size, max_difficulty, chosen_difficulty: Difficulty::Tutorial }
    }

    pub fn draw(&mut self, font: &Font) -> Option<Difficulty> {
        let x = self.pos.x;
        let y = self.pos.y;
        let w = self.size.x;
        let h = self.size.y;
        
        let mouse_pos = mouse_position();
        let mouse_x = mouse_pos.0;
        let mouse_y = mouse_pos.1;

        draw_rectangle(x, y, w, h, BLACK);
        draw_rectangle_lines(x, y, w, h, 5.0, WHITE);

        for (i, difficulty) in Difficulty::iter().enumerate() {
            let font_size = 24;

            let indent = 10.0;
            
            let w = w - indent * 2.0;
            let h = (h - indent * 2.0) / Difficulty::COUNT as f32;
            let x = x + indent;
            let y = y + indent + h * i as f32;

            let text_surface = measure_text(difficulty.to_string(), Some(font), font_size, 1.0);


            let mut text_color = WHITE;
            let mut fill_color = BLACK;

            if mouse_x >= x && mouse_x <= x + w && mouse_y >= y && mouse_y <= y + h {
                fill_color = DARKGRAY;

                if is_mouse_button_pressed(MouseButton::Left) {
                    self.chosen_difficulty = difficulty;
                    return Some(difficulty);
                }
            }
            if difficulty == self.chosen_difficulty {
                fill_color = WHITE;
                text_color = BLACK;
            }
            
            draw_rectangle(x, y, w, h, fill_color);
            draw_rectangle_lines(x, y, w, h, 5.0, WHITE);

            draw_text_ex(difficulty.to_string(), x + (w - text_surface.width) / 2.0, y + (h - text_surface.height) / 2.0,
            TextParams { font: Some(font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: text_color });
        }
        None
    }
}

