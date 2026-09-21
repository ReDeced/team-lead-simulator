use macroquad::{color::*, input::{MouseButton::Left, is_mouse_button_pressed, mouse_position}, math::{Rect, Vec2}, rand::ChooseRandom, shapes::{draw_rectangle, draw_rectangle_lines}, text::*};
use strum::IntoEnumIterator;

use crate::types::{Employee, Position, load_employees};


fn update_available_employees(employees: &Vec<Employee>, exclude_positions: Vec<Position>) -> Vec<Employee> {
    let time_seed = (macroquad::time::get_time() * 1000000.0) as u64; 
    macroquad::rand::srand(time_seed);

    let mut available_employees: Vec<Employee> = Vec::new();

    for position in Position::iter().filter(|pos | !exclude_positions.contains(pos) ) {
        let filtered_employees: Vec<&Employee> = employees.iter()
            .filter(|dev| dev.position == position).collect();

        let random_chosen = filtered_employees.choose_multiple(3);

        for dev_ref in random_chosen {
            available_employees.push((*dev_ref).clone());
        }
    }

    available_employees
}


#[derive(Debug, Clone)]
pub struct EmployeesRotationMenu {
    pos: Vec2,
    size: Vec2,

    available_employees: Vec<Employee>,
    chosen_employees: Vec<bool>,
    
    // TODO: сделать, чтобы карточка сотрудника закрывалась, если выйти за её пределы
    // opened_employee_card_index: usize
}


impl EmployeesRotationMenu {
    pub fn new(pos: Vec2, size: Vec2, exclude_positions: Vec<Position>) -> Self {
        let employees = load_employees(exclude_positions.clone());
        let available_employees = update_available_employees(&employees, exclude_positions);
        
        let mut chosen_employees = Vec::new();

        for _ in available_employees.iter() {
            chosen_employees.push(false);
        }

        Self { pos, size, available_employees, chosen_employees: chosen_employees }
    }
    
    pub fn draw(&mut self, font: &Font) -> Option<Vec<Employee>> {
        draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, BLACK);
        draw_rectangle_lines(self.pos.x, self.pos.y, self.size.x, self.size.y, 5.0, WHITE);
    
        let font_size = 24;
        let indent = 10.0;
        let h = font_size as f32 + indent * 2.0;
        let slot_height = h + indent;
    
        let max_rows_per_column = (self.size.y / slot_height).floor() as usize - 2;
        let max_rows_per_column = max_rows_per_column.max(1);
    
        let total_items = self.available_employees.len();
        let total_columns = (total_items + max_rows_per_column - 1) / max_rows_per_column;
    
        let mut column_widths = vec![0.0f32; total_columns];
    
        for (i, employee) in self.available_employees.iter().enumerate() {
            let text_dimension = measure_text(
                &format!("{:?} {:?}", employee.grade, employee.position),
                Some(font), font_size, 1.0,
            );
            let w = text_dimension.width + indent * 2.0;
            let col_idx = i / max_rows_per_column;
            if col_idx < column_widths.len() {
                column_widths[col_idx] = column_widths[col_idx].max(w);
            }
        }
    
        let mouse_pos = mouse_position();
    
        for (i, employee) in self.available_employees.iter().enumerate().rev() {
            let text_dimension = measure_text(
                &format!("{:?} {:?}", employee.grade, employee.position),
                Some(font), font_size, 1.0,
            );
    
            let col_idx = i / max_rows_per_column;
            let row_idx = i % max_rows_per_column;
    
            let max_col_width = column_widths[col_idx];
            let w = max_col_width;
    
            let mut previous_columns_width = 0.0;
            for c in 0..col_idx {
                previous_columns_width += column_widths[c] + indent;
            }
    
            let x = self.pos.x + indent + previous_columns_width;
            let y = self.pos.y + indent + (row_idx as f32 * slot_height);
    
            let mut card_color = BLACK;
            let mut text_color = WHITE;
    
            if self.chosen_employees[i] {
                card_color = DARKGRAY;
                text_color = WHITE;
            }
    
            if mouse_pos.0 >= x && mouse_pos.0 <= x + w && mouse_pos.1 >= y && mouse_pos.1 <= y + h {
                let menu_text_dimension = measure_text(
                    &format!("{}, {} лет", employee.name, employee.age),
                    Some(font), font_size, 1.0,
                );
    
                let menu_w = menu_text_dimension.width + indent * 2.0;
                let menu_h = h * 2.0 - indent;
                let menu_x = x;
                let menu_y = y + h;
    
                draw_rectangle(x, y, w.max(menu_w), h + menu_h, card_color);
                draw_rectangle_lines(x, y, w.max(menu_w), h + menu_h, 5.0, text_color);
                draw_multiline_text_ex(
                    &format!("{}, {} лет\n{}р/мес.", employee.name, employee.age, employee.salary),
                    menu_x + indent, menu_y + menu_h / 2.0, Some(1.0),
                    TextParams { font: Some(font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: text_color },
                );
    
                if is_mouse_button_pressed(Left) {
                    self.chosen_employees[i] = !self.chosen_employees[i];
                }
            } else {
                draw_rectangle(x, y, w, h, card_color);
                draw_rectangle_lines(x, y, w, h, 5.0, text_color);
            }
    
            draw_text_ex(
                &format!("{:?} {:?}", employee.grade, employee.position),
                x + indent, y + h / 2.0 + indent,
                TextParams { font: Some(&font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: text_color },
            );
        }
    
        // --- Кнопка теперь рисуется и обрабатывается ОДИН раз, поверх всех карточек ---
        let chose_button_text_dimension = measure_text("Выбрать сотрудников", Some(font), font_size, 1.0);
        let chose_button = Rect::new(self.pos.x + indent, self.pos.y + self.size.y - slot_height, self.size.x - indent * 2.0, h);
        let mut chose_button_color = BLACK;
    
        if chose_button.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
            chose_button_color = DARKGRAY;
            if is_mouse_button_pressed(Left) {
                let mut chosen_employees: Vec<Employee> = Vec::new();
                for (i, employee) in self.available_employees.iter().enumerate() {
                    if self.chosen_employees[i] {
                        chosen_employees.push(employee.clone());
                    }
                }
                return Some(chosen_employees);
            }
        }
    
        draw_rectangle(chose_button.x, chose_button.y, chose_button.w, chose_button.h, chose_button_color);
        draw_rectangle_lines(chose_button.x, chose_button.y, chose_button.w, chose_button.h, 5.0, WHITE);
    
        draw_text_ex(
            "Выбрать сотрудников",
            chose_button.x + (chose_button.w - chose_button_text_dimension.width) / 2.0,
            chose_button.y + chose_button_text_dimension.height / 2.0 + chose_button.h / 2.0,
            TextParams { font: Some(font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE },
        );
    
        None
    }
}
