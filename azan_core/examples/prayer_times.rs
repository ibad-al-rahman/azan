use azan::prelude::*;

fn main() {
    let today = NaiveDate::from_ymd_opt(2026, 3, 5).unwrap();

    // --- Calculated prayer times (Beirut coordinates) ---
    let beirut = Coordinates::new(33.8938, 35.5018);
    let params = Method::MuslimWorldLeague.parameters().mazhab(Mazhab::Shafi);
    let calculated = PrayerTimes::new(today, beirut, params);

    println!("=== Calculated (Beirut) ===");
    println!(
        "Fajr:    {}",
        calculated.time(Prayer::Fajr).format("%H:%M UTC")
    );
    println!(
        "Sunrise: {}",
        calculated.time(Prayer::Sunrise).format("%H:%M UTC")
    );
    println!(
        "Dhuhr:   {}",
        calculated.time(Prayer::Dhuhr).format("%H:%M UTC")
    );
    println!(
        "Asr:     {}",
        calculated.time(Prayer::Asr).format("%H:%M UTC")
    );
    println!(
        "Maghrib: {}",
        calculated.time(Prayer::Maghrib).format("%H:%M UTC")
    );
    println!(
        "Ishaa:   {}",
        calculated.time(Prayer::Ishaa).format("%H:%M UTC")
    );

    // --- Precomputed prayer times (Dar El-Fatwa, Beirut) ---
    let precomputed =
        PrecomputedPrayerTimes::new(today, Provider::DarElFatwa(ProviderCity::Beirut));

    println!("\n=== Precomputed / Dar El-Fatwa (Beirut) ===");
    println!(
        "Fajr:    {}",
        precomputed.time(Prayer::Fajr).format("%H:%M UTC")
    );
    println!(
        "Sunrise: {}",
        precomputed.time(Prayer::Sunrise).format("%H:%M UTC")
    );
    println!(
        "Dhuhr:   {}",
        precomputed.time(Prayer::Dhuhr).format("%H:%M UTC")
    );
    println!(
        "Asr:     {}",
        precomputed.time(Prayer::Asr).format("%H:%M UTC")
    );
    println!(
        "Maghrib: {}",
        precomputed.time(Prayer::Maghrib).format("%H:%M UTC")
    );
    println!(
        "Ishaa:   {}",
        precomputed.time(Prayer::Ishaa).format("%H:%M UTC")
    );
    println!(
        "Fajr Tomorrow:   {}",
        precomputed.time(Prayer::FajrTomorrow).format("%H:%M UTC")
    );
}
