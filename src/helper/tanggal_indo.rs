// use chrono::{DateTime, Local, Utc};

// pub fn tanggal_indo(datetime_utc: String) -> String {
//     let parsed_time: DateTime<Utc> = datetime_utc.parse().unwrap_or(Utc::now());

//     // Convert to local time (Indonesia Time)
//     let local_time = parsed_time.with_timezone(&Local);

//     // Format the datetime to "19 Februari 2025 17:08"
//     local_time.format("%d %B %Y %H:%M").to_string()
// }



use chrono::{DateTime, Local, Utc};

pub fn tanggal_indo(datetime_utc: String, format: &str) -> String {
    let parsed_time: DateTime<Utc> = datetime_utc.parse().unwrap_or(Utc::now());

    // Convert to local time (Indonesia Time)
    let local_time = parsed_time.with_timezone(&Local);

    // Extract date components
    let day = local_time.format("%d").to_string();
    let month_num = local_time.format("%m").to_string();
    let year = local_time.format("%Y").to_string();
    let time = local_time.format("%H:%M").to_string();

    // Map English month names to Indonesian
    let bulan_indo = match month_num.as_str() {
        "01" => "Januari",
        "02" => "Februari",
        "03" => "Maret",
        "04" => "April",
        "05" => "Mei",
        "06" => "Juni",
        "07" => "Juli",
        "08" => "Agustus",
        "09" => "September",
        "10" => "Oktober",
        "11" => "November",
        "12" => "Desember",
        _ => "Unknown",
    };

    match format {
        "full" => format!("{} {} {} {}", day, bulan_indo, year, time), // "19 Februari 2025 17:08"
        "date_only" => format!("{} {} {}", day, bulan_indo, year),     // "19 Februari 2025"
        "time_only" => format!("{}", time),                            // "17:08"
        _ => format!("{} {} {} {}", day, bulan_indo, year, time),      // Default: Full format
    }
}
