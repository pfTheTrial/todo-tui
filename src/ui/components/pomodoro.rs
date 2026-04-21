use ratatui::{
    layout::{Rect, Layout, Direction, Constraint},
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph, Gauge},
    Frame,
};
use crate::app::{App, ActivePanel};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let theme = &app.theme;
    let active_style = Style::default().fg(theme.accent).add_modifier(Modifier::BOLD);
    let inactive_style = Style::default().fg(theme.muted);

    let pomodoro_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} [2] ", app.t("section.pomodoro")))
        .border_style(if app.active_panel == ActivePanel::Pomodoro { active_style } else { inactive_style })
        .style(Style::default().bg(theme.bg).fg(theme.fg));

    let p = &app.pomodoro;
    let profile = p.active_profile();
    let lock_icon = if p.is_running { " 🔒" } else { "" };
    
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(2),
        ])
        .split(area);

    f.render_widget(pomodoro_block, area);

    let status_text = if p.is_running { app.t("status.running") } else { app.t("status.paused") };
    
    let pomodoro_text = format!(
        "{}: {}{}\n{}: {:?}\n{}: {:02}:{:02}\n{}: {}\n{}: {} | {}: {}m / {}m",
        app.t("pomo.profile"), profile.name, lock_icon,
        app.t("pomo.phase"), p.phase,
        app.t("pomo.time"), p.remaining_seconds / 60, p.remaining_seconds % 60,
        app.t("pomo.status"), status_text,
        app.t("pomo.session"), p.total_sessions_completed,
        app.t("pomo.break"), profile.short_break, profile.long_break
    );
    f.render_widget(Paragraph::new(pomodoro_text).style(Style::default().fg(theme.fg)), layout[0]);

    let progress = p.progress();
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(theme.success).bg(theme.muted).add_modifier(Modifier::ITALIC))
        .label(format!("{:.0}%", progress * 100.0))
        .ratio(progress);
    f.render_widget(gauge, layout[1]);
}
