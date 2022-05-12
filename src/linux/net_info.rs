use std::process::{Command, Stdio};
use colored::Colorize;
use std::env;
use crate::linux::banner::section_label;
use crate::linux::banner::module_label;

pub fn get_info() {
    module_label("Network Information");
    section_label("NIC Interfaces");
    let mut cmd = Command::new("ifconfig");
    match cmd.output() {
    Ok(_status) => {
        // let kernel = Command::new("uname").arg("-a").output().expect("");
        // println!("{} Network Information:\n\n{}", "[+]".green(), String::from_utf8_lossy(&_status.stdout));
        println!("{}", String::from_utf8_lossy(&_status.stdout));
    }
    Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
}
    section_label("DNS");
    let mut cmd = Command::new("cat");
    match cmd.output() {
    Ok(_status) => {
        let resolv = Command::new("cat").arg("/etc/resolv.conf").output().expect("");
        println!("{}", String::from_utf8_lossy(&resolv.stdout));
    }
    Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
}
    section_label("ARP Table");
    let mut cmd = Command::new("arp");
    match cmd.output() {
    Ok(_status) => {
        let arp = Command::new("arp").arg("-a").output().expect("");
        println!("{}", String::from_utf8_lossy(&arp.stdout));
    }
    Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
}
    section_label("Hosts");
    let mut cmd = Command::new("cat");
    match cmd.output() {
    Ok(_status) => {
        let hosts = Command::new("cat").arg("/etc/hosts").output().expect("");
        println!("{}", String::from_utf8_lossy(&hosts.stdout));
    }
    Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
}
    section_label("Listening ports");
    let mut cmd = Command::new("netstat");
    match cmd.output() {
    Ok(_status) => {
        let netstat = Command::new("netstat").arg("-tuanp").stdout(Stdio::piped()).spawn().expect("");
        let grep = Command::new("grep").arg("LISTEN").stdin(netstat.stdout.expect("")).output().expect("");
        println!("{}", String::from_utf8_lossy(&grep.stdout));
    }
    Err(e) => println!("{} Error attempting to execute: {:?}. {}", "[!]".yellow(), cmd.get_program(), e),
}
}