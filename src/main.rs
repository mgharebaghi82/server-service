use std::{fs, io, process::Command};

fn main() {
    let mut file = fs::File::create("/etc/systemd/system/server-service.service").unwrap();
    let mut source = fs::File::open("server.service").unwrap();
    io::copy(&mut source, &mut file).unwrap();

    let command = Command::new("systemctl")
        .arg("daemon-reload")
        .status()
        .unwrap();
    println!("{}", command);

    let enable = Command::new("systemctl")
        .arg("enable")
        .arg("server-service")
        .status()
        .unwrap();
    println!("start status: {}", enable);

    let start = Command::new("systemctl")
        .arg("start")
        .arg("server-service")
        .status()
        .unwrap();
    println!("start status: {}", start);
    let status = Command::new("systemctl")
        .arg("status")
        .arg("server-service")
        .status()
        .unwrap();
    println!("status code: {}", status);
}
