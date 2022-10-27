use chrono::{Local, DateTime};
use colored::Colorize;



#[static_init::dynamic]
static mut start : DateTime<Local> = Local::now();



pub fn critical<S: AsRef<str>>(text : S) {
    let text_ref = text.as_ref();
    println!("{} {} {}{} {} {} {}",
        "[".dimmed().white(), duration().as_str().bright_blue(), "]".dimmed().white(),
        "[".dimmed().white(), "CRITICAL".on_bright_red().white().bold(), "]".dimmed().white(),
        text_ref.on_bright_red().white().bold()
    );
}



pub fn error<S: AsRef<str>>(text : S) {
    let text_ref = text.as_ref();
    println!("{} {} {}{} {} {} {}",
        "[".dimmed().white(), duration().as_str().bright_blue(), "]".dimmed().white(),
        "[".dimmed().white(), "ERROR   ".red().bold(), "]".dimmed().white(),
        text_ref.red().bold()
    );
}



pub fn warning<S: AsRef<str>>(text : S) {
    let text_ref = text.as_ref();
    println!("{} {} {}{} {} {} {}",
        "[".dimmed().white(), duration().as_str().bright_blue(), "]".dimmed().white(),
        "[".dimmed().white(), "WARNING ".yellow().bold(), "]".dimmed().white(),
        text_ref.yellow().bold()
    );
}



pub fn success<S: AsRef<str>>(text : S) {
    let text_ref = text.as_ref();
    println!("{} {} {}{} {} {} {}",
        "[".dimmed().white(), duration().as_str().bright_blue(), "]".dimmed().white(),
        "[".dimmed().white(), "SUCCESS ".bright_green(), "]".dimmed().white(),
        text_ref.bright_green()
    );
}



pub fn info<S: AsRef<str>>(text : S) {
    let text_ref = text.as_ref();
    println!("{} {} {}{} {} {} {}",
        "[".dimmed().white(), duration().as_str().bright_blue(), "]".dimmed().white(),
        "[".dimmed().white(), "INFO    ".bright_white().bold(), "]".dimmed().white(),
        text_ref.bright_white().bold()
    );
}



pub fn debug<S: AsRef<str>>(text : S) {
    let text_ref = text.as_ref();
    println!("{} {} {}{} {} {} {}",
        "[".dimmed().white(), duration().as_str().bright_blue(), "]".dimmed().white(),
        "[".dimmed().white(), "DEBUG   ".white().dimmed(), "]".dimmed().white(),
        text_ref.white().dimmed()
    );
}



fn duration() -> String {
    let duration = (Local::now() - *start.read()).to_std().unwrap();
    let hours    = duration.as_secs() / 3600;
    let minutes  = duration.as_secs() % 3600 / 60;
    let seconds  = duration.as_secs() % 60;
    let decimal  = duration.subsec_nanos();
    format!(
        "{:0>2}:{:0>2}:{:0>2}.{:0>9}",
        hours,
        minutes,
        seconds,
        decimal,
    )
}
