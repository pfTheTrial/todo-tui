use std::collections::HashMap;

pub fn get_strings() -> HashMap<&'static str, &'static str> {
    let mut s = HashMap::new();
    
    // Footer / Hints
    s.insert("hint.help", "[?]Ajuda");
    s.insert("hint.switch", "[←→]Trocar");
    s.insert("hint.nav", "[↑↓]Navegar");
    s.insert("hint.add", "[a]Add");
    s.insert("hint.ok", "[Space]Ok");
    s.insert("hint.undo", "[Bksp]Desfazer");
    s.insert("hint.desc", "[d]Desc");
    s.insert("hint.del", "[x]Del");
    s.insert("hint.pomo", "[p]Pomo");
    s.insert("hint.edit", "[e]Editar");
    s.insert("hint.theme", "[t]Tema");
    s.insert("hint.settings", "[c]Config");
    s.insert("hint.sync", "[s]Sync");
    s.insert("hint.search", "Buscar");
    s.insert("hint.quit", "Sair");
    
    // Sections
    s.insert("section.tasks", "Tarefas");
    s.insert("section.pomodoro", "Pomodoro");
    s.insert("section.info", "Informações");
    s.insert("section.description", "Descrição");
    s.insert("section.importance", "Importância");
    s.insert("section.review_plan", "Plano de Revisão");
    s.insert("section.status", "Status");
    
    // Categories
    s.insert("cat.overdue", "ATRASADAS");
    s.insert("cat.today", "HOJE");
    s.insert("cat.upcoming", "PRÓXIMOS DIAS");
    s.insert("cat.inbox", "INBOX");
    
    // Status
    s.insert("status.running", "RODANDO");
    s.insert("status.paused", "PAUSADO");
    s.insert("status.due", "ATRASADA");
    s.insert("status.scheduled", "Agendado");
    s.insert("status.done", "Concluído");
    
    // Pomodoro labels
    s.insert("pomo.profile", "Perfil");
    s.insert("pomo.phase", "Fase");
    s.insert("pomo.time", "Tempo");
    s.insert("pomo.status", "Status");
    s.insert("pomo.session", "Sessão");
    s.insert("pomo.break", "Pausa");
    s.insert("pomo.done_title", "Pomodoro!");
    s.insert("pomo.done_body", "Fase concluída! Hora de trocar.");
    
    // Sort labels
    s.insert("sort.prio", "Prio");
    s.insert("sort.date", "Data");
    s.insert("sort.name", "Nome");
    
    // Detail labels
    s.insert("detail.due", "Prazo");
    s.insert("detail.empty", "Selecione uma tarefa para ver detalhes");
    
    // Menus
    s.insert("menu.settings.title", " ⚙ Configurações ");
    s.insert("menu.settings.theme", "Tema");
    s.insert("menu.settings.notifications", "Notificações");
    s.insert("menu.settings.startup", "Iniciar c/ Windows");
    s.insert("menu.settings.language", "Idioma");
    
    // Settings sections
    s.insert("settings.section.appearance", "🎨 APARÊNCIA");
    s.insert("settings.section.system", "🔔 SISTEMA");
    s.insert("settings.section.integrations", "🔗 SYNC & INTEGRAÇÕES");
    s.insert("settings.section.actions", "📋 AÇÕES");
    s.insert("settings.section.update", "🔄 ATUALIZAÇÃO");
    
    // Sync
    s.insert("menu.sync.title", " Sync & Integrações ");
    s.insert("menu.sync.github", "GitHub");
    s.insert("menu.sync.gdrive", "Google Drive");
    s.insert("menu.sync.gcal", "Google Calendar");
    s.insert("menu.sync.notion", "Notion");
    s.insert("menu.sync.not_configured", "Não configurado");
    s.insert("menu.sync.configured", "Configurado");
    
    // Wizard
    s.insert("wizard.title", "Título da Tarefa");
    s.insert("wizard.desc", "Descrição");
    s.insert("wizard.date", "Data / Quando (ex: 3d, amanhã, seg)");
    s.insert("wizard.review", "Plano de Revisão (ex: d3 d5 d7)");
    
    // Confirmations
    s.insert("confirm.delete", "Deseja realmente deletar a tarefa:");
    s.insert("confirm.yes", "sim");
    s.insert("confirm.no", "não");
    s.insert("confirm.press", "Pressione");
    s.insert("confirm.for_yes", "para");
    s.insert("confirm.for_no", "para");
    s.insert("confirm.next", " Próximo");
    s.insert("confirm.prev", " Anterior");
    s.insert("confirm.skip", " Pular");

    // Importance levels
    s.insert("importance.urgent", "Urgente");
    s.insert("importance.high", "Alta");
    s.insert("importance.medium", "Média");
    s.insert("importance.low", "Baixa");
    
    // Settings menu extras
    s.insert("menu.settings.export", "📤 Exportar Tarefas (.xlsx)");
    s.insert("menu.settings.import", "📥 Importar Tarefas (.xlsx)");
    s.insert("menu.settings.update", "🔄 Atualizar tdt");
    s.insert("menu.settings.sync", "Sincronização");
    s.insert("menu.settings.notion_key", "Notion API Key");
    s.insert("menu.settings.notion_db", "Notion Database ID");
    s.insert("menu.settings.sync_interval", "Auto-Sync (min)");
    s.insert("menu.settings.export_done", "Exportado p/:");
    s.insert("menu.settings.import_path", "Caminho do arquivo .xlsx");
    s.insert("menu.settings.no_update", "✅ Versão atualizada!");
    s.insert("menu.settings.update_available", "Nova versão disponível:");
    
    // Update / Auto-update
    s.insert("update.available", "disponível!");
    s.insert("update.press_key", "Vá em Config para atualizar");
    s.insert("update.confirm", "Baixar e instalar v{}?");
    s.insert("update.downloading", "Baixando atualização...");
    s.insert("update.success", "✅ Atualizado! Reinicie o tdt.");
    s.insert("update.error", "Erro na atualização");
    s.insert("update.unsupported", "Auto-update não suportado nesta plataforma");
    
    // Messages
    s.insert("msg.imported", "tarefas importadas");
    s.insert("msg.update_error", "Erro ao verificar atualizações");
    s.insert("msg.update_checking", "Verificando atualizações...");
    s.insert("settings.npm_managed", "NPM Managed");
    s.insert("msg.npm_update", "Atulize usando: npm update -g tdt-cli");
    
    // Performance stats
    s.insert("settings.perf.title", "📊 Performance");
    
    // Notifications
    s.insert("notify.tasks_due_title", "Tarefas Pendentes");
    s.insert("notify.tasks_due_body", "Você tem {} tarefas a fazer hoje.");

    // Days text
    s.insert("days.ago", "atrás");
    s.insert("days.in", "em");

    // Welcome Screen
    s.insert("welcome.title_prefix", "Bem-Vindo — Slide");
    s.insert("welcome.splash.subtitle", "Bem-vindo ao seu novo hub de produtividade.");
    s.insert("welcome.splash.desc", "Todo-TUI é projetado para velocidade, foco offline e design limpo.");
    s.insert("welcome.splash.cta", "Pressione 'n' para começar o tour.");
    s.insert("welcome.layout.title", "O LAYOUT (Inspirado no Lazygit)");
    s.insert("welcome.layout.tasks", "[1] Tarefas  : Foque no que importa, organizado por prioridade.");
    s.insert("welcome.layout.detail", "[2] Detalhes : Todo o contexto que você precisa ao seu alcance.");
    s.insert("welcome.layout.pomo", "[3] Pomodoro : Mantenha o foco com timers integrados.");
    s.insert("welcome.layout.nav", "Troque painéis usando [Tab] ou teclas [1, 2, 3].");
    s.insert("welcome.review.title", "REVISÕES INTELIGENTES (SRS)");
    s.insert("welcome.review.line1", "Não apenas termine tarefas — domine-as.");
    s.insert("welcome.review.line2", "Use o sistema de revisão para agendar follow-ups periódicos");
    s.insert("welcome.review.line3", "automaticamente usando a sintaxe '1d 1s 1m'.");
    s.insert("welcome.review.line4", "Perfeito para aprendizado, manutenção e hábitos de longo prazo.");
    s.insert("welcome.cmd.title", "COMANDOS E ATALHOS");
    s.insert("welcome.cmd.search", " [/] Buscar  : Encontre qualquer tarefa instantaneamente.");
    s.insert("welcome.cmd.sort", " [o] Ordenar : Alterne entre Prioridade, Data ou Nome.");
    s.insert("welcome.cmd.add", " [a] Adicionar: Wizard de entrada rápida.");
    s.insert("welcome.cmd.config", " [c] Config  : Personalize profundamente sua experiência.");
    s.insert("welcome.cmd.help", " [?] Ajuda   : Todos os atalhos listados.");
    s.insert("welcome.int.title", "INTEGRAÇÕES");
    s.insert("welcome.int.line1", "Todo-TUI suporta sincronização com Notion.");
    s.insert("welcome.int.line2", "Vá em Configurações [c] para adicionar sua API Key e Database ID.");
    s.insert("welcome.int.sync_label", "Sync Atual: ");
    s.insert("welcome.ready.title", "TUDO PRONTO");
    s.insert("welcome.ready.line1", "Tudo está configurado.");
    s.insert("welcome.ready.line2", "Suas tarefas são armazenadas localmente em um arquivo JSON.");
    s.insert("welcome.ready.cta", "Pressione [n] para entrar no aplicativo.");
    s.insert("welcome.hint.next", "Próximo");
    s.insert("welcome.hint.finish", "Finalizar");
    s.insert("welcome.hint.prev", "Anterior");
    s.insert("welcome.hint.skip", "Pular");
    
    // Help menu title
    s.insert("menu.help.title", " MENU DE AJUDA ");

    s
}
