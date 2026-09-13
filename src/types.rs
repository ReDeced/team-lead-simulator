use serde::Deserialize;


#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Grade {
    Junior,
    Middle,
    Senior
}


#[derive(Deserialize, Debug, Clone, Copy)]
pub enum DevPosition {
    Backend,
    Frontend,
    DevOps,
    Sysadmin,
    Mobile,
    AutoQA,
    ManualQA,
    UIUXDesigner
}


#[derive(Deserialize, Debug, Clone)]
pub struct Developer {
    pub name: String,
    pub age: u16,
    pub position: DevPosition,
    pub grade: Grade,
    pub salary: f32,
    pub speed: f32,
    pub quality: f32,
    pub burnout_coef: f32
}


pub fn load_developers() -> Vec<Developer> {
    const BYNARY_DATA: &[u8] = include_bytes!("../game-data/developers.mpk");
    let developers: Vec<Developer> = rmp_serde::from_slice(BYNARY_DATA)
        .expect("Не удалось распарсить developers.mpk");

    developers
}
