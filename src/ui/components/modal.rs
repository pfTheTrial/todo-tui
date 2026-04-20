use ratatui::{

    widgets::{Block, Borders, Paragraph, Clear},
    Frame,
    text::{Line, Span},
    style::{Modifier, Style},
};
use crate::app::{App, InputMode};

pub fn draw(f: &mut Frame, app: &mut App) {
    let theme = &app.theme;
    
    if app.input_mode != InputMode::Normal {
        let title = match app.input_mode {
            InputMode::EditingTitle | InputMode::CreatingTitle => app.t("wizard.title"),
            InputMode::EditingDescription | InputMode::CreatingDescription => app.t("wizard.desc"),
            InputMode::CreatingDate | InputMode::EditingDate => app.t("wizard.date"),
            InputMode::EditingReview | InputMode::CreatingReview => app.t("wizard.review"),
            InputMode::EditingPomodoro => app.t("hint.edit"),
            InputMode::ConfirmingDelete => app.t("hint.del"),
            InputMode::Filtering => app.t("section.tasks"), // Or a specific filter key
            _ => "",
        };

        // Context for Wizard / Confirmation
        let mut text = match app.input_mode {
            InputMode::MenuSettings => {
                let area = crate::ui::centered_rect(60, 40, f.area());
                super::settings_menu::draw(f, app, area);
                return;
            }
            InputMode::MenuSync => {
                let area = crate::ui::centered_rect(60, 40, f.area());
                super::sync_menu::draw(f, app, area);
                return;
            }
            InputMode::ConfirmingDelete => {
                vec![Line::from(vec![
                    Span::raw(format!("{} ", app.t("confirm.delete"))),
                    Span::styled(app.current_task().map_or("", |t| &t.title), Style::default().fg(theme.error).add_modifier(Modifier::BOLD))
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::raw(format!("{} ", app.t("confirm.press"))),
                    Span::styled("[y]", Style::default().fg(theme.success).add_modifier(Modifier::BOLD)),
                    Span::raw(format!(" {} {} ", app.t("confirm.for_yes"), app.t("confirm.yes"))),
                    Span::raw(format!("{} ", app.t("confirm.for_no"))),
                    Span::styled("[n/Esc]", Style::default().fg(theme.muted).add_modifier(Modifier::BOLD)),
                    Span::raw(format!(" {} {}.", app.t("confirm.for_no"), app.t("confirm.no"))),
                ])]
            }
            _ => {
                let cursor = if app.frame_count % 2 == 0 { "_" } else { " " };
                vec![Line::from(vec![
                    Span::styled(app.input_buffer.as_str(), Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                    Span::styled(cursor, Style::default().fg(theme.accent).add_modifier(Modifier::BOLD))
                ])]
            }
        };
        
        match app.input_mode {
            InputMode::CreatingDescription => {
                text.insert(0, Line::from(vec![Span::raw(format!("{}: ", app.t("wizard.title"))), Span::styled(&app.wizard_title, Style::default().fg(theme.accent))]));
                text.insert(1, Line::from(""));
            }
            InputMode::CreatingDate => {
                text.insert(0, Line::from(vec![Span::raw(format!("{}: ", app.t("wizard.title"))), Span::styled(&app.wizard_title, Style::default().fg(theme.accent))]));
                text.insert(1, Line::from(vec![Span::raw(format!("{}: ", app.t("wizard.desc"))), Span::styled(&app.wizard_desc, Style::default().fg(theme.muted))]));
                text.insert(2, Line::from(""));
            }
            InputMode::CreatingReview => {
                text.insert(0, Line::from(vec![Span::raw(format!("{}: ", app.t("wizard.title"))), Span::styled(&app.wizard_title, Style::default().fg(theme.accent))]));
                text.insert(1, Line::from(vec![Span::raw(format!("{}: ", app.t("wizard.desc"))), Span::styled(&app.wizard_desc, Style::default().fg(theme.muted))]));
                text.insert(2, Line::from(vec![Span::raw(format!("{}: ", app.t("wizard.date"))), Span::styled(&app.wizard_date, Style::default().fg(theme.accent_secondary))]));
                text.insert(3, Line::from(""));
            }
            _ => {}
        }

        let area = crate::ui::centered_rect(60, 30, f.area());
        f.render_widget(Clear, area);
        f.render_widget(Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(title).border_style(Style::default().fg(theme.accent)))
            .style(Style::default().bg(theme.bg)), area);
    }

    if app.show_help {
        let area = crate::ui::centered_rect(50, 75, f.area());
        f.render_widget(Clear, area);
        let help_text = vec![
            format!(" --- {} ---", app.t("hint.switch")),
             format!(" \u{2191}\u{2193} (j/k)   : {}", app.t("hint.nav")),
             format!(" \u{2190}\u{2192} (Tab)   : {}", app.t("hint.switch")),
            " 1, 2, 3       : Jump to panel".to_string(),
            " ".to_string(),
            format!(" --- {} ---", app.t("section.tasks")),
             format!(" a             : {}", app.t("hint.add")),
             format!(" Space         : {}", app.t("hint.ok")),
             format!(" Backspace     : {}", app.t("hint.undo")),
             format!(" x             : {}", app.t("hint.del")),
             format!(" i             : Importance cycle"),
             format!(" o             : Sort cycle"),
             format!(" /             : {}", app.t("hint.search")),
             format!(" d             : {}", app.t("hint.desc")),
             format!(" t             : Date / Time"),
             format!(" e             : {}", app.t("hint.edit")),
             format!(" r             : {}", app.t("section.review_plan")),
            " ".to_string(),
            format!(" --- {} ---", app.t("section.pomodoro")),
             format!(" p             : Play/Pause"),
             format!(" S (Shift+s)   : Skip phase"),
             format!(" R (Shift+r)   : Reset Pomodoro"),
             format!(" f             : Force Break"),
            " ".to_string(),
            format!(" --- {} ---", "SYSTEM"),
             format!(" c             : {}", app.t("hint.settings")),
             format!(" ?             : Toggle help"),
             format!(" q             : Quit"),
        ].join("\n");
        
        let help = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title(app.t("menu.help.title")).border_style(Style::default().fg(theme.accent)))
            .style(Style::default().bg(theme.bg).fg(theme.fg));
        f.render_widget(help, area);
    }
}


