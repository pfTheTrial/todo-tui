use chrono::{DateTime, Utc, Local, Duration, NaiveDate, Datelike};

pub fn parse_date_input(input: &str) -> Option<DateTime<Utc>> {
    let input = input.trim().to_lowercase();
    if input.is_empty() { return None; }

    // Constants
    let now = Utc::now();
    let today = Local::now().date_naive();

    // Keywords
    if input == "hoje" {
        return Some(now);
    }
    if input == "amanha" || input == "amanhã" {
        return Some(now + Duration::days(1));
    }
    if input == "ontem" {
        return Some(now - Duration::days(1));
    }

    // Relative Days: 3d, d3
    if input.ends_with('d') {
        if let Ok(days) = input.trim_end_matches('d').parse::<i64>() {
            return Some(now + Duration::days(days));
        }
    }
    if input.starts_with('d') {
        if let Ok(days) = input[1..].parse::<i64>() {
            return Some(now + Duration::days(days));
        }
    }

    // Relative Weeks: 2s, s2
    if input.ends_with('s') {
        if let Ok(weeks) = input.trim_end_matches('s').parse::<i64>() {
            return Some(now + Duration::days(weeks * 7));
        }
    }
    if input.starts_with('s') {
        if let Ok(weeks) = input[1..].parse::<i64>() {
            return Some(now + Duration::days(weeks * 7));
        }
    }

    // Relative Months: 1m, m1 (Approx 30 days)
    if input.ends_with('m') {
        if let Ok(months) = input.trim_end_matches('m').parse::<i64>() {
            return Some(now + Duration::days(months * 30));
        }
    }
    if input.starts_with('m') {
        if let Ok(months) = input[1..].parse::<i64>() {
            return Some(now + Duration::days(months * 30));
        }
    }

    // Day of week
    let dow = match input.as_str() {
        "seg" | "segunda" => Some(chrono::Weekday::Mon),
        "ter" | "terça" => Some(chrono::Weekday::Tue),
        "qua" | "quarta" => Some(chrono::Weekday::Wed),
        "qui" | "quinta" => Some(chrono::Weekday::Thu),
        "sex" | "sexta" => Some(chrono::Weekday::Fri),
        "sab" | "sábado" | "sabado" => Some(chrono::Weekday::Sat),
        "dom" | "domingo" => Some(chrono::Weekday::Sun),
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
        // Try dd/mm with current year
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
    if input.ends_with('d') {
        if let Ok(days) = input.trim_end_matches('d').parse::<i64>() {
            return Some(base + Duration::days(days));
        }
    }
    if input.starts_with('d') {
        if let Ok(days) = input[1..].parse::<i64>() {
            return Some(base + Duration::days(days));
        }
    }

    // Relative Weeks: 2s, s2
    if input.ends_with('s') {
        if let Ok(weeks) = input.trim_end_matches('s').parse::<i64>() {
            return Some(base + Duration::days(weeks * 7));
        }
    }
    if input.starts_with('s') {
        if let Ok(weeks) = input[1..].parse::<i64>() {
            return Some(base + Duration::days(weeks * 7));
        }
    }

    // Relative Months: 1m, m1
    if input.ends_with('m') {
        if let Ok(months) = input.trim_end_matches('m').parse::<i64>() {
            return Some(base + Duration::days(months * 30));
        }
    }
    if input.starts_with('m') {
        if let Ok(months) = input[1..].parse::<i64>() {
            return Some(base + Duration::days(months * 30));
        }
    }

    parse_date_input(input.as_str())
}
