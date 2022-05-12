use std::process::{Command, Stdio};
use colored::Colorize;
// use std::io::stdout;
// use std::io::Write;

use crate::linux::banner::section_label;
use crate::linux::banner::module_label;
use aho_corasick::AhoCorasick;

pub fn get_info() {

    // println!("{}","                                         ╔═══════════════════╗".blue());
    // println!("{}{}{}","═════════════════════════════════════════╣ ".blue(),"Basic information".green()," ╠═════════════════════════════════════════".blue());
    // println!("{}","                                         ╚═══════════════════╝".blue());
    
    module_label("Basic Information");
    let whoami = Command::new("whoami")
        .output()
        .expect("cmd failed to be executed.");
    let whoami = String::from_utf8_lossy(&whoami.stdout);
    if whoami.trim().contains("root") {
        println!("{} Current User: {}", "[+]".green(), whoami.red());
    }else{
        println!("{} Current User: {}", "[+]".green(), whoami.green());
    }
    
    let _critical_groups = &["root", "sudo", "shadow", "admin", "adm", "wheel", "auth"] ;
    let _critical_groups_highlight = &["root".red().to_string(), "sudo".red().to_string(), "shadow".red().to_string(),"admin".red().to_string(),"adm".red().to_string(),"wheel".red().to_string(),"auth".red().to_string()] ;
    let _interesting_groups = &["docker", "lxc", "lxd", "disk", "cdrom"] ;
    let _interesting_groups_highlight = &["docker".cyan().to_string(), "lxc".cyan().to_string(), "lxd".cyan().to_string(), "disk".cyan().to_string(), "cdrom".cyan().to_string()] ;
    let mut cmd = Command::new("id");
    match cmd.output() {
        Ok(_status) => {
            let id = String::from_utf8_lossy(&_status.stdout);
            if id.trim().contains("root") {
                let str_match = &whoami.trim();
                let id_output = id.replace(str_match, &str_match.red().to_string());
                let ac = AhoCorasick::new(_critical_groups);
                let id_output = ac.replace_all(&id_output, _critical_groups_highlight);
                let ac = AhoCorasick::new(_interesting_groups);
                let id_output = ac.replace_all(&id_output, _interesting_groups_highlight);
                println!("{} ID:\n{}", "[+]".green(), id_output);
            }else{
                let str_match = &whoami.trim();
                let id_output = id.replace(str_match, &str_match.green().to_string());
                let ac = AhoCorasick::new(_critical_groups);
                let id_output = ac.replace_all(&id_output, _critical_groups_highlight);
                let ac = AhoCorasick::new(_interesting_groups);
                let id_output = ac.replace_all(&id_output, _interesting_groups_highlight);
                println!("{} ID:\n{}", "[+]".green(), id_output);
            };
            //couldnt figure out how to do a multi-string replace with different colors in the output from above.  Only works per match and then overwrites...hence this hack.
                // let id = String::from_utf8_lossy(&id.stdout);
                // let mut id_output = String::new();
                // for group in interesting_groups.iter() {
                //     let str_match = group;
                //     let id_output = id.replace(str_match, &str_match.cyan().to_string());
                // }
                // println!("{}", id_output);
            // println!("{} Elevated group membership:", "[+]".green());
            // for group in _critical_groups.iter() {
            //     if id.trim().contains(group) {
            //         println!("{} ", group.red());
            //     }
            // }
            // println!("\n{} Interesting group membership:", "[+]".green());
            // for group in _interesting_groups.iter() {
            //     if id.trim().contains(group) {
            //         print!("{} ", group.cyan());
            //         // stdout().flush();
            //     }
            // }

        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    let mut cmd = Command::new("hostname");
    match cmd.output() {
        Ok(_status) => {
            println!("{} Hostname: {}", "[+]".green(), String::from_utf8_lossy(&_status.stdout).cyan());
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    let mut cmd = Command::new("date");
    match cmd.output() {
        Ok(_status) => {
            println!("{} Time: {}", "[+]".green(), String::from_utf8_lossy(&_status.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    let mut cmd = Command::new("w");
    match cmd.output() {
        Ok(_status) => {
            println!("{} Logged in users: {}", "[+]".green(), String::from_utf8_lossy(&_status.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    let mut cmd = Command::new("uname");
    match cmd.output() {
        Ok(_status) => {
            let kernel = Command::new("uname").arg("-a").output().expect("");
            println!("{} Kernel Version: {}", "[+]".green(), String::from_utf8_lossy(&kernel.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }
    
    println!("{} System Protections:", "[+]".green());
    let mut cmd = Command::new("cat");
    match cmd.output() {
        Ok(_status) => {
            let mmap = Command::new("cat").arg("/proc/sys/vm/mmap_min_addr").output().expect("");
            if mmap.stdout.contains(&b'0') {
                println!("Mmap:{}", "Disabled".red());
            }else{
                println!("Mmap:{}", "Enabled".green());
            }
            
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    let mut cmd = Command::new("cat");
    match cmd.output() {
        Ok(_status) => {
            let aslr = Command::new("cat").arg("/proc/sys/kernel/randomize_va_space").output().expect("");
            if aslr.stdout.contains(&b'0') {
                println!("ASLR:{}", "Disabled".red());
            }else{
                println!("ASLR:{}",  "Enabled".green());
            }
            
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    let mut cmd = Command::new("cat");
    match cmd.output() {
        Ok(_status) => {
            let kptr = Command::new("cat").arg("/proc/sys/kernel/kptr_restrict").output().expect("");
            if kptr.stdout.contains(&b'0') {
                println!("KPTR:{}", "Disabled".red());
            }else{
                println!("KPTR:{}", "Enabled".green());
            }
            
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    let mut cmd = Command::new("cat");
    match cmd.output() {
        Ok(_status) => {
            let release = Command::new("cat").arg("/etc/lsb-release").output().expect("");
            println!("\n{} Distribution Details:\n{}", "[+]".green(), String::from_utf8_lossy(&release.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }

    section_label("Running processes");
    let mut cmd = Command::new("ps");
    match cmd.output() {
        Ok(_status) => {
            let cron = Command::new("ps").arg("-aux").output().expect("");
            println!("{}", String::from_utf8_lossy(&cron.stdout));
        }
        Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
    }
}
