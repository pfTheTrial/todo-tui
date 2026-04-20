use ratatui::{
    layout::Rect,
    style::Style,
    widgets::Paragraph,
    Frame,
};
use crate::app::{App, InputMode, ActivePanel};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let hints = match app.input_mode {
        InputMode::Normal => {
            let global = format!("[?]Help [Tab]Next [c]Cfg [q]Quit");
            let contextual = match app.active_panel {
                ActivePanel::TaskList => format!(
                    "[↑↓]Nav [g/G]Top/End [a]Add [Spc]Ok [x]Del [e]Edit [d]Desc [r]Rev [i]Pri [Bsp]Undo"
                ),
                ActivePanel::TaskDetail => format!(
                    "[↑↓]Scroll [J/K]Tasks [e]Edit [d]Desc [r]Rev"
                ),
                ActivePanel::Pomodoro => format!(
                    "[↑↓]Profile [p]Play [S]Skip [R]Reset [f]Break [e]Edit"
                ),
            };
            format!(" {} │ {} ", global, contextual)
        }
        InputMode::EditingTitle => format!(" [Enter]{} [Esc]{} ({}) ", app.t("confirm.yes"), app.t("confirm.no"), app.t("wizard.title")),
        InputMode::EditingDescription => format!(" [Enter]{} [Esc]{} ({}) ", app.t("confirm.yes"), app.t("confirm.no"), app.t("wizard.desc")),
        InputMode::EditingDate => format!(" [Enter]{} [Esc]{} ({}) ", app.t("confirm.yes"), app.t("confirm.no"), app.t("wizard.date")),
        InputMode::EditingReview => format!(" [Enter]{} [Esc]{} ({}) ", app.t("confirm.yes"), app.t("confirm.no"), app.t("wizard.review")),
        InputMode::CreatingTitle => format!(" [Enter]{} [Esc]{} (Wizard 1/4) ", app.t("confirm.yes"), app.t("confirm.no")),
        InputMode::CreatingDescription => format!(" [Enter]{} [Esc]{} (Wizard 2/4) ", app.t("confirm.yes"), app.t("confirm.no")),
        InputMode::CreatingDate => format!(" [Enter]{} [Esc]{} (Wizard 3/4) ", app.t("confirm.yes"), app.t("confirm.no")),
        InputMode::CreatingReview => format!(" [Enter]{} [Esc]{} (Wizard 4/4) ", app.t("confirm.yes"), app.t("confirm.no")),
        InputMode::EditingPomodoro => format!(" [Enter]{} [Esc]{} (ex: NOME WORK SHORT LONG) ", app.t("confirm.yes"), app.t("confirm.no")),
        InputMode::ConfirmingDelete => format!(" [y]{} [n/Esc]{} ", app.t("confirm.yes"), app.t("confirm.no")),
        InputMode::MenuSettings => format!(" [↑↓]{} [←→/Space/Enter]{} [Esc]{} ", app.t("hint.nav"), app.t("hint.edit"), app.t("confirm.no")),
        InputMode::MenuSync => format!(" [↑↓]{} [Esc]{} ", app.t("hint.nav"), app.t("confirm.no")),
        InputMode::EditingNotionKey => format!(" [Enter]{} [Esc]{} (Notion Key) ", app.t("confirm.yes"), app.t("confirm.no")),
        InputMode::EditingNotionDatabase => format!(" [Enter]{} [Esc]{} (Notion DB) ", app.t("confirm.yes"), app.t("confirm.no")),
        InputMode::EditingSyncInterval => format!(" [Enter]{} [Esc]{} (Interval) ", app.t("confirm.yes"), app.t("confirm.no")),
        InputMode::Filtering => format!(" {}... [Enter/Esc] ", app.t("hint.search")),
        InputMode::ImportingExcel => format!(" [Enter]{} [Esc]{} (Excel Path) ", app.t("confirm.yes"), app.t("confirm.no")),
        InputMode::Onboarding => format!(" [n/Enter]{} [p]{} [Esc]{} ", app.t("confirm.next"), app.t("confirm.prev"), app.t("confirm.skip")),
    };
    let footer = Paragraph::new(hints).style(Style::default().bg(app.theme.footer_bg).fg(app.theme.footer_fg));
    f.render_widget(footer, area);
}
