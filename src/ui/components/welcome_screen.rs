use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Clear},
    Frame,
};
use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = crate::ui::centered_rect(70, 70, f.area());
    let theme = &app.theme;
    
    f.render_widget(Clear, area);
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} {}/6 ", app.t("welcome.title_prefix"), app.onboarding_index + 1))
        .border_style(Style::default().fg(theme.accent).add_modifier(Modifier::BOLD))
        .style(Style::default().bg(theme.bg));

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(8), // Animation/Logo
            Constraint::Min(5),    // Content
            Constraint::Length(3), // Footer hints
        ])
        .split(area);

    f.render_widget(block, area);

    // Slide Content
    match app.onboarding_index {
        0 => draw_slide_1(f, app, layout[0], layout[1]),
        1 => draw_slide_2(f, app, layout[0], layout[1]),
        2 => draw_slide_3(f, app, layout[0], layout[1]),
        3 => draw_slide_4(f, app, layout[0], layout[1]),
        4 => draw_slide_5(f, app, layout[0], layout[1]),
        5 => draw_slide_6(f, app, layout[0], layout[1]),
        _ => {}
    }

    let hints = Line::from(vec![
        Span::styled(" [n] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::raw(if app.onboarding_index < 5 { format!("{} ", app.t("welcome.hint.next")) } else { format!("{} ", app.t("welcome.hint.finish")) }),
        Span::styled(" [p] ", Style::default().fg(theme.muted)),
        Span::raw(format!("{} ", app.t("welcome.hint.prev"))),
        Span::styled(" [Esc] ", Style::default().fg(theme.error)),
        Span::raw(app.t("welcome.hint.skip")),
    ]);
    f.render_widget(Paragraph::new(hints).alignment(Alignment::Center), layout[2]);
}

fn draw_slide_1(f: &mut Frame, app: &App, logo_area: Rect, content_area: Rect) {
    let logo = r#"
  ████████╗██████╗ ████████╗
  ╚══██╔══╝██╔══██╗╚══██╔══╝
     ██║   ██║  ██║   ██║   
     ██║   ██║  ██║   ██║   
     ██║   ██████╔╝   ██║   
     ╚═╝   ╚═════╝    ╚═╝   
    "#;
    f.render_widget(Paragraph::new(logo).style(Style::default().fg(app.theme.accent)).alignment(Alignment::Center), logo_area);
    
    let text = vec![
        Line::from(app.t("welcome.splash.subtitle")),
        Line::from(app.t("welcome.splash.desc")),
        Line::from(""),
        Line::from(vec![Span::styled(app.t("welcome.splash.cta"), Style::default().add_modifier(Modifier::ITALIC))]),
    ];
    f.render_widget(Paragraph::new(text).alignment(Alignment::Center), content_area);
}

fn draw_slide_2(f: &mut Frame, app: &App, _logo_area: Rect, content_area: Rect) {
    let text = vec![
        Line::from(Span::styled(app.t("welcome.layout.title"), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(app.t("welcome.layout.tasks")),
        Line::from(app.t("welcome.layout.detail")),
        Line::from(app.t("welcome.layout.pomo")),
        Line::from(""),
        Line::from(app.t("welcome.layout.nav")),
    ];
    f.render_widget(Paragraph::new(text).alignment(Alignment::Center), content_area);
}

fn draw_slide_3(f: &mut Frame, app: &App, _logo_area: Rect, content_area: Rect) {
    let text = vec![
        Line::from(Span::styled(app.t("welcome.review.title"), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(app.t("welcome.review.line1")),
        Line::from(app.t("welcome.review.line2")),
        Line::from(app.t("welcome.review.line3")),
        Line::from(""),
        Line::from(app.t("welcome.review.line4")),
    ];
    f.render_widget(Paragraph::new(text).alignment(Alignment::Center), content_area);
}

fn draw_slide_4(f: &mut Frame, app: &App, _logo_area: Rect, content_area: Rect) {
    let text = vec![
        Line::from(Span::styled(app.t("welcome.cmd.title"), Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(app.t("welcome.cmd.search")),
        Line::from(app.t("welcome.cmd.sort")),
        Line::from(app.t("welcome.cmd.add")),
        Line::from(app.t("welcome.cmd.config")),
        Line::from(app.t("welcome.cmd.help")),
    ];
    f.render_widget(Paragraph::new(text).alignment(Alignment::Left).block(Block::default().borders(Borders::NONE).padding(ratatui::widgets::Padding::left(20))), content_area);
}

fn draw_slide_5(f: &mut Frame, app: &App, _logo_area: Rect, content_area: Rect) {
    let text = vec![
        Line::from(Span::styled(app.t("welcome.int.title"), Style::default().fg(app.theme.success).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(app.t("welcome.int.line1")),
        Line::from(app.t("welcome.int.line2")),
        Line::from(""),
        Line::from(vec![
            Span::raw(app.t("welcome.int.sync_label")),
            Span::styled(format!("{} min", app.settings.sync_interval_mins), Style::default().fg(app.theme.accent))
        ]),
    ];
    f.render_widget(Paragraph::new(text).alignment(Alignment::Center), content_area);
}

fn draw_slide_6(f: &mut Frame, app: &App, _logo_area: Rect, content_area: Rect) {
    let text = vec![
        Line::from(Span::styled(app.t("welcome.ready.title"), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(app.t("welcome.ready.line1")),
        Line::from(app.t("welcome.ready.line2")),
        Line::from(""),
        Line::from(app.t("welcome.ready.cta")),
    ];
    f.render_widget(Paragraph::new(text).alignment(Alignment::Center), content_area);
}


