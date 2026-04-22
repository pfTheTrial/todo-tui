use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Utc};

pub fn parse_date_input(input: &str) -> Option<DateTime<Utc>> {
    let input = input.trim().to_lowercase();
    if input.is_empty() {
        return None;
    }

    let now = Utc::now();
    let today = Local::now().date_naive();

    // Keywords — PT-BR, EN, ES
    if input == "hoje" || input == "today" || input == "hoy" {
        return Some(now);
    }
    if input == "amanha"
        || input == "amanhã"
        || input == "tomorrow"
        || input == "mañana"
        || input == "manana"
    {
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
        "seg" | "segunda" | "mon" | "monday" => Some(chrono::Weekday::Mon),
        "ter" | "terça" | "terca" | "tue" | "tuesday" => Some(chrono::Weekday::Tue),
        "qua" | "quarta" | "wed" | "wednesday" => Some(chrono::Weekday::Wed),
        "qui" | "quinta" | "thu" | "thursday" => Some(chrono::Weekday::Thu),
        "sex" | "sexta" | "fri" | "friday" => Some(chrono::Weekday::Fri),
        "sab" | "sábado" | "sabado" | "sat" | "saturday" => Some(chrono::Weekday::Sat),
        "dom" | "domingo" | "sun" | "sunday" => Some(chrono::Weekday::Sun),
        _ => None,
    };

    if let Some(target_dow) = dow {
        let current_dow = today.weekday();
        let days_to_add = (target_dow.num_days_from_monday() as i32
            - current_dow.num_days_from_monday() as i32
            + 7)
            % 7;
        let days_to_add = if days_to_add == 0 { 7 } else { days_to_add };
        let target_date = today + Duration::days(days_to_add as i64);
        return Some(
            target_date
                .and_hms_opt(12, 0, 0)
                .unwrap()
                .and_local_timezone(Local)
                .unwrap()
                .with_timezone(&Utc),
        );
    }

    // Exact dates: dd/mm/yyyy, dd/mm, dd-mm-yyyy, yyyy-mm-dd
    let formats = ["%d/%m/%Y", "%d/%m", "%d-%m-%Y", "%Y-%m-%d"];
    for fmt in formats {
        if let Ok(parsed) = NaiveDate::parse_from_str(&input, fmt) {
            return Some(
                parsed
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
                    .and_local_timezone(Local)
                    .unwrap()
                    .with_timezone(&Utc),
            );
        }
        if fmt == "%d/%m" {
            if let Ok(parsed) = NaiveDate::parse_from_str(
                &(input.to_string() + &format!("/{}", today.year())),
                "%d/%m/%Y",
            ) {
                return Some(
                    parsed
                        .and_hms_opt(12, 0, 0)
                        .unwrap()
                        .and_local_timezone(Local)
                        .unwrap()
                        .with_timezone(&Utc),
                );
            }
        }
    }

    None
}

pub fn parse_relative_date(input: &str, base: DateTime<Utc>) -> Option<DateTime<Utc>> {
    let input = input.trim().to_lowercase();
    if input.is_empty() {
        return None;
    }

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

#[cfg(test)]
mod tests {
    use super::{parse_date_input, parse_relative_date};
    use chrono::{Duration, TimeZone, Utc};

    #[test]
    fn parses_multilingual_keywords() {
        assert!(parse_date_input("hoje").is_some());
        assert!(parse_date_input("today").is_some());
        assert!(parse_date_input("mañana").is_some());
        assert!(parse_date_input("not-a-date").is_none());
    }

    #[test]
    fn parses_relative_dates_from_now() {
        let now = Utc::now();
        let in_three_days = parse_date_input("3d").expect("3d should parse");
        let diff = in_three_days.signed_duration_since(now).num_days();
        assert!((2..=3).contains(&diff));

        let in_two_weeks = parse_date_input("2w").expect("2w should parse");
        let diff = in_two_weeks.signed_duration_since(now).num_days();
        assert!((13..=14).contains(&diff));
    }

    #[test]
    fn parses_relative_dates_from_base() {
        let base = Utc.with_ymd_and_hms(2026, 4, 21, 12, 0, 0).unwrap();
        assert_eq!(
            parse_relative_date("1d", base),
            Some(base + Duration::days(1))
        );
        assert_eq!(
            parse_relative_date("2s", base),
            Some(base + Duration::days(14))
        );
        assert_eq!(
            parse_relative_date("1m", base),
            Some(base + Duration::days(30))
        );
    }
}
