struct AISolutionArchitectSkills{
    devops_skills: String,
    system_design: String,
    design_pattern: String,
    database: String,
    machine_learning: String,
    programming:String,
    ielts: f32
}

impl AISolutionArchitectSkills{

    // Constructor method
    fn new(
        devops_skills: String, 
        system_design: String, 
        design_pattern: String, 
        database: String,
        machine_learning: String, 
        programming: String, 
        ielts: f32
    ) -> Self {
        Self {
            devops_skills,
            system_design,
            design_pattern,
            database,
            machine_learning,
            programming,
            ielts,
        }
    }

    fn from_info(other: AISolutionArchitectSkills) -> AISolutionArchitectSkills{
        return AISolutionArchitectSkills{
            devops_skills: other.devops_skills,
            system_design: other.system_design,
            design_pattern: other.design_pattern,
            database: other.database,
            machine_learning: other.machine_learning,
            programming: other.programming,
            ielts: other.ielts,
        }
    }

    fn update_info(&mut self, other: AISolutionArchitectSkills){
        self.devops_skills = other.devops_skills;
        self.system_design = other.system_design;
        self.design_pattern = other.design_pattern;
        self.database =other.database;
        self.machine_learning = other.machine_learning;
        self.programming = other.programming;
        self.ielts = other.ielts;
    }

    fn display(&self){
        println!("
                DevOps Skills: {}\n
                System Design: {}\n
                Design Pattern: {}\n
                Database: {}\n
                Machine Learning: {}\n
                Programming: {}\n
                IELTS: {}
            ", 
            self.devops_skills, 
            self.system_design, 
            self.design_pattern, 
            self.database, 
            self.machine_learning, 
            self.programming, 
            self.ielts
        );
    }
}

fn main(){
    let mut leon = AISolutionArchitectSkills{
        devops_skills: String::from("Docker, K8s"),
        system_design: String::from("Load balancer, Caching"),
        design_pattern: String::from("Singleton, Factory"),
        database: String::from("MSSQL, PostgreSQL"),
        machine_learning: String::from("UDA, LLM, LVM"),
        programming: String::from("Python, Rust"),
        ielts: 8.0
    };

    leon.display();

    let nina = AISolutionArchitectSkills{
        devops_skills: String::from("Docker"),
        system_design: String::from("Load balancer"),
        design_pattern: String::from("Builder"),
        database: String::from("MySQL"),
        machine_learning: String::from("Supervised Learning"),
        programming: String::from("C++"),
        ielts: 9.0
    };

    leon.update_info(nina);
    leon.display();

    let steven = AISolutionArchitectSkills::from_info(leon);
    steven.display();
}