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
    
    let args: Vec<String> = std::env::args().collect();
    let is_npm = std::env::current_exe()
        .map(|p| {
            let p_str = p.to_string_lossy().to_lowercase();
            p_str.contains("node_modules") || p_str.contains("npm") || p_str.contains("nvm")
        })
        .unwrap_or(false);

    if args.iter().any(|a| a == "update" || a == "--update") {
        if is_npm {
            println!("tdt foi instalado via NPM. Use: npm update -g tdt-cli");
            return Ok(());
        }
        println!("Verificando atualizações...");
        if let Some(info) = crate::utils::update::check_for_update() {
            if info.has_update {
                println!("Baixando v{}...", info.latest);
                match crate::utils::auto_update::perform_update(&info.latest) {
                    Ok(_) => println!("tdt atualizado com sucesso!"),
                    Err(e) => println!("Erro ao atualizar: {}", e),
                }
            } else {
                println!("Você já está na versão mais recente (v{}).", info.current);
            }
        } else {
            println!("Falha ao verificar atualizações.");
        }
        return Ok(());
    }

    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!("tdt — Gerenciador de Tarefas no Terminal");
        println!("Uso: tdt [CMD / OPTION]");
        println!("\n  update, --update    Sincroniza para a nova versão (Auto-updater)");
        println!("  -h, --help          Mostra a ajuda");
        println!("  -V, --version       Mostra a versão");
        return Ok(());
    }

    if args.iter().any(|a| a == "-V" || a == "--version") {
        println!("tdt v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    
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
    app.is_npm = is_npm;
    
    // Startup update check — background thread so UI doesn't block on network
    let pending_update: std::sync::Arc<std::sync::Mutex<Option<crate::utils::update::UpdateInfo>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    
    // Only auto-check if not installed by NPM (NPM users should use npm update)
    if !is_npm {
        let pending = pending_update.clone();
        std::thread::spawn(move || {
            if let Some(info) = crate::utils::update::check_for_update() {
                if let Ok(mut guard) = pending.lock() {
                    *guard = Some(info);
                }
            }
        });
    }

    // Run app
    let res = run_app(&mut terminal, &mut app, pending_update);

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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    pending_update: std::sync::Arc<std::sync::Mutex<Option<crate::utils::update::UpdateInfo>>>,
) -> io::Result<()> {
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
                    let title = app.t("pomo.done_title").to_string();
                    let body = app.t("pomo.done_body").to_string();
                    crate::utils::notifications::notify_pomodoro_finish(&title, &body);
                }
            }
            
            // Poll the background update check (non-blocking)
            if app.update_info.is_none() {
                if let Ok(mut guard) = pending_update.try_lock() {
                    if let Some(info) = guard.take() {
                        if info.has_update {
                            app.status_message = Some(format!(
                                "✨ {} v{}",
                                app.t("menu.settings.update_available"),
                                info.latest
                            ));
                        }
                        app.update_info = Some(info);
                    }
                }
            }

            // Sample own resource usage every 2 ticks (~2s)
            if app.frame_count % 2 == 0 {
                let (ram, cpu) = crate::utils::perf::sample_self();
                app.sys_ram_mb = ram;
                app.sys_cpu_pct = cpu;
            }

            last_tick = Instant::now();
        }
        app.frame_count = app.frame_count.wrapping_add(1);
    }
}
