use std::{fs::File, io::Write};


#[allow(unused_variables)]
fn main() {
    print!("cargo:rerun-if-changed=game-data");
    let json_file = File::open("game-data/developers.json").expect("Не удалось открыть JSON");
    let json_value: serde_json::Value = serde_json::from_reader(json_file).expect("Ошибка валидации JSON");
    let mpk_bytes = rmp_serde::to_vec(&json_value).expect("Ошибка валидации в MessagePack");
    let mut mpk_file = File::create("game-data/developers.mpk").expect("Не удалось создать mpk файл");
    mpk_file.write_all(&mpk_bytes).expect("Ошибка записи байт");
}

