use std::process::{Command, Stdio};
use colored::Colorize;
use std::env;
use crate::linux::banner::section_label;
use crate::linux::banner::module_label;

pub fn get_info() {

    // println!("{}","                                         ╔════════════════════╗".blue());
    // println!("{}{}{}","═════════════════════════════════════════╣ ".blue(),"System information".green()," ╠═════════════════════════════════════════".blue());
    // println!("{}","                                         ╚════════════════════╝".blue());
    module_label("System Information");
    section_label("PATH Variables");
    match env::var("PATH") {
        Ok(path) => println!("{}", path),
        Err(e) => println!("No PATH variable set. {}", e),
    };

    section_label("Environment Variables");
    let mut cmd = Command::new("env");
    match cmd.output() {
        Ok(_status) => {
            println!("{}", String::from_utf8_lossy(&_status.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    section_label("/etc/passwd");
    let mut cmd = Command::new("cat");
    match cmd.output() {
        Ok(_status) => {
            let passwd = Command::new("cat").arg("/etc/passwd").output().expect("");
            println!("{}", String::from_utf8_lossy(&passwd.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    section_label("Enumerating sudo");
    let mut cmd = Command::new("sudo");
    match cmd.output() {
        Ok(_status) => {
            let sudo = Command::new("sudo").arg("-V").output().expect("");
            println!("{}", String::from_utf8_lossy(&sudo.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    let mut cmd = Command::new("sudo");
    match cmd.output() {
        Ok(_status) => {
            let sudo = Command::new("sudo").arg("-l").output().expect("");
            println!("{}", String::from_utf8_lossy(&sudo.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }
    //broken
    let mut cmd = Command::new("sudoedit");
    match cmd.output() {
        Ok(_status) => {
            let sudoedit = Command::new("sudoedit").arg("-s").arg("Y").output().expect("");
            let output = String::from_utf8_lossy(&sudoedit.stdout);
            if output.contains("usage") {
                println!("{} {}", "[-]".red(), "Prompted for usage.  Probably not vulnerable.");
            }
            if output.contains("password") {
                println!("{} {}", "[+]".green(), "Prompted for password.  Probably vulnerable!");
            }
            println!("{}", String::from_utf8_lossy(&sudoedit.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    section_label("Possible account forgery?");
    println!("{} Can write to /etc/passwd?", "[+]".green());
    let mut cmd = Command::new("ls");
    match cmd.output() {
        Ok(_status) => {
            let ls = Command::new("ls").arg("-l").arg("/etc/passwd").stdout(Stdio::piped()).spawn().expect("");
            let cut1 = Command::new("cut").arg("-d").arg(" ").arg("-f").arg("1").stdin(ls.stdout.expect("")).stdout(Stdio::piped()).spawn().expect("");
            let cut2 = Command::new("cut").arg("-c").arg("9-").stdin(cut1.stdout.expect("")).output().expect("");
            if cut2.stdout.contains(&b'w') {
                println!("{} {} {}", "[+]".green(), String::from_utf8_lossy(&cut2.stdout).trim(), "yes!".green());
            }else{
                println!("{} {} {}", "[-]".red(), String::from_utf8_lossy(&cut2.stdout).trim(), "nope!".red());
            }
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    println!("{} Can write to /etc/sudoers?", "[+]".green());
    let mut cmd = Command::new("ls");
    match cmd.output() {
        Ok(_status) => {
            let ls = Command::new("ls").arg("-l").arg("/etc/sudoers").stdout(Stdio::piped()).spawn().expect("");
            let cut1 = Command::new("cut").arg("-d").arg(" ").arg("-f").arg("1").stdin(ls.stdout.expect("")).stdout(Stdio::piped()).spawn().expect("");
            let cut2 = Command::new("cut").arg("-c").arg("9-").stdin(cut1.stdout.expect("")).output().expect("");
            if cut2.stdout.contains(&b'w') {
                println!("{} {} {}", "[+]".green(), String::from_utf8_lossy(&cut2.stdout).trim(), "yes!".green());
            }else{
                println!("{} {} {}", "[-]".red(), String::from_utf8_lossy(&cut2.stdout).trim(), "nope!".red());
            }
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    println!("{} Can write to /etc/shadow?", "[+]".green());
    let mut cmd = Command::new("ls");
    match cmd.output() {
        Ok(_status) => {
            let ls = Command::new("ls").arg("-l").arg("/etc/shadow").stdout(Stdio::piped()).spawn().expect("");
            let cut1 = Command::new("cut").arg("-d").arg(" ").arg("-f").arg("1").stdin(ls.stdout.expect("")).stdout(Stdio::piped()).spawn().expect("");
            let cut2 = Command::new("cut").arg("-c").arg("9-").stdin(cut1.stdout.expect("")).output().expect("");
            if cut2.stdout.contains(&b'w') {
                println!("{} {} {}", "[+]".green(), String::from_utf8_lossy(&cut2.stdout).trim(), "yes!".green());
            }else{
                println!("{} {} {}", "[-]".red(), String::from_utf8_lossy(&cut2.stdout).trim(), "nope!".red());
            }
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    section_label("Current user crontab");
    let mut cmd = Command::new("cat");
    match cmd.output() {
        Ok(_status) => {
            let cron = Command::new("cat").arg("/etc/crontab").output().expect("");
            println!("{}", String::from_utf8_lossy(&cron.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    section_label("/etc/cron.d");
    let mut cmd = Command::new("ls");
    match cmd.output() {
        Ok(_status) => {
            let cron = Command::new("ls").arg("-la").arg("/etc/cron.d").output().expect("");
            println!("{}", String::from_utf8_lossy(&cron.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    section_label("/etc/cron.hourly");
    let mut cmd = Command::new("ls");
    match cmd.output() {
        Ok(_status) => {
            let cron = Command::new("ls").arg("-la").arg("/etc/cron.hourly").output().expect("");
            println!("{}", String::from_utf8_lossy(&cron.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    section_label("/etc/cron.daily");
    let mut cmd = Command::new("ls");
    match cmd.output() {
        Ok(_status) => {
            let cron = Command::new("ls").arg("-la").arg("/etc/cron.daily").output().expect("");
            println!("{}", String::from_utf8_lossy(&cron.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    section_label("/etc/cron.weekly");
    let mut cmd = Command::new("ls");
    match cmd.output() {
        Ok(_status) => {
            let cron = Command::new("ls").arg("-la").arg("/etc/cron.weekly").output().expect("");
            println!("{}", String::from_utf8_lossy(&cron.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    section_label("/etc/cron.monthly");
    let mut cmd = Command::new("ls");
    match cmd.output() {
        Ok(_status) => {
            let cron = Command::new("ls").arg("-la").arg("/etc/cron.monthly").output().expect("");
            println!("{}", String::from_utf8_lossy(&cron.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

}