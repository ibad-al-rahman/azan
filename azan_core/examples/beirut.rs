use azan::prelude::*;

fn main() {
    println!("Prayer times for Beirut in UTC");
    println!("------------------------------");
    println!();
    let makka = Coordinates::new(33.888630, 35.495480);
    let date = Utc::now().date_naive();
    let params = Method::Egyptian.parameters();
    let prayer_times = PrayerTimes::new(date, makka, params);

    println!(
        "{:?}: {}",
        Prayer::Fajr,
        prayer_times
            .time(Prayer::Fajr)
            .format("%-l:%M %p")
            .to_string()
    );
    println!(
        "{:?}: {}",
        Prayer::Sunrise,
        prayer_times
            .time(Prayer::Sunrise)
            .format("%-l:%M %p")
            .to_string()
    );
    println!(
        "{:?}: {}",
        Prayer::Dhuhr,
        prayer_times
            .time(Prayer::Dhuhr)
            .format("%-l:%M %p")
            .to_string()
    );
    println!(
        "{:?}: {}",
        Prayer::Asr,
        prayer_times
            .time(Prayer::Asr)
            .format("%-l:%M %p")
            .to_string()
    );
    println!(
        "{:?}: {}",
        Prayer::Maghrib,
        prayer_times
            .time(Prayer::Maghrib)
            .format("%-l:%M %p")
            .to_string()
    );
    println!(
        "{:?}: {}",
        Prayer::Ishaa,
        prayer_times
            .time(Prayer::Ishaa)
            .format("%-l:%M %p")
            .to_string()
    );
    println!(
        "{:?}: {}",
        Prayer::Qiyam,
        prayer_times
            .time(Prayer::Qiyam)
            .format("%-l:%M %p")
            .to_string()
    );
}
