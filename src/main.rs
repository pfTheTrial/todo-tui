use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{io, time::{Duration, Instant}};

mod app;
mod i18n;
mod model;
mod storage;
mod ui;
mod utils;
mod integrations;
mod input;

use crate::app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("todo-tui");
    let mut app = App::new(data_dir)?;
    
    // Startup update check
    if let Some(info) = crate::utils::update::check_for_update() {
        if info.has_update {
            app.status_message = Some(format!("✨ Nova versão disponível! (v{})", info.latest));
        }
        app.update_info = Some(info);
    }

    // Run app
    let res = run_app(&mut terminal, &mut app);

    // Final Sync & Save on Exit
    let _ = app.save();

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(1000);
    let mut last_tick = Instant::now();
    
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if input::handle_key(app, key) {
                        return Ok(());
                    }
                }
            }
        }
        
        // Handle periodic sync and pomodoro
        if last_tick.elapsed() >= tick_rate {
            if app.pomodoro_tick() {
                if app.settings.notifications_enabled {
                    let phase_name = format!("{:?}", app.pomodoro.phase);
                    crate::utils::notifications::notify_pomodoro_finish(&phase_name);
                }
            }
            
            // TODO: Phase 7 — aqui entrará a lógica real de sync periódico
            // if app.settings.sync_interval_mins > 0 && app.last_sync.elapsed() >= ... { sync_all_providers(&app); }

            last_tick = Instant::now();
        }
        app.frame_count = app.frame_count.wrapping_add(1);
    }
}
