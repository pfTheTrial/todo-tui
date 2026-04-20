use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Clear},
    Frame,
    text::{Line, Span},
    style::{Modifier, Style},
};
use crate::app::App;

pub const SETTINGS_ITEM_COUNT: usize = 11;

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let theme = &app.theme;
    
    let config_items: Vec<(String, String)> = vec![
        (app.t("menu.settings.theme").to_string(), app.theme.name.clone()),
        (app.t("menu.settings.notifications").to_string(), if app.settings.notifications_enabled { "ON".to_string() } else { "OFF".to_string() }),
        (app.t("menu.settings.startup").to_string(), if app.settings.startup_with_windows { "ON".to_string() } else { "OFF".to_string() }),
        (app.t("menu.settings.language").to_string(), app.settings.language.code().to_string()),
        (app.t("menu.settings.sync_interval").to_string(), format!("{} min", app.settings.sync_interval_mins)),
        (app.t("menu.settings.notion_key").to_string(), match &app.settings.notion_api_key {
            Some(k) if k.len() > 8 => {
                let prefix: String = k.chars().take(7).collect();
                let suffix: String = k.chars().rev().take(4).collect::<Vec<_>>().into_iter().rev().collect();
                format!("{}...{}", prefix, suffix)
            },
            Some(_) => "****".to_string(),
            None => "".to_string(),
        }),
        (app.t("menu.settings.notion_db").to_string(), match &app.settings.notion_database_id {
            Some(id) if id.len() > 8 => {
                let prefix: String = id.chars().take(7).collect();
                let suffix: String = id.chars().rev().take(4).collect::<Vec<_>>().into_iter().rev().collect();
                format!("{}...{}", prefix, suffix)
            },
            Some(_) => "****".to_string(),
            None => "".to_string(),
        }),
    ];

    let action_items: Vec<String> = vec![
        app.t("menu.settings.export").to_string(),
        app.t("menu.settings.import").to_string(),
        app.t("menu.settings.update").to_string(),
        app.t("menu.settings.sync").to_string(),
    ];

    let mut lines = Vec::new();
    
    // Config items
    let config_len = config_items.len();
    for (i, (label, value)) in config_items.iter().enumerate() {
        let prefix = if i == app.menu_cursor { "> " } else { "  " };
        let style = if i == app.menu_cursor {
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };

        lines.push(Line::from(vec![
            Span::styled(format!("{}{}: ", prefix, label), style),
            Span::styled(value.as_str(), Style::default().fg(theme.accent_secondary).add_modifier(Modifier::BOLD)),
        ]));
    }

    // Separator
    lines.push(Line::from(Span::styled(
        format!("  {}", "─".repeat(area.width.saturating_sub(4) as usize)),
        Style::default().fg(theme.muted)
    )));

    // Action items
    for (i, label) in action_items.iter().enumerate() {
        let real_index = i + config_len;
        let prefix = if real_index == app.menu_cursor { "> " } else { "  " };
        let style = if real_index == app.menu_cursor {
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };
        lines.push(Line::from(Span::styled(format!("{}{}", prefix, label), style)));
    }

    lines.push(Line::from(""));
    
    // Status message
    if let Some(ref msg) = app.status_message {
        lines.push(Line::from(Span::styled(format!("  {}", msg), Style::default().fg(theme.success).add_modifier(Modifier::BOLD))));
    }
    
    lines.push(Line::from(Span::styled(format!(" [Esc] {}", app.t("confirm.no")), Style::default().fg(theme.muted))));

    let block = Block::default()
        .borders(Borders::ALL)
        .title(app.t("menu.settings.title"))
        .border_style(Style::default().fg(theme.accent));

    f.render_widget(Clear, area);
    f.render_widget(Paragraph::new(lines).block(block).style(Style::default().bg(theme.bg)), area);
}
