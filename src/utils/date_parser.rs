use chrono::{DateTime, Utc, Local, Duration, NaiveDate, Datelike};

pub fn parse_date_input(input: &str) -> Option<DateTime<Utc>> {
    let input = input.trim().to_lowercase();
    if input.is_empty() { return None; }

    let now = Utc::now();
    let today = Local::now().date_naive();

    // Keywords — PT-BR, EN, ES
    if input == "hoje" || input == "today" || input == "hoy" {
        return Some(now);
    }
    if input == "amanha" || input == "amanhã" || input == "tomorrow" || input == "mañana" || input == "manana" {
        return Some(now + Duration::days(1));
    }
    if input == "ontem" || input == "yesterday" || input == "ayer" {
        return Some(now - Duration::days(1));
    }

    // Relative Days: 3d, d3
    if let Some(suffix) = input.strip_suffix('d') {
        if let Ok(days) = suffix.parse::<i64>() {
            return Some(now + Duration::days(days));
        }
    }
    if let Some(rest) = input.strip_prefix('d') {
        if let Ok(days) = rest.parse::<i64>() {
            return Some(now + Duration::days(days));
        }
    }

    // Relative Weeks: 2s/2w, s2/w2
    if let Some(suffix) = input.strip_suffix('s').or_else(|| input.strip_suffix('w')) {
        if let Ok(weeks) = suffix.parse::<i64>() {
            return Some(now + Duration::days(weeks * 7));
        }
    }
    if let Some(rest) = input.strip_prefix('s').or_else(|| input.strip_prefix('w')) {
        if let Ok(weeks) = rest.parse::<i64>() {
            return Some(now + Duration::days(weeks * 7));
        }
    }

    // Relative Months: 1m, m1 (approx 30 days)
    if let Some(suffix) = input.strip_suffix('m') {
        if let Ok(months) = suffix.parse::<i64>() {
            return Some(now + Duration::days(months * 30));
        }
    }
    if let Some(rest) = input.strip_prefix('m') {
        if let Ok(months) = rest.parse::<i64>() {
            return Some(now + Duration::days(months * 30));
        }
    }

    // Day of week — PT-BR + EN
    let dow = match input.as_str() {
        "seg" | "segunda" | "mon" | "monday"       => Some(chrono::Weekday::Mon),
        "ter" | "terça" | "terca" | "tue" | "tuesday"  => Some(chrono::Weekday::Tue),
        "qua" | "quarta" | "wed" | "wednesday"     => Some(chrono::Weekday::Wed),
        "qui" | "quinta" | "thu" | "thursday"      => Some(chrono::Weekday::Thu),
        "sex" | "sexta" | "fri" | "friday"         => Some(chrono::Weekday::Fri),
        "sab" | "sábado" | "sabado" | "sat" | "saturday" => Some(chrono::Weekday::Sat),
        "dom" | "domingo" | "sun" | "sunday"       => Some(chrono::Weekday::Sun),
        _ => None,
    };

    if let Some(target_dow) = dow {
        let current_dow = today.weekday();
        let days_to_add = (target_dow.num_days_from_monday() as i32 - current_dow.num_days_from_monday() as i32 + 7) % 7;
        let days_to_add = if days_to_add == 0 { 7 } else { days_to_add };
        let target_date = today + Duration::days(days_to_add as i64);
        return Some(target_date.and_hms_opt(12, 0, 0).unwrap().and_local_timezone(Local).unwrap().with_timezone(&Utc));
    }

    // Exact dates: dd/mm/yyyy, dd/mm, dd-mm-yyyy, yyyy-mm-dd
    let formats = ["%d/%m/%Y", "%d/%m", "%d-%m-%Y", "%Y-%m-%d"];
    for fmt in formats {
        if let Ok(parsed) = NaiveDate::parse_from_str(&input, fmt) {
            return Some(parsed.and_hms_opt(12, 0, 0).unwrap().and_local_timezone(Local).unwrap().with_timezone(&Utc));
        }
        if fmt == "%d/%m" {
            if let Ok(parsed) = NaiveDate::parse_from_str(&(input.to_string() + &format!("/{}", today.year())), "%d/%m/%Y") {
                return Some(parsed.and_hms_opt(12, 0, 0).unwrap().and_local_timezone(Local).unwrap().with_timezone(&Utc));
            }
        }
    }

    None
}

pub fn parse_relative_date(input: &str, base: DateTime<Utc>) -> Option<DateTime<Utc>> {
    let input = input.trim().to_lowercase();
    if input.is_empty() { return None; }

    // Relative Days: 3d, d3
    if let Some(suffix) = input.strip_suffix('d') {
        if let Ok(days) = suffix.parse::<i64>() {
            return Some(base + Duration::days(days));
        }
    }
    if let Some(rest) = input.strip_prefix('d') {
        if let Ok(days) = rest.parse::<i64>() {
            return Some(base + Duration::days(days));
        }
    }

    // Relative Weeks: 2s/2w, s2/w2
    if let Some(suffix) = input.strip_suffix('s').or_else(|| input.strip_suffix('w')) {
        if let Ok(weeks) = suffix.parse::<i64>() {
            return Some(base + Duration::days(weeks * 7));
        }
    }
    if let Some(rest) = input.strip_prefix('s').or_else(|| input.strip_prefix('w')) {
        if let Ok(weeks) = rest.parse::<i64>() {
            return Some(base + Duration::days(weeks * 7));
        }
    }

    // Relative Months: 1m, m1
    if let Some(suffix) = input.strip_suffix('m') {
        if let Ok(months) = suffix.parse::<i64>() {
            return Some(base + Duration::days(months * 30));
        }
    }
    if let Some(rest) = input.strip_prefix('m') {
        if let Ok(months) = rest.parse::<i64>() {
            return Some(base + Duration::days(months * 30));
        }
    }

    parse_date_input(input.as_str())
}
