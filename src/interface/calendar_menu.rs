use macroquad::{color::{BLACK, DARKGRAY, WHITE}, input::{MouseButton, is_mouse_button_pressed, mouse_position}, math::Vec2, shapes::{draw_rectangle, draw_rectangle_lines}, text::{Font, TextParams, draw_text_ex, measure_text}};

pub struct CalendarMenu {
    pos: Vec2,
    size: Vec2,

    selected_day: u8,
}


impl CalendarMenu {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size, selected_day: 1 }
    }

    pub fn draw(&mut self, font: &Font) -> Option<u8> {
        let font_size = 24;
        let indent = 10.0;

        let x = self.pos.x;
        let y = self.pos.y;
        let w = self.size.x;
        let h = self.size.y;

        let mouse_pos = mouse_position();
        let mouse_x = mouse_pos.0;
        let mouse_y = mouse_pos.1;

        draw_rectangle(x, y, w, h, BLACK);
        draw_rectangle_lines(x, y, w, h, 5.0, WHITE);

        let title = "Календарь";
        let title_surface = measure_text(title, Some(font), font_size, 1.0);
        draw_text_ex(
            title,
            x + (w - title_surface.width) / 2.0,
            y + title_surface.height + indent,
            TextParams { font: Some(font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE },
        );

        let week_days = ["Пн", "Вт", "Ср", "Чт", "Пт", "Сб", "Вс"];
        let header_y = y + title_surface.height + indent * 2.0;
        let cell_w = (w - indent * 2.0) / 7.0;
        let cell_h = (h - (header_y - y) - indent * 2.0) / 6.0;

        for (i, week_day) in week_days.iter().enumerate() {
            let text_surface = measure_text(week_day, Some(font), font_size, 1.0);
            let cell_x = x + indent + cell_w * i as f32;

            draw_text_ex(
                week_day,
                cell_x + (cell_w - text_surface.width) / 2.0,
                header_y + text_surface.height,
                TextParams { font: Some(font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: WHITE },
            );
        }

        for day in 1..=30 {
            let day_index = day - 1;
            let row = day_index / 7;
            let col = day_index % 7;

            let cell_x = x + indent + col as f32 * cell_w;
            let cell_y = header_y + cell_h + row as f32 * cell_h;

            let mut text_color = WHITE;
            let mut fill_color = BLACK;

            let is_hovered = mouse_x >= cell_x
                && mouse_x <= cell_x + cell_w
                && mouse_y >= cell_y
                && mouse_y <= cell_y + cell_h;

            if is_hovered {
                fill_color = DARKGRAY;
                if is_mouse_button_pressed(MouseButton::Left) {
                    self.selected_day = day;
                    return Some(day);
                }
            }

            if self.selected_day == day {
                fill_color = WHITE;
                text_color = BLACK;
            }

            draw_rectangle(cell_x, cell_y, cell_w, cell_h, fill_color);
            draw_rectangle_lines(cell_x, cell_y, cell_w, cell_h, 2.0, WHITE);

            let day_label = day.to_string();
            let text_surface = measure_text(day_label.as_str(), Some(font), font_size, 1.0);
            draw_text_ex(
                day_label.as_str(),
                cell_x + (cell_w - text_surface.width) / 2.0,
                cell_y + (cell_h + text_surface.height) / 2.0,
                TextParams { font: Some(font), font_size, font_scale: 1.0, font_scale_aspect: 1.0, rotation: 0.0, color: text_color },
            );
        }

        None
    }
}
