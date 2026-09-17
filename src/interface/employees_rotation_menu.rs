use macroquad::{color::*, math::Vec2, rand::ChooseRandom, shapes::{draw_rectangle, draw_rectangle_lines}, text::*};
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
    available_employees: Vec<Employee>
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

    pub fn menu(&self, font: &Font, mouse_pos: (f32, f32)) -> Option<Vec<Employee>> {
        draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, BLACK);
        draw_rectangle_lines(self.pos.x, self.pos.y, self.size.x, self.size.y, 5.0, BLACK);

        for (i, employee) in self.available_employees.iter().enumerate().rev() {
            let font_size = 24;
            
            let text_dimension = measure_text(format!("{:?} {:?}", employee.grade, employee.position),
            Some(font), font_size, 1.0);
            
            let indent = 10.0;
    
            let w = text_dimension.width + indent * 2.0;
            let h = font_size as f32 + indent * 2.0;
            let x = self.pos.x + indent;
            let y = self.pos.y + indent + (h + indent * 2.0) * i as f32;
            
            if mouse_pos.0 >= x && mouse_pos.0 <= x + w && mouse_pos.1 >= y && mouse_pos.1 <= y + h {
                let menu_text_dimension = measure_text(format!("{}, {} лет", employee.name, employee.age),
                Some(font), font_size, 1.0);
    
                let menu_w = menu_text_dimension.width + indent * 2.0;
                let menu_h = h - indent;
                let menu_x = x;
                let menu_y = y + h;
    
                draw_rectangle(x, y, w.max(menu_w), h + menu_h, BLACK);
                draw_rectangle_lines(x, y, w.max(menu_w), h + menu_h, 5.0, WHITE);
                draw_text_ex(format!("{}, {} лет", employee.name, employee.age),
                menu_x + indent, menu_y + menu_h / 2.0,
                TextParams { font: Some(font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE });
            } else {
                draw_rectangle(x, y, w, h, BLACK);
                draw_rectangle_lines(x, y, w, h, 5.0, WHITE);
            }
    
            draw_text_ex(format!("{:?} {:?}", employee.grade, employee.position),
            x + indent, y + h / 2.0 + indent,
            TextParams { font: Some(&font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE });
        }
        None
    }
}
