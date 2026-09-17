use macroquad::{color::{BLACK, WHITE}, input::mouse_position, math::Vec2, shapes::{draw_rectangle, draw_rectangle_lines}, text::Font};

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
        let x = self.pos.x;
        let y = self.pos.y;
        let w = self.size.x;
        let h = self.size.y;
        
        let mouse_pos = mouse_position();

        draw_rectangle(x, y, w, h, BLACK);
        draw_rectangle_lines(x, y, w, h, 5.0, WHITE);

        if self.state == GameStartMenuState::None {
            // TODO: Сделать меню с двумя кнопками, меняющими self.state
        }
        else if self.state == GameStartMenuState::ChosingDifficulty {
            let result = self.difficulty_chose_menu.draw(font);
            if let Some(chosen_difficulty) = result {
                self.chosen_difficulty = chosen_difficulty;
            }
        }
        else if self.state == GameStartMenuState::ChosingEmployees {
            let result = self.employees_rotation_menu.draw(font);
            if let Some(chosen_employees) = result {
                self.chosen_employees = chosen_employees;
            }
        }

        None
    }
}
