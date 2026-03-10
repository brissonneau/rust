pub enum Status {
    Pending,
    Done,
}

pub struct Task {
    pub id: u32,
    pub title: String,
    pub status: Status,
}

impl Task {
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            status: Status::Pending,
        }
    }

    pub fn display(&self) {
        let check = match self.status {
            Status::Pending => "En Cours",
            Status::Done => "Complété",
        };
        println!("{} - {} - {}", self.id, self.title, check);
    }
}


#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_nouvelle_tache_est_pending() {
        let t = Task::new(1, "Apprendre les tests".to_string());
        assert_eq!(t.id, 1);
        match t.status {
            Status::Pending => assert!(true),
            _ => panic!("La tâche devrait être en attente !"),
        }
    }
}