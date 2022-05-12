// use colored::Colorize;

mod linux;

fn main() {
    linux::banner::banner();
    linux::basic_info::get_info();
    linux::net_info::get_info();
    linux::sys_info::get_info();
}
