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

    let mut force_lang = None;
    let mut custom_data_dir = None;
    let mut no_sync = false;
    let mut show_setup = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "update" | "--update" => {
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
                    println!("Falha ao verificar atualizações / Nenhuma atualização válida.");
                }
                return Ok(());
            }
            "sync" => {
                println!("Sincronização manual acionada... (Processo agendado)");
                return Ok(());
            }
            "reset" => {
                println!("Resetando configurações...");
                let data_dir = dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::from(".")).join("todo-tui");
                let settings_path = data_dir.join("settings.json");
                if settings_path.exists() {
                    let _ = std::fs::remove_file(settings_path);
                    println!("Configurações apagadas com sucesso.");
                } else {
                    println!("Nenhuma configuração encontrada.");
                }
                return Ok(());
            }
            "--uninstall" => {
                println!("Desinstalando TDT...");
                let data_dir = dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::from(".")).join("todo-tui");
                if data_dir.exists() {
                    let _ = std::fs::remove_dir_all(data_dir);
                    println!("✔ Diretório de dados removido com sucesso.");
                }
                if is_npm {
                    println!("✔ Execute 'npm uninstall -g tdt-cli' para remover o executável.");
                } else {
                    println!("✔ Exclua o arquivo executável para finalizar a desinstalação.");
                }
                return Ok(());
            }
            "--setup" | "--wizard" => show_setup = true,
            "--no-sync" => no_sync = true,
            "--lang" => {
                if i + 1 < args.len() {
                    force_lang = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--data-dir" => {
                if i + 1 < args.len() {
                    custom_data_dir = Some(std::path::PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "-h" | "--help" => {
                println!("USAGE:");
                println!("    tdt [OPTIONS] [COMMAND]\n");
                println!("COMMANDS:");
                println!("    update         Verifica e instala a última versão");
                println!("    sync           Força um sync manual e sai");
                println!("    reset          Reseta configurações (preserva tarefas)\n");
                println!("OPTIONS:");
                println!("    -h, --help           Mostra a ajuda completa");
                println!("    -V, --version        Mostra a versão atual");
                println!("        --setup          Abre o Setup Wizard");
                println!("        --lang <LANG>    Força idioma temporário (pt-br, en, es)");
                println!("        --data-dir <PATH> Diretório de dados customizado");
                println!("        --no-sync        Desativa sync automático nesta sessão");
                println!("        --uninstall      Desinstala o TDT (Deleta os arquivos)");
                return Ok(());
            }
            "-V" | "--version" => {
                println!("tdt v{}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            cmd => {
                if cmd.starts_with("-") {
                    println!("error: Option '{}' not recognized.", cmd);
                } else {
                    println!("error: Command '{}' not recognized.", cmd);
                }
                println!("\nUsage: tdt [OPTIONS] [COMMAND]");
                println!("For a list of commands and options, type 'tdt --help'");
                return Ok(());
            }
        }
        i += 1;
    }
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let data_dir = custom_data_dir.unwrap_or_else(|| {
        dirs::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("todo-tui")
    });
    
    let mut app = App::new(data_dir)?;
    app.is_npm = is_npm;
    
    if show_setup {
        app.input_mode = crate::app::InputMode::Onboarding;
        app.onboarding_index = 0;
    }
    if let Some(lang_str) = force_lang {
        let lang = match lang_str.to_lowercase().as_str() {
            "en" | "eng" => crate::i18n::Language::En,
            "es" | "spa" => crate::i18n::Language::Es,
            _ => crate::i18n::Language::PtBr,
        };
        app.settings.language = lang;
        app.i18n = crate::i18n::I18n::new(lang);
    }
    if no_sync {
        println!("--no-sync: Ação registrada (sync automático não está implementado ainda).");
    }

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

            // Daily Task Digest
            if app.settings.task_reminders_enabled {
                let today_str = chrono::Local::now().format("%Y-%m-%d").to_string();
                if app.settings.last_daily_digest.as_deref() != Some(&today_str) {
                    let due_count = app.tasks.iter().filter(|t| !t.completed && t.effective_date().map_or(false, |dt| dt.with_timezone(&chrono::Local).date_naive() <= chrono::Local::now().date_naive())).count();
                    
                    if due_count > 0 {
                        let title = app.t("notify.tasks_due_title").to_string();
                        let body = app.t("notify.tasks_due_body").replace("{}", &due_count.to_string());
                        crate::utils::notifications::notify_tasks_due(&title, &body);
                    }
                    
                    app.settings.last_daily_digest = Some(today_str);
                    let _ = app.save_settings();
                }
            }

            last_tick = Instant::now();
        }
        app.frame_count = app.frame_count.wrapping_add(1);
    }
}
