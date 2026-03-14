use miqat::prelude::*;

fn main() {
    println!("Islamic Events for 2026");
    println!("=======================");
    println!();

    let occurrences = events_for_gregorian_year(2026);

    for o in &occurrences {
        let gregorian = o.gregorian_date.format("%B %-d, %Y");
        println!("{:?}", o.event);
        println!(
            "  Hijri:     {}/{}/{} (AH)",
            o.hijri_date.day, o.hijri_date.month, o.hijri_date.year
        );
        println!("  Gregorian: {gregorian}");
        println!();
    }

    println!("Total: {} events", occurrences.len());
}
