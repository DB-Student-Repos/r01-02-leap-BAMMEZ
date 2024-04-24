pub fn is_leap_year(year: u64) -> bool {
    if year % 4 != 0 {
        return false; // Not divisible by 4, not a leap year
    }
    if year % 100 == 0 {
        // Divisible by 100
        if year % 400 == 0 {
            // Divisible by 400, leap year
            return true;
        }
        return false; // Divisible by 100 but not by 400, not a leap year
    }
    true // Divisible by 4 but not by 100, leap year
}

fn main() {
    let year = 2000;
    if is_leap_year(year) {
        println!("{} is a leap year.", year);
    } else {
        println!("{} is not a leap year.", year);
    }
}








