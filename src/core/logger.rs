use chrono::Local;
use colored::Colorize;



pub fn critical<S: AsRef<str>>(text : S) {
    let text_ref = text.as_ref();
    let now = Local::now();
    println!("{} {} {}{} {} {} {}",
        "[".dimmed().white(), now.format("%T.%f").to_string().as_str().bright_blue(), "]".dimmed().white(),
        "[".dimmed().white(), "CRITICAL".on_bright_red().white().bold(), "]".dimmed().white(),
        text_ref.on_bright_red().white().bold()
    );
}



pub fn error<S: AsRef<str>>(text : S) {
    let text_ref = text.as_ref();
    let now = Local::now();
    println!("{} {} {}{} {} {} {}",
        "[".dimmed().white(), now.format("%T.%f").to_string().as_str().bright_blue(), "]".dimmed().white(),
        "[".dimmed().white(), "ERROR   ".red().bold(), "]".dimmed().white(),
        text_ref.red().bold()
    );
}



pub fn warning<S: AsRef<str>>(text : S) {
    let text_ref = text.as_ref();
    let now = Local::now();
    println!("{} {} {}{} {} {} {}",
        "[".dimmed().white(), now.format("%T.%f").to_string().as_str().bright_blue(), "]".dimmed().white(),
        "[".dimmed().white(), "WARNING ".yellow().bold(), "]".dimmed().white(),
        text_ref.yellow().bold()
    );
}



pub fn success<S: AsRef<str>>(text : S) {
    let text_ref = text.as_ref();
    let now = Local::now();
    println!("{} {} {}{} {} {} {}",
        "[".dimmed().white(), now.format("%T.%f").to_string().as_str().bright_blue(), "]".dimmed().white(),
        "[".dimmed().white(), "SUCCESS ".bright_green(), "]".dimmed().white(),
        text_ref.bright_green()
    );
}



pub fn info<S: AsRef<str>>(text : S) {
    let text_ref = text.as_ref();
    let now = Local::now();
    println!("{} {} {}{} {} {} {}",
        "[".dimmed().white(), now.format("%T.%f").to_string().as_str().bright_blue(), "]".dimmed().white(),
        "[".dimmed().white(), "INFO    ".white().bold(), "]".dimmed().white(),
        text_ref.white().bold()
    );
}



pub fn debug<S: AsRef<str>>(text : S) {
    let text_ref = text.as_ref();
    let now = Local::now();
    println!("{} {} {}{} {} {} {}",
        "[".dimmed().white(), now.format("%T.%f").to_string().as_str().bright_blue(), "]".dimmed().white(),
        "[".dimmed().white(), "DEBUG   ".bright_black(), "]".dimmed().white(),
        text_ref.bright_black()
    );
}



pub fn trace<S: AsRef<str>>(text : S) {
    let text_ref = text.as_ref();
    let now = Local::now();
    println!("{} {} {}{} {} {} {}",
        "[".dimmed().white(), now.format("%T.%f").to_string().as_str().bright_blue(), "]".dimmed().white(),
        "[".dimmed().white(), "TRACE   ".bright_black().dimmed(), "]".dimmed().white(),
        text_ref.bright_black().dimmed()
    );
}
