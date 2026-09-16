use std::{fs, process::Command};

fn main() {
    println!("Сборка wasm");
    let status = Command::new("cargo").args(&["build", "--release", "--target", "wasm32-unknown-unknown"])
        .status().expect("Ошибка запуска cargo build");
    if !status.success() {
        println!("Ошибка сборки");
        std::process::exit(1);
    }

    if !fs::metadata("deploy").is_ok() {
        println!("Отсутствует директория deploy");
    }

    let source = "target/wasm32-unknown-unknown/release/team-lead-simulator.wasm";
    let target = "deploy/game.wasm";

    fs::copy(source, target).expect("Ошибка копирования .wasm файла");

    println!("Сборка готова! Запуск сервера...");
    
    let status = Command::new("basic-http-server").args(&["-a", "0.0.0.0:6767", "deploy/"])
        .status().expect("Ошибка запуса basic-http-server");
    if !status.success() {
        std::process::exit(2);
    }
}
