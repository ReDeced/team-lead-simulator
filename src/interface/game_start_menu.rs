use macroquad::{color::{BLACK, WHITE}, input::{MouseButton, is_mouse_button_pressed, mouse_position}, math::{Rect, Vec2, Vec4}, miniquad::TextureParams, shapes::{draw_rectangle, draw_rectangle_lines}, text::{Font, TextParams, draw_text_ex, measure_text}};

use crate::{interface::{difficulty_chose_menu::DifficultyChooseMenu, employees_rotation_menu::EmployeesRotationMenu}, types::{Difficulty, Employee}};


#[derive(PartialEq)]
enum GameStartMenuState {
    None,
    ChosingDifficulty,
    ChosingEmployees
}


pub struct GameStartMenu {
    pos: Vec2,
    size: Vec2,
    state: GameStartMenuState,
    
    chosen_employees: Vec<Employee>,
    chosen_difficulty: Difficulty,

    employees_rotation_menu: EmployeesRotationMenu,
    difficulty_chose_menu: DifficultyChooseMenu
}


impl GameStartMenu {
    pub fn new(pos: Vec2, size: Vec2, max_difficulty: Difficulty) -> Self {
        let employees_rotation_menu = EmployeesRotationMenu::new(pos + pos / 5.0, size / 5.0 * 3.0);
        let difficulty_chose_menu = DifficultyChooseMenu::new(
            pos + pos / 5.0, size / 5.0 * 3.0, max_difficulty);
        Self { pos, size, state: GameStartMenuState::None, chosen_employees: Vec::new(),
            chosen_difficulty: Difficulty::Tutorial, employees_rotation_menu, difficulty_chose_menu }
    }

    pub fn draw(&mut self, font: &Font) -> Option<(Difficulty, Vec<Employee>)> {
        let font_size = 24;

        let x = self.pos.x;
        let y = self.pos.y;
        let w = self.size.x;
        let h = self.size.y;
        
        let mouse_pos = mouse_position();
        
        draw_rectangle(x, y, w, h, BLACK);
        draw_rectangle_lines(x, y, w, h, 5.0, WHITE);
        
        let button_1= Rect {
            x: x + w / 5.0,
            y: y + h / 5.0,
            w: w / 5.0 * 3.0,
            h: h / 5.0
        };
        let text_surface_1 = measure_text(self.chosen_difficulty.to_string(), Some(font), font_size, 1.0);

        let button_2 = Rect {
            x: x + w / 5.0,
            y: y + w / 5.0 * 3.0,
            w: w /5.0 * 3.0,
            h: h / 5.0
        };
        let text_surface_2 = measure_text("Выбрать начальную команду", Some(font), font_size, 1.0);

        draw_rectangle(button_1.x, button_1.y, button_1.w, button_1.h, BLACK);
        draw_rectangle_lines(button_1.x, button_1.y, button_1.w, button_1.h, 5.0, WHITE);

        draw_text_ex(self.chosen_difficulty.to_string(),
        button_1.x + (button_1.w - text_surface_1.width) / 2.0,
        button_1.y + button_1.h / 2.0,
        TextParams { font: Some(font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE }
        );

        draw_rectangle(button_2.x, button_2.y, button_2.w, button_2.h, BLACK);
        draw_rectangle_lines(button_2.x, button_2.y, button_2.w, button_2.h, 5.0, WHITE);
        
        draw_text_ex(self.chosen_difficulty.to_string(),
        button_2.x + (button_2.w - text_surface_2.width) / 2.0,
        button_2.y + button_2.h / 2.0,
        TextParams { font: Some(font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE }
        );
        
        if button_1.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
            if is_mouse_button_pressed(MouseButton::Left) {
                self.state = GameStartMenuState::ChosingDifficulty;
            }
        } 
        else if button_2.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
            if is_mouse_button_pressed(MouseButton::Left) {
                self.state = GameStartMenuState::ChosingEmployees;
            }
        }

        if self.state == GameStartMenuState::None {
            // TODO: Сделать меню с двумя кнопками, меняющими self.state
        }
        else if self.state == GameStartMenuState::ChosingDifficulty {
            let result = self.difficulty_chose_menu.draw(font);
            if let Some(chosen_difficulty) = result {
                self.chosen_difficulty = chosen_difficulty;
                self.state = GameStartMenuState::None;
            }
        }
        else if self.state == GameStartMenuState::ChosingEmployees {
            let result = self.employees_rotation_menu.draw(font);
            if let Some(chosen_employees) = result {
                self.chosen_employees = chosen_employees;
                self.state = GameStartMenuState::None;
            }
        }

        None
    }
}
