use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Clear},
    Frame,
    text::{Line, Span},
    style::{Modifier, Style},
};
use crate::app::App;

// Items: Appearance(2) + System(3) + Sync(1) + Update(1) + Stats(1) = 8
pub const SETTINGS_ITEM_COUNT: usize = 8;

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let theme = &app.theme;
    let header_style = Style::default().fg(theme.category_fg).add_modifier(Modifier::BOLD);
    let separator = format!("  {}", "─".repeat(area.width.saturating_sub(6) as usize));

    // Items  (index must match input.rs handler)
    // 0: Theme        1: Language
    // 2: Notif Pomo   3: Notif Tasks   4: Startup
    // 5: Sync & Integrations
    // 6: Update tdt   7: Stats
    let items: [(String, String); SETTINGS_ITEM_COUNT] = [
        // 🎨 APPEARANCE
        (app.t("menu.settings.theme").to_string(),         app.theme.name.clone()),
        (app.t("menu.settings.language").to_string(),      app.settings.language.code().to_string()),
        // 🔔 SYSTEM
        (app.t("menu.settings.notif_pomo").to_string(), if app.settings.notifications_enabled { "ON".into() } else { "OFF".into() }),
        (app.t("menu.settings.notif_tasks").to_string(), if app.settings.task_reminders_enabled { "ON".into() } else { "OFF".into() }),
        (app.t("menu.settings.startup").to_string(),       if app.settings.startup_with_windows { "ON".into() } else { "OFF".into() }),
        // 🔗 INTEGRATIONS (→ opens sync submenu)
        (app.t("settings.section.integrations").to_string(), {
            let notion_ok = app.settings.notion_api_key.is_some();
            format!("{} Notion {}", if notion_ok { "●" } else { "○" },
                if notion_ok { app.t("menu.sync.configured") } else { app.t("menu.sync.not_configured") })
        }),
        // 🔄 UPDATE
        (app.t("menu.settings.update").to_string(), {
            if app.is_npm {
                app.t("settings.npm_managed").to_string()
            } else if let Some(ref info) = app.update_info {
                if info.has_update {
                    format!("v{} {}", info.latest, app.t("update.available"))
                } else {
                    app.t("menu.settings.no_update").to_string()
                }
            } else {
                String::new()
            }
        }),
        (app.t("settings.perf.title").to_string(), {
            format!("RAM {:.1} MB  CPU {:.1}%", app.sys_ram_mb, app.sys_cpu_pct)
        }),
    ];

    let mut lines = Vec::new();

    for (i, (label, value)) in items.iter().enumerate() {
        // Section headers
        match i {
            0 => lines.push(Line::from(Span::styled(
                format!("  {}", app.t("settings.section.appearance")), header_style,
            ))),
            2 => {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("  {}", app.t("settings.section.system")), header_style,
                )));
            }
            5 => {
                lines.push(Line::from(Span::styled(&separator, Style::default().fg(theme.muted))));
            }
            _ => {}
        }

        let prefix = if i == app.menu_cursor { " ▸ " } else { "   " };
        let style = if i == app.menu_cursor {
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };

        if value.is_empty() {
            lines.push(Line::from(Span::styled(format!("{}{}", prefix, label), style)));
        } else {
            lines.push(Line::from(vec![
                Span::styled(format!("{}{}: ", prefix, label), style),
                Span::styled(value.as_str(),
                    Style::default().fg(theme.accent_secondary).add_modifier(Modifier::BOLD)),
            ]));
        }
    }

    lines.push(Line::from(""));

    if let Some(ref msg) = app.status_message {
        lines.push(Line::from(Span::styled(
            format!("  {}", msg),
            Style::default().fg(theme.success).add_modifier(Modifier::BOLD),
        )));
    }

    lines.push(Line::from(Span::styled(
        format!(" [Esc] {}", app.t("confirm.no")),
        Style::default().fg(theme.muted),
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .title(app.t("menu.settings.title"))
        .border_style(Style::default().fg(theme.accent));

    f.render_widget(Clear, area);
    f.render_widget(
        Paragraph::new(lines).block(block).style(Style::default().bg(theme.bg)),
        area,
    );
}
