use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
pub mod components;
pub mod theme;
use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    // Layout principle (lazygit style):
    // [ Task List (35%) ] [ Detail (65%, top 60%)      ]
    // [                 ] [ Pomodoro (65%, bottom 40%) ]
    
    let screen = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(1),
    ]).split(f.area());
    
    let main = Layout::horizontal([
        Constraint::Percentage(35),
        Constraint::Percentage(65),
    ]).split(screen[0]);
    
    let right = Layout::vertical([
        Constraint::Percentage(60),
        Constraint::Percentage(40),
    ]).split(main[1]);

    // Render Components
    components::task_list::draw(f, app, main[0]);
    components::detail::draw(f, app, right[0]);
    components::pomodoro::draw(f, app, right[1]);
    components::footer::draw(f, app, screen[1]);
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
