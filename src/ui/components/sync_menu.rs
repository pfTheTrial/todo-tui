use ratatui::{
    layout::{Rect},
    widgets::{Block, Borders, Paragraph, Clear},
    Frame,
    text::{Line, Span},
    style::{Modifier, Style},
};
use crate::app::{App};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let theme = &app.theme;
    
    let notion_configured = app.settings.notion_api_key.is_some();
    let items = vec![
        (app.t("menu.sync.github"), false),
        (app.t("menu.sync.gdrive"), false),
        (app.t("menu.sync.gcal"), false),
        (app.t("menu.sync.notion"), notion_configured),
    ];

    let mut lines = Vec::new();
    for (i, (label, configured)) in items.iter().enumerate() {
        let prefix = if i == app.menu_cursor { "> " } else { "  " };
        let style = if i == app.menu_cursor {
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };

        let status_text = if *configured { 
            app.t("menu.sync.configured")
        } else { 
            app.t("menu.sync.not_configured") 
        };
        
        let status_style = if *configured {
            Style::default().fg(theme.success)
        } else {
            Style::default().fg(theme.muted)
        };

        let dot = if *configured { "●" } else { "○" };

        lines.push(Line::from(vec![
            Span::styled(format!("{}{}: ", prefix, label), style),
            Span::styled(format!("{} {}", dot, status_text), status_style),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(format!(" [Esc] {}", app.t("confirm.no")), Style::default().fg(theme.muted))));

    let block = Block::default()
        .borders(Borders::ALL)
        .title(app.t("menu.sync.title"))
        .border_style(Style::default().fg(theme.accent));

    f.render_widget(Clear, area);
    f.render_widget(Paragraph::new(lines).block(block).style(Style::default().bg(theme.bg)), area);
}
