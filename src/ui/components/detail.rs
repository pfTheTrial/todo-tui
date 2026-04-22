use crate::app::{ActivePanel, App, TaskSyncState};
use chrono::Local;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let theme = &app.theme;
    let active_style = Style::default()
        .fg(theme.accent)
        .add_modifier(Modifier::BOLD);
    let inactive_style = Style::default().fg(theme.muted);
    let today = Local::now().date_naive();

    let detail_active = app.active_panel == ActivePanel::TaskDetail;
    let detail_style = if detail_active {
        active_style
    } else {
        inactive_style
    };

    if let Some(task) = app.current_task() {
        // Calculate how much space we need for info+reviews
        let review_lines = task.review_subtasks.len();
        let info_min_height = (6 + review_lines) as u16;

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(info_min_height.max(8)), // Info + Reviews (dynamic)
                Constraint::Percentage(40),              // Description
                Constraint::Length(3),                   // Importance / Review Box
            ])
            .split(area);

        let mut lines = Vec::new();

        let next_rev = match task.effective_date() {
            Some(date) => date.with_timezone(&Local).format("%d/%m/%Y").to_string(),
            None => app.t("status.scheduled").to_string(),
        };

        // Importance with color
        let (imp_label, imp_color) = match task.importance {
            crate::model::task::Importance::Urgent => (app.t("importance.urgent"), theme.error),
            crate::model::task::Importance::High => (app.t("importance.high"), theme.warning),
            crate::model::task::Importance::Medium => (app.t("importance.medium"), theme.accent),
            crate::model::task::Importance::Low => (app.t("importance.low"), theme.success),
        };

        lines.push(Line::from(vec![
            Span::raw(format!(" {}  : ", app.t("wizard.title"))),
            Span::styled(
                &task.title,
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::raw(format!(" {}   : ", app.t("detail.due"))),
            Span::styled(next_rev, Style::default().fg(theme.accent_secondary)),
        ]));
        lines.push(Line::from(vec![
            Span::raw(format!(" {}   : ", app.t("section.status"))),
            Span::styled(
                if task.review_state.is_due() {
                    app.t("status.due")
                } else {
                    app.t("status.scheduled")
                },
                Style::default().fg(if task.review_state.is_due() {
                    theme.error
                } else {
                    theme.success
                }),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::raw(format!(" {}   : ", app.t("section.importance"))),
            Span::styled(
                imp_label,
                Style::default().fg(imp_color).add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::raw(format!(" {}   : ", app.t("section.review_plan"))),
            Span::styled(&task.custom_review_str, Style::default().fg(theme.muted)),
        ]));
        if let Some(sync_status) = app.task_sync_status(task.id, "notion") {
            let sync_label = match (sync_status.state, sync_status.action) {
                (TaskSyncState::Failed, crate::model::SyncAction::Delete) => {
                    app.t("sync.task.failed_delete")
                }
                (TaskSyncState::Failed, crate::model::SyncAction::Pull) => {
                    app.t("sync.task.failed_pull")
                }
                (TaskSyncState::Failed, crate::model::SyncAction::Push) => {
                    app.t("sync.task.failed_push")
                }
                (TaskSyncState::Pending, crate::model::SyncAction::Delete) => {
                    app.t("sync.task.pending_delete")
                }
                (TaskSyncState::Pending, crate::model::SyncAction::Pull) => {
                    app.t("sync.task.pending_pull")
                }
                (TaskSyncState::Pending, crate::model::SyncAction::Push) => {
                    app.t("sync.task.pending_push")
                }
                (TaskSyncState::Synced, _) => app.t("sync.task.synced"),
            };
            let sync_color = match sync_status.state {
                TaskSyncState::Failed => theme.error,
                TaskSyncState::Pending => theme.warning,
                TaskSyncState::Synced => theme.success,
            };
            lines.push(Line::from(vec![
                Span::raw(format!(" {}     : ", app.t("menu.settings.sync"))),
                Span::styled(
                    format!("{} ({})", sync_label, sync_status.provider),
                    Style::default().fg(sync_color),
                ),
            ]));
            if let Some(next_retry_at) = sync_status.next_retry_at {
                lines.push(Line::from(vec![
                    Span::raw(format!(" {}    : ", app.t("sync.retry_at"))),
                    Span::styled(
                        next_retry_at
                            .with_timezone(&Local)
                            .format("%d/%m %H:%M")
                            .to_string(),
                        Style::default().fg(theme.warning),
                    ),
                ]));
            }
            if let Some(error) = sync_status.last_error.as_deref() {
                lines.push(Line::from(vec![
                    Span::raw(format!(" {}: ", app.t("sync.last_error"))),
                    Span::styled(error.to_string(), Style::default().fg(theme.error)),
                ]));
            }
        }

        if !task.review_subtasks.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![Span::styled(
                format!(" --- {} ---", app.t("section.review_plan")),
                Style::default().fg(theme.category_fg),
            )]));
            for sub in &task.review_subtasks {
                let status = if sub.completed { "[✓]" } else { "[ ]" };
                let days_left = sub
                    .date
                    .with_timezone(&Local)
                    .date_naive()
                    .signed_duration_since(today)
                    .num_days();
                let dl_str = if sub.completed {
                    app.t("status.done").to_string()
                } else if days_left < 0 {
                    format!("{}d {}", -days_left, app.t("days.ago"))
                } else if days_left == 0 {
                    app.t("cat.today").to_string()
                } else {
                    format!("{}d", days_left)
                };
                let format_date = sub.date.with_timezone(&Local).format("%d/%m").to_string();

                let line_str = format!("  {} {} - {} ({})", status, sub.label, format_date, dl_str);

                if sub.completed {
                    lines.push(Line::from(Span::styled(
                        line_str,
                        Style::default().bg(theme.success).fg(theme.bg),
                    )));
                } else {
                    lines.push(Line::from(line_str));
                }
            }
        }

        let scroll_indicator = if detail_active && app.detail_scroll_offset > 0 {
            format!(
                " {} [3] ▲{}",
                app.t("section.info"),
                app.detail_scroll_offset
            )
        } else {
            format!(" {} [3] ", app.t("section.info"))
        };

        let header = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(scroll_indicator)
                    .border_style(detail_style)
                    .style(Style::default().bg(theme.bg)),
            )
            .scroll((app.detail_scroll_offset, 0));
        f.render_widget(header, layout[0]);

        let description = Paragraph::new(task.description.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} [d/e] ", app.t("section.description")))
                .border_style(detail_style)
                .style(Style::default().bg(theme.bg).fg(theme.fg)),
        );
        f.render_widget(description, layout[1]);

        let bottom_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[2]);

        let importance_str = format!(" {}: {} ('i')", app.t("section.importance"), imp_label);
        let importance = Paragraph::new(importance_str).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", app.t("section.importance")))
                .border_style(detail_style)
                .style(Style::default().bg(theme.bg).fg(imp_color)),
        );
        f.render_widget(importance, bottom_layout[0]);

        let subtasks_text = format!(
            " {}: {} ('r')",
            app.t("section.review_plan"),
            task.custom_review_str
        );

        let rev_box = Paragraph::new(subtasks_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", app.t("section.review_plan")))
                .border_style(detail_style)
                .style(Style::default().bg(theme.bg).fg(theme.fg)),
        );
        f.render_widget(rev_box, bottom_layout[1]);
    } else {
        let empty = Paragraph::new(app.t("detail.empty")).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} [3] ", app.t("section.info")))
                .border_style(detail_style)
                .style(Style::default().bg(theme.bg).fg(theme.muted)),
        );
        f.render_widget(empty, area);
    }
}
