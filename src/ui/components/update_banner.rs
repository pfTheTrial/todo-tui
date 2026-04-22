use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::Paragraph,
    Frame,
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    if let Some(ref info) = app.update_info {
        if info.has_update {
            let text = format!(
                " ✨ tdt v{} {} │ {}",
                info.latest,
                app.t("update.available"),
                app.t("update.press_key")
            );
            let banner = Paragraph::new(text).style(
                Style::default()
                    .bg(app.theme.accent)
                    .fg(app.theme.bg)
                    .add_modifier(Modifier::BOLD),
            );
            f.render_widget(banner, area);
        }
    }
}
