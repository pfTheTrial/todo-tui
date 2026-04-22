use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let theme = &app.theme;

    let notion_configured = app.settings.notion_api_key.is_some();

    // Top section: integration status list
    let providers = [
        (app.t("menu.sync.github"), false),
        (app.t("menu.sync.gdrive"), false),
        (app.t("menu.sync.gcal"), false),
        (app.t("menu.sync.notion"), notion_configured),
    ];

    let notion_key_display = match &app.settings.notion_api_key {
        Some(k) if k.len() > 8 => {
            let prefix: String = k.chars().take(7).collect();
            let suffix: String = k
                .chars()
                .rev()
                .take(4)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            format!("{}...{}", prefix, suffix)
        }
        Some(_) => "****".to_string(),
        None => "—".to_string(),
    };

    let notion_db_display = match &app.settings.notion_database_id {
        Some(id) if id.len() > 8 => {
            let prefix: String = id.chars().take(7).collect();
            let suffix: String = id
                .chars()
                .rev()
                .take(4)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            format!("{}...{}", prefix, suffix)
        }
        Some(_) => "****".to_string(),
        None => "—".to_string(),
    };
    let queue_status = app.sync_queue_status(Some("notion"));
    let sync_metrics = app.sync_metrics("notion");

    let mut lines = Vec::new();

    // Provider list (0..=3 cursors)
    for (i, (label, configured)) in providers.iter().enumerate() {
        let prefix = if i == app.menu_cursor { " ▸ " } else { "   " };
        let style = if i == app.menu_cursor {
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };
        let dot = if *configured { "●" } else { "○" };
        let status = if *configured {
            app.t("menu.sync.configured")
        } else {
            app.t("menu.sync.not_configured")
        };
        let status_style = if *configured {
            Style::default().fg(theme.success)
        } else {
            Style::default().fg(theme.muted)
        };
        lines.push(Line::from(vec![
            Span::styled(format!("{}{}", prefix, label), style),
            Span::raw(": "),
            Span::styled(format!("{} {}", dot, status), status_style),
        ]));

        // Expand Notion config inline when Notion is selected
        if i == 3 && app.menu_cursor == 3 {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("   ─── {} ───", app.t("settings.section.integrations")),
                Style::default()
                    .fg(theme.category_fg)
                    .add_modifier(Modifier::BOLD),
            )));

            let interval_str = format!("{} min", app.settings.sync_interval_mins);
            let notion_items = vec![
                (
                    app.t("menu.settings.notion_key").to_string(),
                    notion_key_display.clone(),
                ),
                (
                    app.t("menu.settings.notion_db").to_string(),
                    notion_db_display.clone(),
                ),
                (
                    app.t("menu.settings.sync_interval").to_string(),
                    interval_str,
                ),
                (
                    "Queue".to_string(),
                    format!(
                        "{} pend.  {} retry  {} falhas",
                        queue_status.pending, queue_status.retrying, queue_status.failed
                    ),
                ),
                (
                    "Push".to_string(),
                    format!(
                        "{} pend.  {} retry  {} falhas",
                        queue_status.pending_actions.push,
                        queue_status.retrying_actions.push,
                        queue_status.failed_actions.push
                    ),
                ),
                (
                    "Delete".to_string(),
                    format!(
                        "{} pend.  {} retry  {} falhas",
                        queue_status.pending_actions.delete,
                        queue_status.retrying_actions.delete,
                        queue_status.failed_actions.delete
                    ),
                ),
                (
                    "Pull".to_string(),
                    format!(
                        "{} pend.  {} retry  {} falhas",
                        queue_status.pending_actions.pull,
                        queue_status.retrying_actions.pull,
                        queue_status.failed_actions.pull
                    ),
                ),
            ];
            for (label, value) in notion_items {
                lines.push(Line::from(vec![
                    Span::raw(format!("     {}: ", label)),
                    Span::styled(
                        value,
                        Style::default()
                            .fg(theme.accent_secondary)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("  [Enter]", Style::default().fg(theme.muted)),
                ]));
            }
            if let Some(next_retry_at) = queue_status.next_retry_at {
                lines.push(Line::from(vec![
                    Span::raw("     Próximo retry: "),
                    Span::styled(
                        next_retry_at
                            .with_timezone(&chrono::Local)
                            .format("%d/%m %H:%M")
                            .to_string(),
                        Style::default().fg(theme.warning),
                    ),
                ]));
            }
            if let Some(metrics) = sync_metrics {
                if let Some(last_success_at) = metrics.last_success_at {
                    lines.push(Line::from(vec![
                        Span::raw(format!("     {}: ", app.t("sync.last_success"))),
                        Span::styled(
                            last_success_at
                                .with_timezone(&chrono::Local)
                                .format("%d/%m %H:%M")
                                .to_string(),
                            Style::default().fg(theme.success),
                        ),
                    ]));
                }
                if let Some(last_error_at) = metrics.last_error_at {
                    let message = metrics.last_error.as_deref().unwrap_or("unknown");
                    lines.push(Line::from(vec![
                        Span::raw(format!("     {}: ", app.t("sync.last_error"))),
                        Span::styled(
                            format!(
                                "{} - {}",
                                last_error_at
                                    .with_timezone(&chrono::Local)
                                    .format("%d/%m %H:%M"),
                                message
                            ),
                            Style::default().fg(theme.error),
                        ),
                    ]));
                }
                for record in metrics.recent_errors.iter().rev().take(3) {
                    lines.push(Line::from(vec![
                        Span::raw("       - "),
                        Span::styled(
                            format!(
                                "{} {:?}: {}",
                                record
                                    .at
                                    .with_timezone(&chrono::Local)
                                    .format("%d/%m %H:%M"),
                                record.action,
                                record.message
                            ),
                            Style::default().fg(theme.error),
                        ),
                    ]));
                }
            }
            lines.push(Line::from(""));
        }
    }

    lines.push(Line::from(""));

    // Status message
    if let Some(ref msg) = app.status_message {
        lines.push(Line::from(Span::styled(
            format!("  {}", msg),
            Style::default()
                .fg(theme.success)
                .add_modifier(Modifier::BOLD),
        )));
    }

    lines.push(Line::from(Span::styled(
        format!(
            " [Enter] {} │ [Esc] {}",
            app.t("hint.edit"),
            app.t("confirm.no")
        ),
        Style::default().fg(theme.muted),
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .title(app.t("menu.sync.title"))
        .border_style(Style::default().fg(theme.accent));

    f.render_widget(Clear, area);
    f.render_widget(
        Paragraph::new(lines)
            .block(block)
            .style(Style::default().bg(theme.bg)),
        area,
    );
}
