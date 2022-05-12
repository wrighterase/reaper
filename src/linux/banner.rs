use colored::Colorize;

pub fn banner() {
    println!("{}","___ ____ ____ ___  ____ ____".cyan());
    println!("{}","|__/ |___ |__| |__] |___ |__/ ".cyan());
    println!("{}","|  \\ |___ |  | |    |___ |  \\".cyan());
    println!("{}{} {}{} {}{} {}{} {}{} {}{}",
    "R".red(),"ust",
    "E".red(),"nhanced",
    "A".red(),"utomated",
    "P".red(),"rivilege",
    "E".red(),"scalation",
    "R".red(),"econ");
    println!("");
}

pub fn section_label(x: &str) {
    println!("{}{}", "╔══════════╣".blue(), x.green());
}

pub fn module_label(x: &str) {
    println!("{} {} {}", 
    "═════════════════════════════════════════╣". blue(), 
    x.green(),
    "╠═════════════════════════════════════════".blue());
}