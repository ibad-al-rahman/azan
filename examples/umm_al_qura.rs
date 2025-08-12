use azan::prelude::*;

fn main() {
    println!("Prayer times for Makka in UTC");
    println!("------------------------------");
    println!();
    let makka = Coordinates::new(21.427009, 39.828685);
    let date = Utc::now().date_naive();
    let params = Configuration::with(Method::UmmAlQura, Mazhab::Shafi);
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(makka)
        .with_configuration(params)
        .calculate();

    let Ok(prayer) = prayers else {
        eprintln!("Could not calculate prayer times");
        return;
    };

    println!(
        "{}: {}",
        Prayer::Fajr.name(),
        prayer.time(Prayer::Fajr).format("%-l:%M %p").to_string()
    );
    println!(
        "{}: {}",
        Prayer::Sunrise.name(),
        prayer.time(Prayer::Sunrise).format("%-l:%M %p").to_string()
    );
    println!(
        "{}: {}",
        Prayer::Dhuhr.name(),
        prayer.time(Prayer::Dhuhr).format("%-l:%M %p").to_string()
    );
    println!(
        "{}: {}",
        Prayer::Asr.name(),
        prayer.time(Prayer::Asr).format("%-l:%M %p").to_string()
    );
    println!(
        "{}: {}",
        Prayer::Maghrib.name(),
        prayer.time(Prayer::Maghrib).format("%-l:%M %p").to_string()
    );
    println!(
        "{}: {}",
        Prayer::Ishaa.name(),
        prayer.time(Prayer::Ishaa).format("%-l:%M %p").to_string()
    );
    println!(
        "{}: {}",
        Prayer::Qiyam.name(),
        prayer.time(Prayer::Qiyam).format("%-l:%M %p").to_string()
    );
}
