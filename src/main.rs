        // 3.4

/*fn main() {
    
    let name = "Utilisateur";
    let app_name = "Task-CLI";

   
    println!("   {}", app_name);
    println!("   Bienvenue, {} !", name);
    
    println!("\nVoici vos tâches du jour :");
    println!("1. Apprendre les bases de Rust");
    println!("2. Configurer votre environnement");
}

        // 4.1

/*fn main() {
    let a = 0;
    a = 1; 
}*/

fn main() {
    let mut a = 0;
    a = 1; 
    println!("Nombre : {}", a);
    let space = "   ";
    let space = space.len();
}

        //4.4

fn main() {

let mut titre_tache = "Apprendre le Rust";
println!("Tâche initiale : {}", titre_tache);

titre_tache = "Maîtriser le Rust"; 

println!("Ma tâche actuelle est : {}", titre_tache);
}

        //5.3

fn main() {
    afficher_bienvenue();

    let mut titre_tache = "Apprendre le Rust";
    let mut id = 1;

    afficher_tache(id, titre_tache);

    titre_tache = "Maîtriser le Rust";
    id = 2;
    
    afficher_tache(id, titre_tache);
}

fn afficher_bienvenue() {
    println!("BIENVENUE DANS TASK-CLI");
}

fn afficher_tache(id: i32, titre: &str) {
    println!("[Tâche #{}] : {}", id, titre);
}

        //5.4

fn main() {
    let ma_nouvelle_tache = create_task("Maîtriser le Rust");
    
    println!("[Notification] {}", ma_nouvelle_tache);
}

fn create_task(titre: &str) -> String {
    println!("Création d'une nouvelle tâche en cours...");
    
    format!("Nouvelle tâche : {}", titre)
}

        //6.1

fn main() {
    let a = 1;

    if a == 1 {
        println!("a est égal à 1");
    } else {
        println!("a n'est pas égal à 1");
    }

}

        //6.2

fn main() {
    let position = "2eme";

match position {
    "1er" => println!("Médaille d'or !"),
    "2eme" => println!("Médaille d'argent !"),
    "3eme" => println!("Médaille de bronze !"),
    _ => println!("Pas de médaille..."), 
}
}

        //6.3

        //6.4

use std::io; 

fn main() {
    println!("Création d'une nouvelle tâche");
    println!("Entrez le titre de la tâche :");

    let mut saisie = String::new();

    io::stdin()
        .read_line(&mut saisie)
        .expect("Erreur lors de la lecture");

    let titre = saisie.trim();
    
    if titre.is_empty() {
        println!("Erreur : Le titre ne peut pas être vide !");
    } else {
        println!("Tâche '{}' enregistrée avec succès !", titre);
        println!("Voici la liste des tâches enregistrées : ");
        println!("1. {}", titre);
    }
}

        // 7.4, fin du module 1, code complet de ce module

use std::io; 
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            completed: false,
        }
    }

    
    fn display(&self) {
        let check = if self.completed { "Complété" } else { "En Cours" };
        println!("{} - {} - {}",self.id, self.title, check);
    }
}

fn main() {
    let mut list: Vec<Task> = Vec::new();
    let mut id = 1;

    println!("BIENVENUE DANS TASK-CLI");

    loop {
        println!("\nEntrez une nouvelle tâche (ou tapez 'quitter' pour voir la liste et partir) :");

        
        let mut saisie = String::new();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur de lecture");

        let titre = saisie.trim();

        
        if titre == "quitter" {
            break;
        }

        if !titre.is_empty() {
            
            let nouvelle_tache = Task::new(id, titre.to_string());
            list.push(nouvelle_tache);
            
            println!("Tâche ajoutée !");
            id += 1; 
        } else {
            println!("Le titre ne peut pas être vide.");
        }
    }

    
    println!("\nListe de Tâches : ");
    for task in &list {
        task.display();
    }
}


            //8.3

use std::io; 
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            completed: false,
        }
    }

    
    fn display(&self) {
        let check = if self.completed { "Complété" } else { "En Cours" };
        println!("{} - {} - {}",self.id, self.title, check);
    }
}

fn main() {
    let mut list: Vec<Task> = Vec::new();
    let mut id = 1;

    println!("BIENVENUE DANS TASK-CLI");

    loop {
        println!("\nEntrez une nouvelle tâche (ou tapez 'quitter' pour voir la liste et partir) :");

        
        let mut saisie = String::new();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur de lecture");

        let titre = saisie.trim();

        
        if titre == "quitter" {
            break;
        }

        if !titre.is_empty() {

            let titre = titre.to_string();
            
            let nouvelle_tache = Task::new(id, titre);
            list.push(nouvelle_tache);
            
            println!("Tâche {} ajoutée !", titre);
            id += 1; 
        } else {
            println!("Le titre ne peut pas être vide.");
        }
    }

    
    println!("\nListe de Tâches : ");
    for task in &list {
        task.display();
    }
}*/

            //10.4

use std::io; 
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            completed: false,
        }
    }

    
    fn display(&self) {
        let check = if self.completed { "Complété" } else { "En Cours" };
        println!("{} - {} - {}",self.id, self.title, check);
    }
}

fn main() {
    let mut list: Vec<Task> = Vec::new();
    let mut id = 1;

    println!("BIENVENUE DANS TASK-CLI");

    loop {
        println!("\nEntrez une nouvelle tâche, son ID pour la modifier ou 'liste' pour voir la liste des tâches 
        (ou tapez 'quitter' pour partir) :");

        
        let mut saisie = String::new();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur de lecture");

        let titre = saisie.trim();

        
        if titre == "quitter" {
            break;
        }

        else if titre == "liste" {
            println!("\nListe de Tâches : ");
            for task in &list {
            task.display();
            }
        }

        else if let Ok(id_validation) = titre.parse::<u32>() {
            for task in &mut list {
            if task.id == id_validation {
                task.completed = true;
                println!("Tâche n°{} terminée !", id_validation);
            }
        }
        }

        else if !titre.is_empty() {
            
            let nouvelle_tache = Task::new(id, titre.to_string());
            list.push(nouvelle_tache);
            
            println!("Tâche ajoutée !");
            id += 1; 
        } else {
            println!("Le titre ne peut pas être vide.");
        }
    }

    
    
}
