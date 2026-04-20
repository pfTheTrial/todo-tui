use std::collections::HashMap;

pub fn get_strings() -> HashMap<&'static str, &'static str> {
    let mut s = HashMap::new();
    
    s.insert("hint.help", "[?]Help");
    s.insert("hint.switch", "[←→]Switch");
    s.insert("hint.nav", "[↑↓]Nav");
    s.insert("hint.add", "[a]Add");
    s.insert("hint.ok", "[Space]Ok");
    s.insert("hint.undo", "[Bksp]Undo");
    s.insert("hint.desc", "[d]Desc");
    s.insert("hint.del", "[x]Del");
    s.insert("hint.pomo", "[p]Pomo");
    s.insert("hint.edit", "[e]Edit");
    s.insert("hint.theme", "[t]Theme");
    s.insert("hint.settings", "[c]Config");
    s.insert("hint.sync", "[s]Sync");
    s.insert("hint.search", "Search");
    
    s.insert("section.tasks", "Tasks");
    s.insert("section.pomodoro", "Pomodoro");
    s.insert("section.info", "Info");
    s.insert("section.description", "Description");
    s.insert("section.importance", "Importance");
    s.insert("section.review_plan", "Review Plan");
    
    s.insert("cat.overdue", "OVERDUE");
    s.insert("cat.today", "TODAY");
    s.insert("cat.upcoming", "UPCOMING");
    s.insert("cat.inbox", "INBOX");
    
    s.insert("status.running", "RUNNING");
    s.insert("status.paused", "PAUSED");
    s.insert("status.due", "DUE");
    s.insert("status.scheduled", "Scheduled");
    
    s.insert("menu.settings.title", " Settings ");
    s.insert("menu.settings.theme", "Theme");
    s.insert("menu.settings.notifications", "Notifications");
    s.insert("menu.settings.startup", "Start with Windows");
    s.insert("menu.settings.language", "Language");
    
    s.insert("menu.sync.title", " Sync & Integrations ");
    s.insert("menu.sync.github", "GitHub");
    s.insert("menu.sync.gdrive", "Google Drive");
    s.insert("menu.sync.gcal", "Google Calendar");
    s.insert("menu.sync.notion", "Notion");
    s.insert("menu.sync.not_configured", "Not configured");
    s.insert("menu.sync.configured", "Configured");
    
    s.insert("wizard.title", "Task Title");
    s.insert("wizard.desc", "Description");
    s.insert("wizard.date", "Date / When (ex: 3d, tomorrow, mon)");
    s.insert("wizard.review", "Review Plan (ex: d3 d5 d7)");
    
    s.insert("confirm.delete", "Do you really want to delete the task:");
    s.insert("confirm.yes", "yes");
    s.insert("confirm.no", "no");
    s.insert("confirm.press", "Press");
    s.insert("confirm.for_yes", "for");
    s.insert("confirm.for_no", "for");
    s.insert("confirm.next", " Next");
    s.insert("confirm.prev", " Previous");
    s.insert("confirm.skip", " Skip");
    
    s.insert("importance.urgent", "Urgent");
    s.insert("importance.high", "High");
    s.insert("importance.medium", "Medium");
    s.insert("importance.low", "Low");
    
    s.insert("detail.empty", "Select a task to view details");
    
    s.insert("menu.settings.export", "📤 Export Tasks (.xlsx)");
    s.insert("menu.settings.import", "📥 Import Tasks (.xlsx)");
    s.insert("menu.settings.update", "🔄 Check for Updates");
    s.insert("menu.settings.sync", "🔁 Sync Menu");
    s.insert("menu.settings.notion_key", "Notion API Key");
    s.insert("menu.settings.notion_db", "Notion Database ID");
    s.insert("menu.settings.sync_interval", "Auto-Sync Interval (min)");
    s.insert("menu.settings.export_done", "Exported to:");
    s.insert("menu.settings.import_path", "Path to .xlsx file");
    s.insert("menu.settings.no_update", "Up to date!");
    s.insert("menu.settings.update_available", "New version available:");
    
    // Days text
    s.insert("days.ago", "ago");
    s.insert("days.in", "in");

    // Welcome Screen
    s.insert("welcome.title_prefix", "Welcome — Slide");
    s.insert("welcome.splash.subtitle", "Welcome to your new productivity hub.");
    s.insert("welcome.splash.desc", "Todo-TUI is designed for speed, offline focus, and clean design.");
    s.insert("welcome.splash.cta", "Press 'n' to begin the tour.");
    s.insert("welcome.layout.title", "THE LAYOUT (Lazygit Inspired)");
    s.insert("welcome.layout.tasks", "[1] Tasks   : Focus on what matters, organized by priority.");
    s.insert("welcome.layout.detail", "[2] Details : All the context you need at a glance.");
    s.insert("welcome.layout.pomo", "[3] Pomodoro: Stay in the zone with built-in timers.");
    s.insert("welcome.layout.nav", "Switch panels using [Tab] or [1, 2, 3] keys.");
    s.insert("welcome.review.title", "SMART REVIEWS (SRS)");
    s.insert("welcome.review.line1", "Don't just finish tasks—master them.");
    s.insert("welcome.review.line2", "Use the review system to schedule periodic follow-ups");
    s.insert("welcome.review.line3", "automatically using syntax like '1d 1w 1m'.");
    s.insert("welcome.review.line4", "Perfect for learning, maintenance, and long-term habits.");
    s.insert("welcome.cmd.title", "COMMANDS & SHORTCUTS");
    s.insert("welcome.cmd.search", " [/] Search : Find any task instantly.");
    s.insert("welcome.cmd.sort", " [o] Sort   : Cycle between Priority, Date, or Name.");
    s.insert("welcome.cmd.add", " [a] Add    : Quick entry wizard.");
    s.insert("welcome.cmd.config", " [c] Config : Deeply customize your experience.");
    s.insert("welcome.cmd.help", " [?] Help   : All keys listed.");
    s.insert("welcome.int.title", "INTEGRATIONS");
    s.insert("welcome.int.line1", "Todo-TUI supports Notion syncing out of the box.");
    s.insert("welcome.int.line2", "Go to Settings [c] to add your API Key and Database ID.");
    s.insert("welcome.int.sync_label", "Current Sync: ");
    s.insert("welcome.ready.title", "READY TO GO");
    s.insert("welcome.ready.line1", "Everything is ready.");
    s.insert("welcome.ready.line2", "Your tasks are stored locally in a JSON file.");
    s.insert("welcome.ready.cta", "Press [n] to enter the main application.");
    s.insert("welcome.hint.next", "Next");
    s.insert("welcome.hint.finish", "Finish");
    s.insert("welcome.hint.prev", "Previous");
    s.insert("welcome.hint.skip", "Skip");
    
    // Help menu title
    s.insert("menu.help.title", " HELP MENU ");

    s
}
