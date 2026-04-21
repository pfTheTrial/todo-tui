use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
pub mod components;
pub mod theme;
use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let has_update = app.update_info.as_ref().map_or(false, |i| i.has_update);
    
    let screen = if has_update {
        Layout::vertical([
            Constraint::Length(1),  // Update banner
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Footer
        ]).split(f.area())
    } else {
        Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
        ]).split(f.area())
    };
    
    let (main_area, footer_area) = if has_update {
        (screen[1], screen[2])
    } else {
        (screen[0], screen[1])
    };
    
    // Update banner (top)
    if has_update {
        components::update_banner::draw(f, app, screen[0]);
    }
    
    // Main layout (lazygit style)
    let main = Layout::horizontal([
        Constraint::Percentage(35),
        Constraint::Percentage(65),
    ]).split(main_area);
    
    let right = Layout::vertical([
        Constraint::Percentage(60),
        Constraint::Percentage(40),
    ]).split(main[1]);

    // Render Components
    components::task_list::draw(f, app, main[0]);
    components::detail::draw(f, app, right[0]);
    components::pomodoro::draw(f, app, right[1]);
    components::footer::draw(f, app, footer_area);
    components::modal::draw(f, app);

    if app.input_mode == crate::app::InputMode::Onboarding {
        components::welcome_screen::draw(f, app);
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
