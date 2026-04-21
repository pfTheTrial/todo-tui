use notify_rust::Notification;

pub fn send_notification(title: &str, body: &str) {
    let _ = Notification::new()
        .summary(title)
        .body(body)
        .show();
}

pub fn notify_pomodoro_finish(title: &str, body: &str) {
    send_notification(title, body);
}

#[allow(dead_code)]
pub fn notify_tasks_due(title: &str, body: &str) {
    send_notification(title, body);
}
