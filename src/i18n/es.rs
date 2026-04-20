use std::collections::HashMap;

pub fn get_strings() -> HashMap<&'static str, &'static str> {
    let mut s = HashMap::new();
    
    s.insert("hint.help", "[?]Ayuda");
    s.insert("hint.switch", "[←→]Cambiar");
    s.insert("hint.nav", "[↑↓]Navegar");
    s.insert("hint.add", "[a]Add");
    s.insert("hint.ok", "[Space]Ok");
    s.insert("hint.undo", "[Bksp]Deshacer");
    s.insert("hint.desc", "[d]Desc");
    s.insert("hint.del", "[x]Del");
    s.insert("hint.pomo", "[p]Pomo");
    s.insert("hint.edit", "[e]Editar");
    s.insert("hint.theme", "[t]Tema");
    s.insert("hint.settings", "[c]Config");
    s.insert("hint.sync", "[s]Sync");
    s.insert("hint.search", "Buscar");
    
    s.insert("section.tasks", "Tareas");
    s.insert("section.pomodoro", "Pomodoro");
    s.insert("section.info", "Info");
    s.insert("section.description", "Descripción");
    s.insert("section.importance", "Importancia");
    s.insert("section.review_plan", "Plan de Revisión");
    
    s.insert("cat.overdue", "ATRASADAS");
    s.insert("cat.today", "HOY");
    s.insert("cat.upcoming", "PRÓXIMOS DÍAS");
    s.insert("cat.inbox", "INBOX");
    
    s.insert("status.running", "EJECUTANDO");
    s.insert("status.paused", "PAUSADO");
    s.insert("status.due", "ATRASADA");
    s.insert("status.scheduled", "Programado");
    s.insert("status.done", "Hecho");
    s.insert("section.status", "Estado");
    
    s.insert("menu.settings.title", " Configuración ");
    s.insert("menu.settings.theme", "Tema");
    s.insert("menu.settings.notifications", "Notificaciones");
    s.insert("menu.settings.startup", "Iniciar con Windows");
    s.insert("menu.settings.language", "Idioma");
    
    s.insert("menu.sync.title", " Sincronización ");
    s.insert("menu.sync.github", "GitHub");
    s.insert("menu.sync.gdrive", "Google Drive");
    s.insert("menu.sync.gcal", "Google Calendar");
    s.insert("menu.sync.notion", "Notion");
    s.insert("menu.sync.not_configured", "No configurado");
    s.insert("menu.sync.configured", "Configurado");
    
    s.insert("wizard.title", "Título de la Tarea");
    s.insert("wizard.desc", "Descripción");
    s.insert("wizard.date", "Fecha / Cuándo (ej: 3d, mañana, lun)");
    s.insert("wizard.review", "Plan de Revisión (ej: d3 d5 d7)");
    
    s.insert("confirm.delete", "¿Realmente deseja eliminar la tarea?");
    s.insert("confirm.yes", "si");
    s.insert("confirm.no", "no");
    s.insert("confirm.press", "Presione");
    s.insert("confirm.for_yes", "para");
    s.insert("confirm.for_no", "para");
    s.insert("confirm.next", " Siguiente");
    s.insert("confirm.prev", " Anterior");
    s.insert("confirm.skip", " Omitir");
    
    s.insert("importance.urgent", "Urgente");
    s.insert("importance.high", "Alta");
    s.insert("importance.medium", "Media");
    s.insert("importance.low", "Baja");
    
    s.insert("detail.empty", "Seleccione una tarea para ver detalles");
    
    s.insert("menu.settings.export", "📤 Exportar Tareas (.xlsx)");
    s.insert("menu.settings.import", "📥 Importar Tareas (.xlsx)");
    s.insert("menu.settings.update", "🔄 Comprobar Actualizaciones");
    s.insert("menu.settings.sync", "🔁 Menú Sincronización");
    s.insert("menu.settings.notion_key", "Clave API Notion");
    s.insert("menu.settings.notion_db", "ID Base de Datos Notion");
    s.insert("menu.settings.sync_interval", "Intervalo Auto-Sync (min)");
    s.insert("menu.settings.export_done", "Exportado a:");
    s.insert("menu.settings.import_path", "Ruta del archivo .xlsx");
    s.insert("menu.settings.no_update", "¡Versión actualizada!");
    s.insert("menu.settings.update_available", "Nueva versión disponible:");
    
    // Days text
    s.insert("days.ago", "hace");
    s.insert("days.in", "en");

    // Welcome Screen
    s.insert("welcome.title_prefix", "Bienvenido — Diapositiva");
    s.insert("welcome.splash.subtitle", "Bienvenido a tu nuevo hub de productividad.");
    s.insert("welcome.splash.desc", "Todo-TUI está diseñado para velocidad, enfoque offline y diseño limpio.");
    s.insert("welcome.splash.cta", "Presiona 'n' para comenzar el tour.");
    s.insert("welcome.layout.title", "EL LAYOUT (Inspirado en Lazygit)");
    s.insert("welcome.layout.tasks", "[1] Tareas   : Enfócate en lo que importa, organizado por prioridad.");
    s.insert("welcome.layout.detail", "[2] Detalles : Todo el contexto que necesitas de un vistazo.");
    s.insert("welcome.layout.pomo", "[3] Pomodoro : Mantén el enfoque con temporizadores integrados.");
    s.insert("welcome.layout.nav", "Cambia paneles usando [Tab] o las teclas [1, 2, 3].");
    s.insert("welcome.review.title", "REVISIONES INTELIGENTES (SRS)");
    s.insert("welcome.review.line1", "No solo termines tareas — domínalas.");
    s.insert("welcome.review.line2", "Usa el sistema de revisión para programar seguimientos periódicos");
    s.insert("welcome.review.line3", "automáticamente usando la sintaxis '1d 1s 1m'.");
    s.insert("welcome.review.line4", "Perfecto para aprendizaje, mantenimiento y hábitos a largo plazo.");
    s.insert("welcome.cmd.title", "COMANDOS Y ATAJOS");
    s.insert("welcome.cmd.search", " [/] Buscar  : Encuentra cualquier tarea al instante.");
    s.insert("welcome.cmd.sort", " [o] Ordenar : Alterna entre Prioridad, Fecha o Nombre.");
    s.insert("welcome.cmd.add", " [a] Agregar : Asistente de entrada rápida.");
    s.insert("welcome.cmd.config", " [c] Config  : Personaliza profundamente tu experiencia.");
    s.insert("welcome.cmd.help", " [?] Ayuda   : Todos los atajos listados.");
    s.insert("welcome.int.title", "INTEGRACIONES");
    s.insert("welcome.int.line1", "Todo-TUI soporta sincronización con Notion.");
    s.insert("welcome.int.line2", "Ve a Configuraciones [c] para agregar tu API Key y Database ID.");
    s.insert("welcome.int.sync_label", "Sync Actual: ");
    s.insert("welcome.ready.title", "TODO LISTO");
    s.insert("welcome.ready.line1", "Todo está configurado.");
    s.insert("welcome.ready.line2", "Tus tareas se almacenan localmente en un archivo JSON.");
    s.insert("welcome.ready.cta", "Presiona [n] para entrar a la aplicación.");
    s.insert("welcome.hint.next", "Siguiente");
    s.insert("welcome.hint.finish", "Finalizar");
    s.insert("welcome.hint.prev", "Anterior");
    s.insert("welcome.hint.skip", "Omitir");
    
    // Help menu title
    s.insert("menu.help.title", " MENÚ DE AYUDA ");

    s
}
