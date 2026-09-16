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


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameProject {
    pub design_progress: f32,
    pub backend_progress: f32,
    pub frontend_progress: f32,
    pub mobile_progress: f32,
    pub deploy_progress: f32,
    hidden_bugs: Vec<Bug>,
    pub known_bugs: Vec<Bug>,
    pub employees: Vec<ManagedEmployee>,
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
            employees: Vec::new()
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
pub enum Position {
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
pub struct Employee {
    pub name: String,
    pub age: u16,
    pub position: Position,
    pub grade: Grade,
    pub salary: f32,
    speed: f32,
    quality: f32,
    burnout_coef: f32
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManagedEmployee {
    pub base_employee: Employee,
    pub grade: Grade,
    pub education_progress: f32,
    burnout: f32,
    work_progress: f32,
    pub current_bug: Option<usize>
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Difficulty {
    Tutorial,
    Easy,
    Normal,
    Hard,
    Extreme
}


#[derive(Debug, Clone)]
pub struct Game {
    pub difficulty: Difficulty,
    pub project: GameProject,
}


impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            difficulty,
            project: GameProject::new(),
        }
    }
    
    pub fn next_day(&mut self) {

        let project = &mut self.project;
        
        let mut employees = std::mem::take(&mut project.employees);

        for employee in employees.iter_mut() {
            employee.work_progress += employee.base_employee.speed;
            employee.burnout += employee.base_employee.burnout_coef;
            
            let work_done = employee.work_progress.trunc();

            match employee.base_employee.position {
                Position::Backend => {
                    if employee.current_bug.is_none() {
                        let bug_index = project.known_bugs.iter()
                            .position(|bug| bug.bug_type == BugType::Backend && bug.is_available);
                        if let Some(index) = bug_index {
                            project.known_bugs[index].is_available = false;
                            employee.current_bug = Some(index);
                        } else {
                            project.backend_progress += work_done;
                        }
                    } else {
                        if let Some(bug_index) = employee.current_bug {
                            let bug = &mut project.known_bugs[bug_index];
                            bug.complexity -= work_done;

                            if bug.complexity <= 0.0 {
                                employee.current_bug = None;
                                project.known_bugs.remove(bug_index);
                            }
                        }
                    }
                }
                Position::Frontend => {
                    if employee.current_bug.is_none() {
                        let bug_index = project.known_bugs.iter()
                            .position(|bug| bug.bug_type == BugType::Frontend && bug.is_available);
                        if let Some(index) = bug_index {
                            project.known_bugs[index].is_available = false;
                            employee.current_bug = Some(index);
                        } else {
                            project.frontend_progress += work_done;
                        }
                    } else {
                        if let Some(bug_index) = employee.current_bug {
                            let bug = &mut project.known_bugs[bug_index];
                            bug.complexity -= work_done;

                            if bug.complexity <= 0.0 {
                                employee.current_bug = None;
                                project.known_bugs.remove(bug_index);
                            }
                        }
                    }
                }
                Position::Mobile => {
                    if employee.current_bug.is_none() {
                        let bug_index = project.known_bugs.iter()
                            .position(|bug| bug.bug_type == BugType::Mobile && bug.is_available);
                        if let Some(index) = bug_index {
                            project.known_bugs[index].is_available = false;
                            employee.current_bug = Some(index);
                        } else {
                            project.mobile_progress += work_done;
                        }
                    } else {
                        if let Some(bug_index) = employee.current_bug {
                            let bug = &mut project.known_bugs[bug_index];
                            bug.complexity -= work_done;

                            if bug.complexity <= 0.0 {
                                employee.current_bug = None;
                                project.known_bugs.remove(bug_index);
                            }
                        }
                    }
                }
                Position::DevOps | Position::Sysadmin => {
                    if employee.current_bug.is_none() {
                        let bug_index = project.known_bugs.iter()
                            .position(|bug| bug.bug_type == BugType::Server && bug.is_available);
                        if let Some(index) = bug_index {
                            project.known_bugs[index].is_available = false;
                            employee.current_bug = Some(index);
                        } else {
                            project.deploy_progress += work_done;
                        }
                    } else {
                        if let Some(bug_index) = employee.current_bug {
                            let bug = &mut project.known_bugs[bug_index];
                            bug.complexity -= work_done;

                            if bug.complexity <= 0.0 {
                                employee.current_bug = None;
                                project.known_bugs.remove(bug_index);
                            }
                        }
                    }

                }
                Position::UIUXDesigner => {
                    project.design_progress += work_done;
                }
                Position::AutoQA | Position::ManualQA => {
                    let bugs_to_reveal = (work_done as usize)
                        .min(project.hidden_bugs.len());
                    for _ in 0..bugs_to_reveal {
                        if let Some(bug) = project.hidden_bugs.pop() {
                            project.known_bugs.push(bug);
                        }
                    }
                }
            }
            employee.work_progress -= work_done;
        }

        project.employees = employees;
    }
}


pub fn load_employees() -> Vec<Employee> {
    const BYNARY_DATA: &[u8] = include_bytes!("../game-data/employees.mpk");
    let employees: Vec<Employee> = rmp_serde::from_slice(BYNARY_DATA)
        .expect("Не удалось распарсить employees.mpk");

    employees
}
