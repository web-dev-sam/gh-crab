use chrono::{Days, NaiveDate};

pub fn commit_timestamps(image: &[[i32; 16]; 7], image_start: &NaiveDate) -> Vec<i64> {
    let mut timestamps = Vec::new();

    for (day, row) in image.iter().enumerate() {
        for (week, &intensity) in row.iter().enumerate() {
            let Some(date) = image_start
                .checked_add_days(Days::new((week * 7 + day) as u64))
                .and_then(|s| s.and_hms_opt(12, 0, 0))
            else {
                continue;
            };

            let count = intensity_to_commits(intensity);
            for _ in 0..count {
                timestamps.push(date.and_utc().timestamp());
            }
        }
    }

    timestamps
}

const fn intensity_to_commits(intensity: i32) -> i32 {
    match intensity {
        1 => 1,
        2 => 7,
        3 => 14,
        4 => 20,
        _ => 0,
    }
}
