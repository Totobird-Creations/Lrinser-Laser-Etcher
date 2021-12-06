use colored::Colorize;



pub fn info(text : &str) {
    println!("{} {} {}", "[".dimmed().white(), "INFO".white(), "]".dimmed().white());
}
macro_rules! info {
    ($($arg : tt) *) => { ... };
}
