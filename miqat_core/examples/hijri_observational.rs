use calendrical_calculations::islamic::MECCA;
use miqat::prelude::*;

fn main() {
    let date = Utc::now().date_naive();

    // Tabular Hijri (arithmetic, same result everywhere)
    let tabular = HijriDate::from_gregorian(date);
    println!("Tabular Hijri:       {tabular}");

    // Observational Hijri for Mecca (UTC+3)
    let mecca_hijri = HijriDate::from_gregorian_observational(date, MECCA);
    println!("Observational Mecca: {mecca_hijri}");

    // Observational Hijri for New York (UTC-5)
    // let new_york = Location::try_new(40.7, -74.0, 10.0, -5.0 / 24.0).unwrap();
    // let ny_hijri = HijriDate::from_gregorian_observational(date, new_york);
    // println!("Observational NYC:   {ny_hijri}");

    // Show any Islamic events for today
    println!();
    let events = mecca_hijri.events();
    if events.is_empty() {
        println!("No Islamic events today.");
    } else {
        for event in events {
            println!("Islamic event: {event:?}");
        }
    }
}
