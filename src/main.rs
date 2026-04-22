use clap::{Parser, Subcommand};
use color_eyre::Result;
use crossterm::{
    cursor::Show,
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    io,
    path::PathBuf,
    time::{Duration, Instant},
};

mod app;
mod i18n;
mod input;
mod integrations;
mod model;
mod storage;
mod ui;
mod utils;

use crate::app::App;

#[derive(Debug, Parser)]
#[command(name = "tdt", version, about = "Terminal Task Dashboard")]
struct Cli {
    #[arg(long, help = "Abre o Setup Wizard")]
    setup: bool,
    #[arg(long = "wizard", hide = true)]
    wizard: bool,
    #[arg(
        long,
        value_name = "LANG",
        help = "Força idioma temporário (pt-br, en, es)"
    )]
    lang: Option<String>,
    #[arg(long, value_name = "PATH", help = "Diretório de dados customizado")]
    data_dir: Option<PathBuf>,
    #[arg(long, help = "Desativa sync automático nesta sessão")]
    no_sync: bool,
    #[arg(long, help = "Desativa verificação automática de update nesta sessão")]
    no_update_check: bool,
    #[arg(long, help = "Confirma comandos destrutivos ou de atualização")]
    yes: bool,
    #[arg(long, hide = true)]
    uninstall: bool,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Verifica e instala a última versão")]
    Update,
    #[command(about = "Força sync manual e sai")]
    Sync {
        #[arg(
            long,
            help = "Mostra o que seria sincronizado sem chamar integrações externas"
        )]
        dry_run: bool,
        #[arg(
            long,
            help = "Puxa tarefas do Notion em vez de enviar alterações locais"
        )]
        pull: bool,
        #[arg(long, help = "Zera o backoff da fila e reprocessa agora")]
        retry_now: bool,
    },
    #[command(about = "Reseta configurações, preservando tarefas")]
    Reset,
    #[command(about = "Remove dados locais do TDT")]
    Uninstall,
}

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let is_npm = std::env::current_exe()
        .map(|p| {
            let p_str = p.to_string_lossy().to_lowercase();
            p_str.contains("node_modules") || p_str.contains("npm") || p_str.contains("nvm")
        })
        .unwrap_or(false);

    let data_dir = data_dir(cli.data_dir.clone());
    let command = if cli.uninstall {
        Some(Command::Uninstall)
    } else {
        cli.command
    };

    if let Some(command) = command {
        return run_command(command, &data_dir, cli.yes, is_npm);
    }

    let _guard = TerminalGuard::enter()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(data_dir)?;
    app.is_npm = is_npm;

    if cli.setup || cli.wizard {
        app.input_mode = crate::app::InputMode::Onboarding;
        app.onboarding_index = 0;
    }
    if let Some(lang_str) = cli.lang {
        let lang = parse_language(&lang_str);
        app.settings.language = lang;
        app.i18n = crate::i18n::I18n::new(lang);
    }
    app.settings.sync_enabled = !cli.no_sync;

    // Startup update check — background thread so UI doesn't block on network
    let pending_update: std::sync::Arc<std::sync::Mutex<Option<crate::utils::update::UpdateInfo>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));

    // Only auto-check if not installed by NPM (NPM users should use npm update)
    if !is_npm && !cli.no_update_check {
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
    drop(terminal);
    drop(_guard);

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn data_dir(custom: Option<PathBuf>) -> PathBuf {
    custom.unwrap_or_else(|| {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("todo-tui")
    })
}

fn parse_language(lang: &str) -> crate::i18n::Language {
    match lang.to_lowercase().as_str() {
        "en" | "eng" => crate::i18n::Language::En,
        "es" | "spa" => crate::i18n::Language::Es,
        _ => crate::i18n::Language::PtBr,
    }
}

fn run_command(
    command: Command,
    data_dir: &std::path::Path,
    yes: bool,
    is_npm: bool,
) -> Result<()> {
    match command {
        Command::Update => run_update(yes, is_npm),
        Command::Sync {
            dry_run,
            pull,
            retry_now,
        } => run_sync(data_dir.to_path_buf(), dry_run, pull, retry_now),
        Command::Reset => run_reset(data_dir),
        Command::Uninstall => run_uninstall(data_dir, yes, is_npm),
    }
}

fn run_update(yes: bool, is_npm: bool) -> Result<()> {
    if is_npm {
        println!("tdt foi instalado via NPM. Use: npm update -g todo-tui");
        return Ok(());
    }
    println!("Verificando atualizações...");
    if let Some(info) = crate::utils::update::check_for_update() {
        if info.has_update {
            if !yes {
                println!(
                    "Update v{} disponível. Rode `tdt --yes update` para baixar e instalar.",
                    info.latest
                );
                return Ok(());
            }
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
    Ok(())
}

fn run_sync(data_dir: PathBuf, dry_run: bool, pull: bool, retry_now: bool) -> Result<()> {
    let mut app = App::new(data_dir)?;
    if dry_run {
        let configured =
            app.settings.notion_api_key.is_some() && app.settings.notion_database_id.is_some();
        let queue = app.sync_queue_status(Some("notion"));
        println!(
            "Sync dry-run: {} tarefa(s) ativa(s); Notion {}; modo {}; fila {} pend./{} retry/{} falhas.",
            app.filtered_task_count(),
            if configured {
                "configurado"
            } else {
                "não configurado"
            },
            if pull { "pull" } else { "push" },
            queue.pending,
            queue.retrying,
            queue.failed
        );
        return Ok(());
    }
    if retry_now {
        app.retry_all_sync_jobs_now(Some("notion"));
    }
    if pull {
        match app.pull_from_notion() {
            Ok(count) => println!("Pull concluído: {} tarefa(s) reconciliada(s).", count),
            Err(err) => println!("Pull não executado: {}", err),
        }
        return Ok(());
    }
    match app.sync_all_notion() {
        Ok(summary) => println!(
            "Sincronização concluída: {} enviada(s), {} deletada(s), {} puxada(s), {} ignorada(s), {} falha(s), {} enfileirada(s).",
            summary.pushed, summary.deleted, summary.pulled, summary.skipped, summary.failed, summary.queued
        ),
        Err(err) => println!("Sincronização não executada: {}", err),
    }
    Ok(())
}

fn run_reset(data_dir: &std::path::Path) -> Result<()> {
    println!("Resetando configurações em {}...", data_dir.display());
    let settings_path = data_dir.join("pomodoro.json");
    if settings_path.exists() {
        std::fs::remove_file(settings_path)?;
        println!("Configurações apagadas com sucesso. Tarefas preservadas.");
    } else {
        println!("Nenhuma configuração encontrada.");
    }
    Ok(())
}

fn run_uninstall(data_dir: &std::path::Path, yes: bool, is_npm: bool) -> Result<()> {
    let data_dir = data_dir
        .canonicalize()
        .unwrap_or_else(|_| data_dir.to_path_buf());
    if !yes {
        println!("Isto removerá os dados locais em: {}", data_dir.display());
        println!("Rode `tdt --yes uninstall` para confirmar.");
        return Ok(());
    }
    if data_dir.exists() {
        std::fs::remove_dir_all(&data_dir)?;
        println!("Diretório de dados removido: {}", data_dir.display());
    } else {
        println!(
            "Nenhum diretório de dados encontrado em: {}",
            data_dir.display()
        );
    }
    if is_npm {
        println!("Execute `npm uninstall -g todo-tui` para remover o executável.");
    } else {
        println!("Exclua o arquivo executável para finalizar a desinstalação.");
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
                if key.kind == KeyEventKind::Press && input::handle_key(app, key) {
                    return Ok(());
                }
            }
        }

        // Handle periodic sync and pomodoro
        if last_tick.elapsed() >= tick_rate {
            if app.pomodoro_tick() && app.settings.notifications_enabled {
                let title = app.t("pomo.done_title").to_string();
                let body = app.t("pomo.done_body").to_string();
                crate::utils::notifications::notify_pomodoro_finish(&title, &body);
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
            if app.frame_count.is_multiple_of(2) {
                let (ram, cpu) = crate::utils::perf::sample_self();
                app.sys_ram_mb = ram;
                app.sys_cpu_pct = cpu;
                if app.settings.sync_enabled {
                    if let Ok(true) = app.reload_tasks_if_external_change() {
                        app.status_message = Some("tasks.json recarregado do disco.".to_string());
                    }
                }
            }

            // Daily Task Digest
            if app.settings.task_reminders_enabled {
                let today_str = chrono::Local::now().format("%Y-%m-%d").to_string();
                if app.settings.last_daily_digest.as_deref() != Some(&today_str) {
                    let due_count = app
                        .tasks
                        .iter()
                        .filter(|t| {
                            !t.completed
                                && !t.is_deleted()
                                && t.effective_date().is_some_and(|dt| {
                                    dt.with_timezone(&chrono::Local).date_naive()
                                        <= chrono::Local::now().date_naive()
                                })
                        })
                        .count();

                    if due_count > 0 {
                        let title = app.t("notify.tasks_due_title").to_string();
                        let body = app
                            .t("notify.tasks_due_body")
                            .replace("{}", &due_count.to_string());
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
