#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Role {
        let su = s.to_lowercase().to_owned();
        match su {
            val if val == "ceo".to_string() => Role::CEO,
            val if val == "manager".to_string() => Role::Manager,
            val if val == "worker".to_string() => Role::Worker,
            _ => Role::Worker, // default case for any other string
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        WorkEnvironment {
            grade: None,
        }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let new_worker = Worker {
            name: name.to_string(),
            role: Role::from(role),
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(new_worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(worker) = self.grade.take() {
            self.grade = worker.next;
            Some(worker.name)
        } else {
            None
        }
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        let mut current = &self.grade;
        let mut last_worker = None;

        while let Some(worker) = current {
            last_worker = Some((worker.name.clone(), worker.role.clone()));
            current = &worker.next;
        }

        last_worker
    }
}
