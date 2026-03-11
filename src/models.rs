/// Représente l'état d'avancement d'une tâche.
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    /// La tâche est créée mais pas encore terminée.
    Pending,
    /// La tâche a été validée par l'utilisateur.
    Done,
}

/// Structure principale représentant une tâche unique dans l'application.

pub struct Task {
    /// Identifiant numérique unique.
    pub id: u32,
    /// Nom de la tâche.
    pub title: String,
    /// État actuel (Pending ou Done).
    pub status: Status,
}

impl Task {
    /// Crée une nouvelle instance de 'Task'.
    ///
    /// Par défaut, toute nouvelle tâche est initialisée avec le statut 'Status::Pending'.
    ///
    /// # Arguments
    /// * 'id' - L'identifiant unique à attribuer.
    /// * 'title' - Le titre de la tâche.
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            status: Status::Pending,
        }
    }
    /// Affiche les informations de la tâche dans la console de manière formatée.
    /// 
    /// Le format de sortie est : 'ID - Titre - Statut'
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
    /// Vérifie qu'une tâche créée possède bien les bonnes valeurs initiales.
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