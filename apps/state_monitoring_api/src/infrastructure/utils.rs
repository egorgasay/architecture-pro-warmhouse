use chrono::{DateTime, NaiveDateTime, Utc};
use log::{error, debug};

/// Парсит строку даты в различных форматах и возвращает DateTime<Utc>
/// Поддерживает следующие форматы:
/// - RFC3339 (ISO 8601)
/// - "%Y-%m-%d %H:%M:%S%.f %z" (с микросекундами и часовым поясом)
/// - "%Y-%m-%d %H:%M:%S%.f" (с микросекундами без часового пояса)
/// - "%Y-%m-%d %H:%M:%S" (без микросекунд и часового пояса)
/// 
/// В случае ошибки парсинга возвращает текущее время UTC
pub fn parse_datetime(date_str: &str) -> DateTime<Utc> {
    // Попробуем RFC3339 формат
    if let Ok(datetime) = DateTime::parse_from_rfc3339(date_str) {
        debug!("Successfully parsed date '{}' using RFC3339 format", date_str);
        return datetime.with_timezone(&Utc);
    }

    // Попробуем формат с микросекундами и часовым поясом
    if let Ok(datetime) = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S%.f %z") {
        debug!("Successfully parsed date '{}' using format with microseconds and timezone", date_str);
        return datetime.with_timezone(&Utc);
    }

    // Попробуем формат с микросекундами без часового пояса
    if let Ok(naive) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S%.f") {
        debug!("Successfully parsed date '{}' using format with microseconds", date_str);
        return DateTime::from_naive_utc_and_offset(naive, Utc);
    }

    // Попробуем базовый формат без микросекунд и часового пояса
    if let Ok(naive) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
        debug!("Successfully parsed date '{}' using basic format", date_str);
        return DateTime::from_naive_utc_and_offset(naive, Utc);
    }

    // Если все попытки не удались, логируем ошибку и возвращаем текущее время
    error!("Failed to parse date '{}' with any supported format, using current time", date_str);
    Utc::now()
}

/// Безопасно форматирует NaiveDateTime в строку
pub fn format_datetime_safe(datetime: &NaiveDateTime) -> String {
    datetime.format("%Y-%m-%d %H:%M:%S%.f").to_string()
}
