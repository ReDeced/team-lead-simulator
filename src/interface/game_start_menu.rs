use macroquad::{color::{BLACK, WHITE}, input::{MouseButton, is_mouse_button_pressed, mouse_position}, math::{Rect, Vec2}, shapes::{draw_rectangle, draw_rectangle_lines}, text::{Font, TextParams, draw_text_ex, measure_text}};

use crate::{interface::{calendar_menu::CalendarMenu, difficulty_chose_menu::DifficultyChooseMenu, employees_rotation_menu::EmployeesRotationMenu}, types::{Difficulty, Employee, Position}};


#[derive(PartialEq)]
enum GameStartMenuState {
    None,
    ChosingDifficulty,
    ChosingEmployees,
    ChosingCalendar,
}


pub struct GameStartMenu {
    pos: Vec2,
    size: Vec2,
    state: GameStartMenuState,
    
    chosen_employees: Vec<Employee>,
    chosen_difficulty: Difficulty,
    chosen_day: u8,

    employees_rotation_menu: EmployeesRotationMenu,
    difficulty_chose_menu: DifficultyChooseMenu,
    calendar_menu: CalendarMenu,
}


impl GameStartMenu {
    pub fn new(pos: Vec2, size: Vec2, max_difficulty: Difficulty) -> Self {
        let employees_rotation_menu = EmployeesRotationMenu::new(pos, size,
            vec![Position::ManualQA, Position::AutoQA, Position::Sysadmin, Position::DevOps]);
        let difficulty_chose_menu = DifficultyChooseMenu::new(
            pos + size / 5.0, size / 5.0 * 3.0, max_difficulty);
        let calendar_menu = CalendarMenu::new(pos + size / 5.0, size / 5.0 * 3.0);
        Self { pos, size, state: GameStartMenuState::None, chosen_employees: Vec::new(),
            chosen_difficulty: Difficulty::Tutorial, chosen_day: 1, employees_rotation_menu, difficulty_chose_menu, calendar_menu }
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
        
        let button_w = w / 5.0 * 3.0;
        let button_h = h / 6.0;
        let button_x = x + w / 5.0;
        let button_gap = h / 12.0;

        let button_1= Rect {
            x: button_x,
            y: y + button_gap,
            w: button_w,
            h: button_h
        };
        let text_surface_1 = measure_text(self.chosen_difficulty.to_string(), Some(font), font_size, 1.0);

        let button_2 = Rect {
            x: button_x,
            y: button_1.y + button_h + button_gap,
            w: button_w,
            h: button_h
        };
        let text_surface_2 = measure_text("Выбрать начальную команду", Some(font), font_size, 1.0);

        let button_3 = Rect {
            x: button_x,
            y: button_2.y + button_h + button_gap,
            w: button_w,
            h: button_h
        };
        let text_surface_3 = measure_text(format!("День старта: {}", self.chosen_day).as_str(), Some(font), font_size, 1.0);

        draw_rectangle(button_1.x, button_1.y, button_1.w, button_1.h, BLACK);
        draw_rectangle_lines(button_1.x, button_1.y, button_1.w, button_1.h, 5.0, WHITE);

        draw_text_ex(self.chosen_difficulty.to_string(),
            button_1.x + (button_1.w - text_surface_1.width) / 2.0,
            button_1.y + (button_1.h + text_surface_1.height) / 2.0,
            TextParams { font: Some(font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE }
        );

        draw_rectangle(button_2.x, button_2.y, button_2.w, button_2.h, BLACK);
        draw_rectangle_lines(button_2.x, button_2.y, button_2.w, button_2.h, 5.0, WHITE);
        
        draw_text_ex("Выбрать начальную команду",
            button_2.x + (button_2.w - text_surface_2.width) / 2.0,
            button_2.y + (button_2.h + text_surface_2.height) / 2.0,
            TextParams { font: Some(font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE }
        );

        draw_rectangle(button_3.x, button_3.y, button_3.w, button_3.h, BLACK);
        draw_rectangle_lines(button_3.x, button_3.y, button_3.w, button_3.h, 5.0, WHITE);

        draw_text_ex(format!("День старта: {}", self.chosen_day).as_str(),
            button_3.x + (button_3.w - text_surface_3.width) / 2.0,
            button_3.y + (button_3.h + text_surface_3.height) / 2.0,
            TextParams { font: Some(font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE }
        );

        if self.state == GameStartMenuState::None {
            if button_1.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
                if is_mouse_button_pressed(MouseButton::Left) {
                    self.state = GameStartMenuState::ChosingDifficulty;
                }
            } 
            else if button_2.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
                if is_mouse_button_pressed(MouseButton::Left) {
                    self.state = GameStartMenuState::ChosingEmployees;
                }
            } else if button_3.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
                if is_mouse_button_pressed(MouseButton::Left) {
                    self.state = GameStartMenuState::ChosingCalendar;
                }
            }
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
        else if self.state == GameStartMenuState::ChosingCalendar {
            let result = self.calendar_menu.draw(font);
            if let Some(chosen_day) = result {
                self.chosen_day = chosen_day;
                self.state = GameStartMenuState::None;
            }
        }
        
        // TODO: сделать кнопку начала игры

        None
    }
}
