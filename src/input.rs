use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{App, InputMode, ActivePanel};

pub fn handle_key(app: &mut App, key: KeyEvent) -> bool {
    match app.input_mode {
        InputMode::Normal => handle_normal(app, key),
        InputMode::EditingTitle => handle_text_input(app, key, |a, text| {
            a.set_current_task_title(text);
        }),
        InputMode::EditingDescription => handle_text_input(app, key, |a, text| {
            a.set_current_task_description(text);
        }),
        InputMode::EditingReview => handle_text_input(app, key, |a, text| {
            a.set_custom_review(text);
        }),
        InputMode::EditingPomodoro => handle_text_input(app, key, |a, text| {
            a.edit_pomodoro_profile(text);
        }),
        InputMode::EditingDate => handle_text_input(app, key, |a, text| {
            if let Some(task) = a.current_task_mut() {
                task.due_date = crate::utils::parse_date_input(&text);
                task.generate_review_subtasks();
            }
            a.sort_tasks();
            let _ = a.save();
        }),
        InputMode::ConfirmingDelete => handle_confirm_delete(app, key),
        InputMode::CreatingTitle => handle_creating_title(app, key),
        InputMode::CreatingDescription => handle_creating_description(app, key),
        InputMode::CreatingDate => handle_creating_date(app, key),
        InputMode::CreatingReview => handle_creating_review(app, key),
        InputMode::MenuSettings => handle_menu_settings(app, key),
        InputMode::MenuSync => handle_menu_sync(app, key),
        InputMode::Filtering => handle_filtering(app, key),
        InputMode::Onboarding => handle_onboarding(app, key),
        InputMode::EditingNotionKey => handle_text_input(app, key, |a, text| {
            a.settings.notion_api_key = if text.is_empty() { None } else { Some(text) };
            let _ = a.save_settings();
            // Chain to DB ID next
            a.input_buffer = a.settings.notion_database_id.clone().unwrap_or_default();
            a.input_mode = InputMode::EditingNotionDatabase;
        }),
        InputMode::EditingNotionDatabase => handle_text_input(app, key, |a, text| {
            a.settings.notion_database_id = if text.is_empty() { None } else { Some(text) };
            let _ = a.save_settings();
            // Chain to sync interval next
            a.input_buffer = a.settings.sync_interval_mins.to_string();
            a.input_mode = InputMode::EditingSyncInterval;
        }),
        InputMode::EditingSyncInterval => handle_sync_interval_input(app, key),
        InputMode::ConfirmingUpdate => handle_confirm_update(app, key),
    }
}

fn handle_normal(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('q') => return true,
        KeyCode::Char('?') => app.show_help = !app.show_help,
        KeyCode::Esc => {
            if app.show_help {
                app.show_help = false;
            }
        }
        KeyCode::Char('j') | KeyCode::Down => {
            match app.active_panel {
                ActivePanel::Pomodoro => app.next_pomodoro_profile(),
                ActivePanel::TaskDetail => app.scroll_detail_down(),
                _ => app.next_task(),
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            match app.active_panel {
                ActivePanel::Pomodoro => app.prev_pomodoro_profile(),
                ActivePanel::TaskDetail => app.scroll_detail_up(),
                _ => app.previous_task(),
            }
        }
        KeyCode::Char('J') => { app.next_task(); app.detail_scroll_offset = 0; }
        KeyCode::Char('K') => { app.previous_task(); app.detail_scroll_offset = 0; }
        KeyCode::Char('g') => { app.go_to_first_task(); app.detail_scroll_offset = 0; }
        KeyCode::Char('G') => { app.go_to_last_task(); app.detail_scroll_offset = 0; }
        KeyCode::Tab | KeyCode::Char('l') | KeyCode::Right => app.next_panel(),
        KeyCode::BackTab | KeyCode::Char('h') | KeyCode::Left => app.previous_panel(),
        KeyCode::Char('1') => app.active_panel = ActivePanel::TaskList,
        KeyCode::Char('2') => app.active_panel = ActivePanel::Pomodoro,
        KeyCode::Char('3') => app.active_panel = ActivePanel::TaskDetail,
        KeyCode::Char('/') => {
            app.input_mode = InputMode::Filtering;
            app.input_buffer = app.filter_text.clone();
        }
        KeyCode::Char('o') => app.cycle_sort(),
        KeyCode::Char('x') => {
            if !app.tasks.is_empty() {
                app.input_mode = InputMode::ConfirmingDelete;
            }
        }
        KeyCode::Char(' ') => app.toggle_task_completion(),
        KeyCode::Backspace => app.untoggle_task_completion(),
        KeyCode::Char('p') => app.toggle_pomodoro(),
        KeyCode::Char('S') => app.skip_pomodoro_phase(),
        KeyCode::Char('R') => app.reset_pomodoro_timer(),
        KeyCode::Char('f') => app.force_pomodoro_break(),
        KeyCode::Char('c') => {
            app.input_mode = InputMode::MenuSettings;
            app.menu_cursor = 0;
        }
        KeyCode::Char('r') => {
            if let Some(task) = app.current_task() {
                app.input_buffer = task.custom_review_str.clone();
                app.input_mode = InputMode::EditingReview;
            }
        }
        KeyCode::Char('i') => app.toggle_importance(),
        KeyCode::Char('a') => {
            app.input_mode = InputMode::CreatingTitle;
            app.input_buffer.clear();
        }
        KeyCode::Char('e') => {
            if app.active_panel == ActivePanel::TaskList {
                if let Some(task) = app.current_task() {
                    app.input_buffer = task.title.clone();
                    app.input_mode = InputMode::EditingTitle;
                }
            } else if app.active_panel == ActivePanel::TaskDetail {
                if let Some(task) = app.current_task() {
                    app.input_buffer = task.description.clone();
                    app.input_mode = InputMode::EditingDescription;
                }
            } else if app.active_panel == ActivePanel::Pomodoro {
                if let Some(p) = app.pomodoro.profiles.get(app.pomodoro.active_profile_index) {
                    app.input_buffer = format!("{} {} {} {}", p.name, p.work_duration, p.short_break, p.long_break);
                }
                app.input_mode = InputMode::EditingPomodoro;
            }
        }
        KeyCode::Char('d') => {
            if let Some(task) = app.current_task() {
                app.input_buffer = task.description.clone();
                app.input_mode = InputMode::EditingDescription;
            }
        }
        KeyCode::Char('t') => {
            if let Some(task) = app.current_task() {
                app.input_buffer = match task.due_date {
                    Some(dt) => dt.with_timezone(&chrono::Local).format("%d/%m/%Y").to_string(),
                    None => String::new(),
                };
                app.input_mode = InputMode::EditingDate;
            }
        }
        _ => {}
    }
    false
}

fn handle_text_input(app: &mut App, key: KeyEvent, on_confirm: impl FnOnce(&mut App, String)) -> bool {
    match key.code {
        KeyCode::Enter => {
            let text: String = app.input_buffer.drain(..).collect();
            on_confirm(app, text);
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => { app.input_buffer.pop(); }
        KeyCode::Esc => {
            app.input_buffer.clear();
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
    false
}

fn handle_confirm_delete(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.delete_task();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
    false
}

fn handle_creating_title(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Enter => {
            if !app.input_buffer.is_empty() {
                app.wizard_title = app.input_buffer.drain(..).collect();
                app.input_mode = InputMode::CreatingDescription;
            }
        }
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => { app.input_buffer.pop(); }
        KeyCode::Esc => { app.input_mode = InputMode::Normal; }
        _ => {}
    }
    false
}

fn handle_creating_description(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Enter => {
            app.wizard_desc = app.input_buffer.drain(..).collect();
            app.input_mode = InputMode::CreatingDate;
        }
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => { app.input_buffer.pop(); }
        KeyCode::Esc => { app.input_mode = InputMode::Normal; }
        _ => {}
    }
    false
}

fn handle_creating_date(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Enter => {
            app.wizard_date = app.input_buffer.drain(..).collect();
            app.input_mode = InputMode::CreatingReview;
        }
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => { app.input_buffer.pop(); }
        KeyCode::Esc => { app.input_mode = InputMode::Normal; }
        _ => {}
    }
    false
}

fn handle_creating_review(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Enter => {
            let rev: String = app.input_buffer.drain(..).collect();
            app.add_task_full(app.wizard_title.clone(), app.wizard_desc.clone(), app.wizard_date.clone(), rev);
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => { app.input_buffer.pop(); }
        KeyCode::Esc => { app.input_mode = InputMode::Normal; }
        _ => {}
    }
    false
}

// Settings menu indices (must match settings_menu.rs draw order):
// 0: Theme  1: Language  2: Notifications  3: Startup
// 4: Sync & Integrations (→ sync submenu)
// 5: Update tdt
// 6: 📊 Performance (read-only display)
fn handle_menu_settings(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc => {
            app.status_message = None;
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.menu_cursor > 0 { app.menu_cursor -= 1; }
            else { app.menu_cursor = crate::ui::components::settings_menu::SETTINGS_ITEM_COUNT - 1; }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.menu_cursor = (app.menu_cursor + 1) % crate::ui::components::settings_menu::SETTINGS_ITEM_COUNT;
        }
        KeyCode::Left | KeyCode::Right | KeyCode::Char('h') | KeyCode::Char('l') | KeyCode::Enter | KeyCode::Char(' ') => {
            match app.menu_cursor {
                // 🎨 APPEARANCE
                0 => app.next_theme(),
                1 => {
                    let langs = crate::i18n::Language::all();
                    let current_idx = langs.iter().position(|l| *l == app.settings.language).unwrap_or(0);
                    let next_idx = (current_idx + 1) % langs.len();
                    app.set_language(langs[next_idx]);
                }
                2 => {
                    app.settings.notifications_enabled = !app.settings.notifications_enabled;
                    let _ = app.save_settings();
                }
                3 => {
                    app.settings.task_reminders_enabled = !app.settings.task_reminders_enabled;
                    let _ = app.save_settings();
                }
                4 => {
                    app.settings.startup_with_windows = !app.settings.startup_with_windows;
                    let _ = app.save_settings();
                }
                // 🔗 SYNC & INTEGRATIONS — open sync submenu
                5 => {
                    app.input_mode = InputMode::MenuSync;
                    app.menu_cursor = 0;
                }
                // 🔄 UPDATE
                6 => {
                    if app.is_npm {
                        app.status_message = Some(app.t("msg.npm_update").to_string());
                    } else if let Some(ref info) = app.update_info {
                        if info.has_update {
                            app.input_mode = InputMode::ConfirmingUpdate;
                        } else {
                            app.status_message = Some(app.t("menu.settings.no_update").to_string());
                        }
                    } else {
                        app.status_message = Some(app.t("msg.update_checking").to_string());
                    }
                }
                // 📊 PERFORMANCE — read only
                7 => {}
                _ => {}
            }
        }
        _ => {}
    }
    false
}

fn handle_menu_sync(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.menu_cursor = 4; // return settings cursor to Sync item
            app.input_mode = InputMode::MenuSettings;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.menu_cursor > 0 { app.menu_cursor -= 1; }
            else { app.menu_cursor = 3; }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.menu_cursor = (app.menu_cursor + 1) % 4;
        }
        // When Notion row (3) is selected, Enter/Space opens sub-editing.
        // We cycle through: API Key → DB ID → Interval
        KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Right | KeyCode::Char('l') => {
            if app.menu_cursor == 3 {
                // Open Notion API Key editing
                app.input_buffer = app.settings.notion_api_key.clone().unwrap_or_default();
                app.input_mode = InputMode::EditingNotionKey;
            }
        }
        _ => {}
    }
    false
}

fn handle_confirm_update(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            let version = if let Some(ref info) = app.update_info {
                info.latest.clone()
            } else {
                app.status_message = Some(app.t("msg.update_error").to_string());
                app.input_mode = InputMode::MenuSettings;
                return false;
            };
            
            app.status_message = Some(app.t("update.downloading").to_string());
            
            match crate::utils::auto_update::perform_update(&version) {
                Ok(_v) => {
                    app.status_message = Some(format!("{}", app.t("update.success")));
                }
                Err(e) => {
                    if e == "unsupported_platform" {
                        app.status_message = Some(app.t("update.unsupported").to_string());
                    } else {
                        app.status_message = Some(format!("{}: {}", app.t("update.error"), e));
                    }
                }
            }
            app.input_mode = InputMode::MenuSettings;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.input_mode = InputMode::MenuSettings;
        }
        _ => {}
    }
    false
}

fn handle_sync_interval_input(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Enter => {
            if let Ok(val) = app.input_buffer.parse::<u32>() {
                app.settings.sync_interval_mins = val;
                let _ = app.save_settings();
            }
            app.input_buffer.clear();
            app.input_mode = InputMode::MenuSync;
            app.menu_cursor = 3; // back on Notion row
        }
        KeyCode::Char(c) if c.is_ascii_digit() => app.input_buffer.push(c),
        KeyCode::Backspace => { app.input_buffer.pop(); }
        KeyCode::Esc => {
            app.input_buffer.clear();
            app.input_mode = InputMode::MenuSync;
            app.menu_cursor = 3;
        }
        _ => {}
    }
    false
}

fn handle_filtering(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Enter => {
            app.filter_text = app.input_buffer.drain(..).collect();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char(c) => {
            app.input_buffer.push(c);
            app.filter_text = app.input_buffer.clone();
            app.selected_task_index = 0;
            app.task_list_state.select(Some(0));
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
            app.filter_text = app.input_buffer.clone();
            app.selected_task_index = 0;
            app.task_list_state.select(Some(0));
        }
        KeyCode::Esc => {
            app.input_buffer.clear();
            app.filter_text.clear();
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
    false
}

fn handle_onboarding(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('n') | KeyCode::Right | KeyCode::Enter => {
            if app.onboarding_index < 5 {
                app.onboarding_index += 1;
            } else {
                app.settings.is_first_run = false;
                app.input_mode = InputMode::Normal;
                let _ = app.save_settings();
            }
        }
        KeyCode::Char('p') | KeyCode::Left => {
            if app.onboarding_index > 0 {
                app.onboarding_index -= 1;
            }
        }
        KeyCode::Esc | KeyCode::Char('q') => {
            app.settings.is_first_run = false;
            app.input_mode = InputMode::Normal;
            let _ = app.save_settings();
        }
        _ => {}
    }
    false
}
