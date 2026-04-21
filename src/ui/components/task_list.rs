use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use crate::app::{App, ActivePanel};
use chrono::Local;

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let theme = &app.theme;
    let active_style = Style::default().fg(theme.accent).add_modifier(Modifier::BOLD);
    let inactive_style = Style::default().fg(theme.muted);
    let highlight_style = Style::default().bg(theme.selection_bg).fg(theme.selection_fg).add_modifier(Modifier::BOLD);
    let due_style = Style::default().fg(theme.error).add_modifier(Modifier::BOLD);

    let today = Local::now().date_naive();

    let mut items = Vec::new();
    let mut current_category = -1;

    let filtered = app.filtered_tasks();
    for (_i, (original_idx, task)) in filtered.iter().enumerate() {
        let cat = match task.effective_date() {
            Some(dt) => {
                let diff = dt.with_timezone(&Local).date_naive().signed_duration_since(today).num_days();
                if diff < 0 { 0 }
                else if diff == 0 { 1 }
                else { 2 }
            },
            None => 3,
        };

        if cat != current_category {
            current_category = cat;
            let header = match cat {
                0 => format!(" [!] {} {} ", app.t("cat.overdue"), "─".repeat(area.width as usize / 4)),
                1 => format!(" [★] {} {} ", app.t("cat.today"), "─".repeat(area.width as usize / 4)),
                2 => format!(" [🕓] {} {} ", app.t("cat.upcoming"), "─".repeat(area.width as usize / 4)),
                _ => format!(" [📥] {} {} ", app.t("cat.inbox"), "─".repeat(area.width as usize / 4)),
            };
            items.push(ListItem::new(header).style(Style::default().fg(theme.category_fg).add_modifier(Modifier::BOLD)));
        }

        let prefix = if *original_idx == app.selected_task_index { "> " } else { "  " };
        let status = if task.completed { 
            if task.review_subtasks.iter().any(|s| !s.completed) { "[✓ Rev]" } else { "[✓]" }
        } else { "[ ]" };
        
        let (importance_flag, _importance_color) = match task.importance {
            crate::model::task::Importance::Urgent => ("🔴", theme.error),
            crate::model::task::Importance::High   => ("🟠", theme.warning),
            crate::model::task::Importance::Medium  => ("🟡", theme.accent),
            crate::model::task::Importance::Low     => ("🟢", theme.success),
        };
        
        let review_flag = if task.review_state.is_due() || task.effective_date().map_or(false, |dt| dt <= chrono::Utc::now()) { "*" } else { " " };
        
        let days_left = match task.effective_date() {
            Some(dt) => {
                let diff = dt.with_timezone(&Local).date_naive().signed_duration_since(today).num_days();
                if diff < 0 { format!("({}d {})", -diff, app.t("days.ago")) }
                else if diff == 0 { format!("({})", app.t("cat.today")) }
                else { format!("({} {}d)", app.t("days.in"), diff) }
            },
            None => String::new(),
        };

        let base_style = if task.completed && !task.review_subtasks.iter().any(|s| !s.completed) {
            Style::default().fg(theme.muted)
        } else if task.review_state.is_due() {
            due_style
        } else {
            Style::default().fg(theme.fg)
        };

        let style = if *original_idx == app.selected_task_index && app.active_panel == ActivePanel::TaskList {
            highlight_style
        } else {
            base_style
        };

        let title_line = format!("{}{} {} {} {} {}", prefix, review_flag, importance_flag, status, task.title, days_left);
        items.push(ListItem::new(title_line).style(style));
    }

    let task_count = filtered.len();
    let sort_label = match app.sort_mode {
        crate::app::SortMode::Priority => app.t("sort.prio"),
        crate::app::SortMode::Date => app.t("sort.date"),
        crate::app::SortMode::Title => app.t("sort.name"),
    };
    let list_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ({}) [1] [{}] ", app.t("section.tasks"), task_count, sort_label))
        .border_style(if app.active_panel == ActivePanel::TaskList { active_style } else { inactive_style })
        .style(Style::default().bg(theme.bg));

    let list = List::new(items).block(list_block);
    f.render_stateful_widget(list, area, &mut app.task_list_state);
}
