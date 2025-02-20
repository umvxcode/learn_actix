use chrono::{DateTime, Local, Utc};

pub struct  AmaDate;
impl AmaDate{
        pub fn indoFormat(datetime_utc: String, format: &str) -> String {
            let parsed_time: DateTime<Utc> = datetime_utc.parse().unwrap_or(Utc::now());

            let local_time = parsed_time.with_timezone(&Local);

            let bulan_indo = [
                "Januari", "Februari", "Maret", "April", "Mei", "Juni",
                "Juli", "Agustus", "September", "Oktober", "November", "Desember",
            ];
            let month_index = local_time.format("%m").to_string().parse::<usize>().unwrap() - 1;
            let bulan = bulan_indo[month_index];

            let formatted_time = format
                .replace("d", &local_time.format("%d").to_string())
                .replace("m", bulan)
                .replace("Y", &local_time.format("%Y").to_string())
                .replace("H", &local_time.format("%H").to_string())
                .replace("M", &local_time.format("%M").to_string())
                .replace("S", &local_time.format("%S").to_string());

            formatted_time
        }

}