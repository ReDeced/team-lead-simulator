use macroquad::{color::{BLACK, WHITE}, math::Vec2, shapes::{draw_rectangle, draw_rectangle_lines}, text::Font};

use crate::{interface::employees_rotation_menu::EmployeesRotationMenu, types::{Difficulty, Employee}};


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
    // difficulty_chose_menu: DiffcultyChoseMenu
}


impl GameStartMenu {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        let employees_rotation_menu = EmployeesRotationMenu::new(pos + pos / 5.0, size / 5.0 * 3.0);
        Self { pos, size, state: GameStartMenuState::None, chosen_employees: Vec::new(),
            chosen_difficulty: Difficulty::Tutorial, employees_rotation_menu }
    }

    pub fn draw(&mut self, font: &Font, mouse_pos: (f32, f32)) -> Option<(Difficulty, Vec<Employee>)> {
        let x = self.pos.x;
        let y = self.pos.y;
        let w = self.size.x;
        let h = self.size.y;

        draw_rectangle(x, y, w, h, BLACK);
        draw_rectangle_lines(x, y, w, h, 5.0, WHITE);

        if self.state == GameStartMenuState::None {
            // TODO: Сделать меню с двумя кнопками, меняющими self.state
        }
        else if self.state == GameStartMenuState::ChosingDifficulty {
            // let result = self.difficulty_chose_menu.draw(font);
            // if let Some(chosen_difficulty) = result {
            //     self.chosen_difficulty = chosen_difficulty;
            // }
        }
        else if self.state == GameStartMenuState::ChosingEmployees {
            let result = self.employees_rotation_menu.menu(font, mouse_pos);
            if let Some(chosen_employees) = result {
                self.chosen_employees = chosen_employees;
            }
        }

        None
    }
}
