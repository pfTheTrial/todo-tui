use notify_rust::Notification;

pub fn send_notification(title: &str, body: &str) {
    let _ = Notification::new()
        .summary(title)
        .body(body)
        .appname("Todo-TUI")
        .icon("clock")
        .show();
}

pub fn notify_pomodoro_finish(phase_name: &str) {
    send_notification(
        "Pomodoro!",
        &format!("Fase {} concluída! Hora de trocar.", phase_name)
    );
}

pub fn notify_tasks_due(count: usize) {
    if count > 0 {
        send_notification(
            "Tarefas Pendentes",
            &format!("Você tem {} tarefas precisando de atenção hoje.", count)
        );
    }
}
