use macroquad::{color::*, input::mouse_position, math::Vec2, rand::ChooseRandom, shapes::{draw_rectangle, draw_rectangle_lines}, text::*};
use strum::IntoEnumIterator;

use crate::types::{Employee, Position, load_employees};


fn update_available_employees(employees: &Vec<Employee>) -> Vec<Employee> {
    let time_seed = (macroquad::time::get_time() * 1000000.0) as u64; 
    macroquad::rand::srand(time_seed);

    let mut available_employees: Vec<Employee> = Vec::new();

    for position in Position::iter() {
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

    employees: Vec<Employee>,
    available_employees: Vec<Employee>,
    
    // TODO: сделать, чтобы карточка сотрудника закрывалась, если выйти за её пределы
    // opened_employee_card_index: usize
}


impl EmployeesRotationMenu {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        let employees = load_employees();
        let available_employees = update_available_employees(&employees);
        Self { pos, size, employees, available_employees }
    }
    
    pub fn update_available_employees(&mut self) {
        self.available_employees = update_available_employees(&self.employees);
    }

    pub fn draw(&self, font: &Font) -> Option<Vec<Employee>> {
        draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, BLACK);
        draw_rectangle_lines(self.pos.x, self.pos.y, self.size.x, self.size.y, 5.0, WHITE);

        let font_size = 24;
        let indent = 10.0;
        let h = font_size as f32 + indent * 2.0;
        let slot_height = h + indent;
        
        // 1. Вычисляем, сколько строк помещается в одну колонку
        let max_rows_per_column = (self.size.y / slot_height).floor() as usize;
        let max_rows_per_column = max_rows_per_column.max(1); 
        
        let total_items = self.available_employees.len();
        let total_columns = (total_items + max_rows_per_column - 1) / max_rows_per_column;
    
        // Массив для хранения максимальной ширины карточек для каждой колонки
        let mut column_widths = vec![0.0f32; total_columns];
    
        // --- ПЕРВЫЙ ПРОХОД: Вычисляем максимальную ширину каждой колонки ---
        for (i, employee) in self.available_employees.iter().enumerate() {
            let text_dimension = measure_text(
                &format!("{:?} {:?}", employee.grade, employee.position),
                Some(font),
                font_size,
                1.0,
            );
            let w = text_dimension.width + indent * 2.0;
            let col_idx = i / max_rows_per_column;
            
            if col_idx < column_widths.len() {
                column_widths[col_idx] = column_widths[col_idx].max(w);
            }
        }
    
        let mouse_pos = mouse_position();
        let mut clicked_employee: Option<Employee> = None;
    
        // --- ВТОРОЙ ПРОХОД: Отрисовка и проверка взаимодействий ---
        for (i, employee) in self.available_employees.iter().enumerate().rev() {
            let text_dimension = measure_text(
                &format!("{:?} {:?}", employee.grade, employee.position),
                Some(font),
                font_size,
                1.0,
            );
            
            let col_idx = i / max_rows_per_column;
            let row_idx = i % max_rows_per_column;
            
            // Ширина текущей карточки и максимальная ширина её колонки
            let max_col_width = column_widths[col_idx];
            let w = max_col_width;
    
            // Считаем X: суммируем максимальные ширины всех предыдущих колонок
            let mut previous_columns_width = 0.0;
            for c in 0..col_idx {
                previous_columns_width += column_widths[c] + indent;
            }
        
            let x = self.pos.x + indent + previous_columns_width;
            let y = self.pos.y + indent + (row_idx as f32 * slot_height);

            // TODO: строка 35
            if mouse_pos.0 >= x && mouse_pos.0 <= x + w && mouse_pos.1 >= y && mouse_pos.1 <= y + h {
                let menu_text_dimension = measure_text(format!("{}, {} лет", employee.name, employee.age),
                Some(font), font_size, 1.0);
    
                let menu_w = menu_text_dimension.width + indent * 2.0;
                let menu_h = h * 2.0 - indent;
                let menu_x = x;
                let menu_y = y + h;
    
                draw_rectangle(x, y, w.max(menu_w), h + menu_h, BLACK);
                draw_rectangle_lines(x, y, w.max(menu_w), h + menu_h, 5.0, WHITE);
                draw_multiline_text_ex(format!("{}, {} лет\n{}р/мес.", employee.name, employee.age, employee.salary),
                menu_x + indent, menu_y + menu_h / 2.0, Some(1.0),
                TextParams { font: Some(font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE });

                // TODO: при нажатии добавить в список выбранных сотрудников

            } else {
                draw_rectangle(x, y, w, h, BLACK);
                draw_rectangle_lines(x, y, w, h, 5.0, WHITE);
            }
    
            draw_text_ex(format!("{:?} {:?}", employee.grade, employee.position),
            x + indent, y + h / 2.0 + indent,
            TextParams { font: Some(&font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE });

            // TODO: сделать кнопку окончания выбора
        }
        None
    }
}
