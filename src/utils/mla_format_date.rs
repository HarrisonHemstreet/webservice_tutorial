use chrono::naive::NaiveDateTime;

pub fn mla_format_date(datetime: NaiveDateTime) -> String {
    let date: String = datetime.date().format("%m %Y %d").to_string();
    let time: String = datetime.time().format("%r %Z").to_string();
    println!("{} {}", date.trim(), time.trim());
    format!("{} {}", date.trim(), time.trim())
}
