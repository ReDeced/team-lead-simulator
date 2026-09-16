use serde::{Deserialize, Serialize};


#[derive(Serialize, PartialEq, Deserialize, Debug, Clone, Copy)]
pub enum BugType {
    Backend,
    Frontend,
    Mobile,
    Server
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bug {
    pub bug_type: BugType,
    pub complexity: f32,
    pub is_available: bool
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Difficulty {
    Tutorial,
    Easy,
    Normal,
    Hard,
    Extreme
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub difficulty: Difficulty,
    pub project: GameProject,
    pub developers: Vec<Developer>
}


impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            difficulty,
            project: GameProject::new(),
            developers: load_developers()
        }
    }

    pub fn next_day(&mut self) {
        let project = &mut self.project;
        
        let mut developers = std::mem::take(&mut project.developers);

        for developer in developers.iter_mut() {
            developer.work_progress += developer.base_developer.speed;
            developer.burnout += developer.base_developer.burnout_coef;
            
            let work_done = developer.work_progress.trunc();

            match developer.base_developer.position {
                DevPosition::Backend => {
                    if developer.current_bug.is_none() {
                        let bug_index = project.known_bugs.iter()
                            .position(|bug| bug.bug_type == BugType::Backend && bug.is_available);
                        if let Some(index) = bug_index {
                            project.known_bugs[index].is_available = false;
                            developer.current_bug = Some(index);
                        } else {
                            project.backend_progress += work_done;
                        }
                    } else {
                        if let Some(bug_index) = developer.current_bug {
                            let bug = &mut project.known_bugs[bug_index];
                            bug.complexity -= work_done;

                            if bug.complexity <= 0.0 {
                                developer.current_bug = None;
                                project.known_bugs.remove(bug_index);
                            }
                        }
                    }
                }
                DevPosition::Frontend => {
                    if developer.current_bug.is_none() {
                        let bug_index = project.known_bugs.iter()
                            .position(|bug| bug.bug_type == BugType::Frontend && bug.is_available);
                        if let Some(index) = bug_index {
                            project.known_bugs[index].is_available = false;
                            developer.current_bug = Some(index);
                        } else {
                            project.frontend_progress += work_done;
                        }
                    } else {
                        if let Some(bug_index) = developer.current_bug {
                            let bug = &mut project.known_bugs[bug_index];
                            bug.complexity -= work_done;

                            if bug.complexity <= 0.0 {
                                developer.current_bug = None;
                                project.known_bugs.remove(bug_index);
                            }
                        }
                    }
                }
                DevPosition::Mobile => {
                    if developer.current_bug.is_none() {
                        let bug_index = project.known_bugs.iter()
                            .position(|bug| bug.bug_type == BugType::Mobile && bug.is_available);
                        if let Some(index) = bug_index {
                            project.known_bugs[index].is_available = false;
                            developer.current_bug = Some(index);
                        } else {
                            project.mobile_progress += work_done;
                        }
                    } else {
                        if let Some(bug_index) = developer.current_bug {
                            let bug = &mut project.known_bugs[bug_index];
                            bug.complexity -= work_done;

                            if bug.complexity <= 0.0 {
                                developer.current_bug = None;
                                project.known_bugs.remove(bug_index);
                            }
                        }
                    }
                }
                DevPosition::DevOps | DevPosition::Sysadmin => {
                    if developer.current_bug.is_none() {
                        let bug_index = project.known_bugs.iter()
                            .position(|bug| bug.bug_type == BugType::Server && bug.is_available);
                        if let Some(index) = bug_index {
                            project.known_bugs[index].is_available = false;
                            developer.current_bug = Some(index);
                        } else {
                            project.deploy_progress += work_done;
                        }
                    } else {
                        if let Some(bug_index) = developer.current_bug {
                            let bug = &mut project.known_bugs[bug_index];
                            bug.complexity -= work_done;

                            if bug.complexity <= 0.0 {
                                developer.current_bug = None;
                                project.known_bugs.remove(bug_index);
                            }
                        }
                    }

                }
                DevPosition::UIUXDesigner => {
                    project.design_progress += work_done;
                }
                DevPosition::AutoQA | DevPosition::ManualQA => {
                    let bugs_to_reveal = (work_done as usize)
                        .min(project.hidden_bugs.len());
                    for _ in 0..bugs_to_reveal {
                        if let Some(bug) = project.hidden_bugs.pop() {
                            project.known_bugs.push(bug);
                        }
                    }
                }
            }
            developer.work_progress -= work_done;
        }

        project.developers = developers;
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameProject {
    pub design_progress: f32,
    pub backend_progress: f32,
    pub frontend_progress: f32,
    pub mobile_progress: f32,
    pub deploy_progress: f32,
    hidden_bugs: Vec<Bug>,
    pub known_bugs: Vec<Bug>,
    pub developers: Vec<ManagedDeveloper>,
}


impl GameProject {
    pub fn new() -> Self {
        Self {
            design_progress: 0.0,
            backend_progress: 0.0,
            frontend_progress: 0.0,
            mobile_progress: 0.0,
            deploy_progress: 0.0,
            hidden_bugs: Vec::new(),
            known_bugs: Vec::new(),
            developers: Vec::new()
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Grade {
    Junior,
    Middle,
    Senior
}


#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Copy, strum::EnumIter)]
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


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Developer {
    pub name: String,
    pub age: u16,
    pub position: DevPosition,
    pub grade: Grade,
    pub salary: f32,
    speed: f32,
    quality: f32,
    burnout_coef: f32
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManagedDeveloper {
    pub base_developer: Developer,
    pub grade: Grade,
    pub education_progress: f32,
    burnout: f32,
    work_progress: f32,
    pub current_bug: Option<usize>
}


pub fn load_developers() -> Vec<Developer> {
    const BYNARY_DATA: &[u8] = include_bytes!("../game-data/developers.mpk");
    let developers: Vec<Developer> = rmp_serde::from_slice(BYNARY_DATA)
        .expect("Не удалось распарсить developers.mpk");

    developers
}
