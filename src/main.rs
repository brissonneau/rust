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
}

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
       
       //12.3. v1

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
            list.remove(0); 
            
            println!("Tâche supprimée !");
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
       
            //12.3 vFinale

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

        else if saisie.starts_with("suppr ") {
            let reste = saisie.replace("suppr ", "");
            
            
            match reste.trim().parse::<u32>() {
                Ok(id_a_supprimer) => {
                    list.retain(|t| t.id != id_a_supprimer);
                    println!("Tâche n°{} supprimée !", id_a_supprimer);
                }
                Err(_) => {
                    println!("Erreur : Veuillez entrer un ID valide après 'suppr'");
                }
            }
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
        //13.2
fn main() {
    let r;                // r est déclarée

    {                     // Début du scope
        let x = 5;        // x est créée ici
        r = &x;           // r pointe vers x
    }                     //  x est détruite ici (Fin du scope)
                          
    println!("r: {}", r); 
}
    


            //14.3

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

fn trouver_tache<'a>(liste: &'a Vec<Task>, id_recherche: u32) -> Option<&'a Task> {
    for task in liste {
        if task.id == id_recherche {
            return Some(task); 
        }
    }
    None
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

        else if saisie.starts_with("suppr ") {
            let reste = saisie.replace("suppr ", "");

        
            
            
            match reste.trim().parse::<u32>() {
                Ok(id_a_supprimer) => {
                    list.retain(|t| t.id != id_a_supprimer);
                    println!("Tâche n°{} supprimée !", id_a_supprimer);
                }
                Err(_) => {
                    println!("Erreur : Veuillez entrer un ID valide après 'suppr'");
                }
            }
        }

        else if saisie.starts_with("voir ") {
            let id_str = saisie.replace("voir ", "");
            if let Ok(id_cible) = id_str.trim().parse::<u32>() {
                if let Some(tache) = trouver_tache(&list, id_cible) {
                    print!("Focus sur : ");
                    tache.display();
                } else {
                    println!("ID {} introuvable.", id_cible);
                }
            }
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

            //16.1

use std::io;

enum Status {
    Pending,
    Done,
}
struct Task {
    id: u32,
    title: String,
    status: Status,
}

impl Task {
    fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            status: Status::Pending,
        }
    }
    

    
    fn display(&self) {
        let check = match self.status {
            Status::Pending => "En Cours",
            Status::Done => "Complété",
        };
        println!("{} - {} - {}",self.id, self.title, check);
    }
}

fn trouver_tache<'a>(liste: &'a Vec<Task>, id_recherche: u32) -> Option<&'a Task> {
    for task in liste {
        if task.id == id_recherche {
            return Some(task); 
        }
    }
    None
}

fn main() {
    let mut list: Vec<Task> = Vec::new();
    let mut id = 1;

    println!("BIENVENUE DANS TASK-CLI");

    loop {
        println!("\nEntrez une nouvelle tâche, son ID pour la modifier, 'suppr ID' pour la supprimer ou 'liste' pour voir la liste des tâches 
        (ou tapez 'quitter' pour partir) :");

        
        let mut saisie = String::new();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur de lecture");

        let titre = saisie.trim();

        
        if titre == "quitter" {
            break;
        }

        else if saisie.starts_with("suppr ") {
            let reste = saisie.replace("suppr ", "");

        
            
            
            match reste.trim().parse::<u32>() {
                Ok(id_a_supprimer) => {
                    list.retain(|t| t.id != id_a_supprimer);
                    println!("Tâche n°{} supprimée !", id_a_supprimer);
                }
                Err(_) => {
                    println!("Erreur : Veuillez entrer un ID valide après 'suppr'");
                }
            }
        }

        else if saisie.starts_with("voir ") {
            let id_str = saisie.replace("voir ", "");
            if let Ok(id_cible) = id_str.trim().parse::<u32>() {
                if let Some(tache) = trouver_tache(&list, id_cible) {
                    print!("Focus sur : ");
                    tache.display();
                } else {
                    println!("ID {} introuvable.", id_cible);
                }
            }
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
                task.status = Status::Done;
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

            // 17.1 + 17.3
use std::io;

enum Status {
    Pending,
    Done,
}
struct Task {
    id: u32,
    title: String,
    status: Status,
}

impl Task {
    fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            status: Status::Pending,
        }
    }
    

    
    fn display(&self) {
        let check = match self.status {
            Status::Pending => "En Cours",
            Status::Done => "Complété",
        };
        println!("{} - {} - {}",self.id, self.title, check);
    }
}

fn trouver_tache<'a>(liste: &'a Vec<Task>, id_recherche: u32) -> Option<&'a Task> {
    for task in liste {
        if task.id == id_recherche {
            return Some(task); 
        }
    }
    None
}

fn chercher_tache<'a>(liste: &'a [Task], mot_cle: &str) -> Option<&'a Task> {
    for task in liste {
        if task.title == mot_cle {
            return Some(task); 
        }
    }
    None 
}

fn main() {
    let mut list: Vec<Task> = Vec::new();
    let mut id = 1;

    println!("BIENVENUE DANS TASK-CLI");

    loop {
        println!("\nEntrez une nouvelle tâche, son ID pour la modifier, 'suppr ID' pour la supprimer ou 'liste' pour voir la liste des tâches 
        (ou tapez 'quitter' pour partir) :");

        
        let mut saisie = String::new();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur de lecture");

        let titre = saisie.trim();

        
        match titre {
            "quitter" => break,
            
            "liste" => {
                println!("\nListe de Tâches : ");
                for task in &list {
                    match task {
                        Task { id, title, status: Status::Done } => println!("[TERMINEE] №{} : {}", id, title),
                        Task { id, title, status: Status::Pending } => println!("[EN COURS] №{} : {}", id, title),
                    }
                }
            }

            s if s.starts_with("chercher ") => {
            let reste = s.replace("chercher ", "");
            
                match chercher_tache(&list, reste.trim()) {
                    Some(tache) => {
                        println!("Trouvé ! Voici le détail :");
                        tache.display();
                    },
                    None => println!("Désolé, aucune tâche ne s'appelle '{}'.", reste.trim()),
                }
            }

            s if s.starts_with("suppr ") => {
                let reste = s.replace("suppr ", "");
                if let Ok(id_a_supprimer) = reste.trim().parse::<u32>() {
                    list.retain(|t| t.id != id_a_supprimer);
                    println!("Tâche n°{} supprimée !", id_a_supprimer);
                }
            }

            s if s.starts_with("voir ") => {
                let reste = s.replace("voir ", "");
                if let Ok(id_cible) = reste.trim().parse::<u32>() {
                    if let Some(tache) = trouver_tache(&list, id_cible) {
                        print!("Focus sur : ");
                        tache.display();
                    } else {
                        println!("ID {} introuvable.", id_cible);
                    }
                }
            }

            s if s.parse::<u32>().is_ok() => {
                let id_val = s.parse::<u32>().unwrap();
                for task in &mut list {
                    if task.id == id_val {
                        task.status = Status::Done;
                        println!("Tâche n°{} terminée !", id_val);
                    }
                }
            }

            s if !s.is_empty() => {
                list.push(Task::new(id, s.to_string()));
                println!("Tâche ajoutée !");
                id += 1;
            }

            _ => println!("Le titre ne peut pas être vide."),
        }
    }

    
    
}
            //19.3
use std::io;

enum Status {
    Pending,
    Done,
}
struct Task {
    id: u32,
    title: String,
    status: Status,
}

impl Task {
    fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            status: Status::Pending,
        }
    }
    

    
    fn display(&self) {
        let check = match self.status {
            Status::Pending => "En Cours",
            Status::Done => "Complété",
        };
        println!("{} - {} - {}",self.id, self.title, check);
    }
}

fn valider_titre(titre: &str) -> Result<String, String> {
    if titre.len() < 3 {
        Err(String::from("Le titre doit faire au moins 3 caractères."))
    } else {
        Ok(titre.to_string())
    }
}

fn trouver_tache<'a>(liste: &'a Vec<Task>, id_recherche: u32) -> Option<&'a Task> {
    for task in liste {
        if task.id == id_recherche {
            return Some(task); 
        }
    }
    None
}

fn chercher_tache<'a>(liste: &'a [Task], mot_cle: &str) -> Option<&'a Task> {
    for task in liste {
        if task.title == mot_cle {
            return Some(task); 
        }
    }
    None 
}

fn main() {
    let mut list: Vec<Task> = Vec::new();
    let mut id = 1;

    println!("BIENVENUE DANS TASK-CLI");

    loop {
        println!("\nEntrez une nouvelle tâche, son ID pour la modifier, 'suppr ID' pour la supprimer ou 'liste' pour voir la liste des tâches 
        (ou tapez 'quitter' pour partir) :");

        
        let mut saisie = String::new();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur de lecture");

        let titre = saisie.trim();

        
        match titre {
            "quitter" => break,
            
            "liste" => {
                println!("\nListe de Tâches : ");
                for task in &list {
                    match task {
                        Task { id, title, status: Status::Done } => println!("[TERMINEE] №{} : {}", id, title),
                        Task { id, title, status: Status::Pending } => println!("[EN COURS] №{} : {}", id, title),
                    }
                }
            }

            s if s.starts_with("chercher ") => {
            let reste = s.replace("chercher ", "");
            
                match chercher_tache(&list, reste.trim()) {
                    Some(tache) => {
                        println!("Trouvé ! Voici le détail :");
                        tache.display();
                    },
                    None => println!("Désolé, aucune tâche ne s'appelle '{}'.", reste.trim()),
                }
            }

            s if s.starts_with("suppr ") => {
                let reste = s.replace("suppr ", "");
                if let Ok(id_a_supprimer) = reste.trim().parse::<u32>() {
                    list.retain(|t| t.id != id_a_supprimer);
                    println!("Tâche n°{} supprimée !", id_a_supprimer);
                }
            }

            s if s.starts_with("voir ") => {
                let reste = s.replace("voir ", "");
                if let Ok(id_cible) = reste.trim().parse::<u32>() {
                    if let Some(tache) = trouver_tache(&list, id_cible) {
                        print!("Focus sur : ");
                        tache.display();
                    } else {
                        println!("ID {} introuvable.", id_cible);
                    }
                }
            }

            s if s.parse::<u32>().is_ok() => {
                let id_val = s.parse::<u32>().unwrap();
                for task in &mut list {
                    if task.id == id_val {
                        task.status = Status::Done;
                        println!("Tâche n°{} terminée !", id_val);
                    }
                }
            }

            s if !s.is_empty() => {
                match valider_titre(s) {
                    Ok(titre_valide) => {
                        list.push(Task::new(id, titre_valide));
                        println!("Tâche ajoutée !");
                        id += 1;
                    },
                    Err(message) => println!("Erreur : {}", message),
                }
            }

            _ => println!("Le titre ne peut pas être vide."),
        }
    }

    
    
}

            //20.2

use std::io;

enum Status {
    Pending,
    Done,
}
struct Task {
    id: u32,
    title: String,
    status: Status,
}

impl Task {
    fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            status: Status::Pending,
        }
    }
    

    
    fn display(&self) {
        let check = match self.status {
            Status::Pending => "En Cours",
            Status::Done => "Complété",
        };
        println!("{} - {} - {}",self.id, self.title, check);
    }
}

fn valider_titre(titre: &str) -> Result<String, String> {
    if titre.len() < 3 {
        Err(String::from("Le titre doit faire au moins 3 caractères."))
    } else {
        Ok(titre.to_string())
    }
}

fn trouver_tache<'a>(liste: &'a Vec<Task>, id_recherche: u32) -> Option<&'a Task> {
    for task in liste {
        if task.id == id_recherche {
            return Some(task); 
        }
    }
    None
}

fn chercher_tache<'a>(liste: &'a [Task], mot_cle: &str) -> Option<&'a Task> {
    for task in liste {
        if task.title == mot_cle {
            return Some(task); 
        }
    }
    None 
}

fn executer_commande(titre: &str, list: &mut Vec<Task>, id: &mut u32) -> Result<(), String> {
    match titre {
        "quitter" => {
            Ok(())
        }

        "liste" => {
            println!("\nListe de Tâches : ");
            for task in list {
                match task {
                    Task { id, title, status: Status::Done } => println!("[TERMINEE] №{} : {}", id, title),
                    Task { id, title, status: Status::Pending } => println!("[EN COURS] №{} : {}", id, title),
                }
            }
            Ok(())
        }

        s if s.starts_with("chercher ") => {
            let reste = s.replace("chercher ", "");
            match chercher_tache(list, reste.trim()) {
                Some(tache) => {
                    println!("Trouvé ! Voici le détail :");
                    tache.display();
                }
                None => println!("Désolé, aucune tâche ne s'appelle '{}'.", reste.trim()),
            }
            Ok(())
        }

        s if s.starts_with("suppr ") => {
            let reste = s.replace("suppr ", "");
            let id_a_supprimer = reste.trim().parse::<u32>().map_err(|_| "L'ID doit être un nombre !")?;
            
            list.retain(|t| t.id != id_a_supprimer);
            println!("Tâche n°{} supprimée !", id_a_supprimer);
            Ok(())
        }

        s if s.starts_with("voir ") => {
            let reste = s.replace("voir ", "");
            let id_cible = reste.trim().parse::<u32>().map_err(|_| "ID invalide pour 'voir'")?;
            
            if let Some(tache) = trouver_tache(list, id_cible) {
                tache.display();
            } else {
                return Err(format!("ID {} introuvable.", id_cible)); 
            }
            Ok(())
        }

        s if s.parse::<u32>().is_ok() => {
            let id_val = s.parse::<u32>().unwrap();
            for task in list {
                if task.id == id_val {
                    task.status = Status::Done;
                    println!("Tâche n°{} terminée !", id_val);
                }
            }
            Ok(())
        }

        s if !s.is_empty() => {
            let titre_valide = valider_titre(s)?; 
            
            list.push(Task::new(*id, titre_valide));
            println!("Tâche ajoutée !");
            *id += 1;
            Ok(())
        }

        _ => Err(String::from("Le titre ne peut pas être vide.")),
    }
}

fn main() {
    let mut list: Vec<Task> = Vec::new();
    let mut id = 1;

    println!("BIENVENUE DANS TASK-CLI");

    loop {
        println!("\nEntrez une nouvelle tâche, son ID pour la modifier, 'suppr ID' pour la supprimer ou 'liste' pour voir la liste des tâches 
        (ou tapez 'quitter' pour partir) :");

        
        let mut saisie = String::new();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur de lecture");

        let titre = saisie.trim();

        
        if titre == "quitter"{
            break;
        }
        if let Err(message_erreur) = executer_commande(titre, &mut list, &mut id) {
            println!("ERREUR : {}", message_erreur);
    }

    
    
}
}

            //21.2
use std::io;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

fn sauvegarder(liste: &[Task]) -> io::Result<()> {
    let mut fichier = File::create("tasks.txt")?; 
    for t in liste {
        let statut_str = match t.status {
            Status::Pending => "P",
            Status::Done => "D",
        };
        writeln!(fichier, "{}|{}|{}", t.id, t.title, statut_str)?;
    }
    Ok(())
}

fn charger() -> io::Result<Vec<Task>> {
    let mut liste = Vec::new();
    let fichier = match File::open("tasks.txt") {
        Ok(f) => f,
        Err(_) => return Ok(liste), 
    };

    let lecteur = BufReader::new(fichier);
    for ligne in lecteur.lines() {
        let l = ligne?;
        let parts: Vec<&str> = l.split('|').collect();
        if parts.len() == 3 {
            let id = parts[0].parse::<u32>().unwrap_or(0);
            let title = parts[1].to_string();
            let status = if parts[2] == "D" { Status::Done } else { Status::Pending };
            liste.push(Task { id, title, status });
        }
    }
    Ok(liste)
}

enum Status {
    Pending,
    Done,
}
struct Task {
    id: u32,
    title: String,
    status: Status,
}

impl Task {
    fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            status: Status::Pending,
        }
    }
    

    
    fn display(&self) {
        let check = match self.status {
            Status::Pending => "En Cours",
            Status::Done => "Complété",
        };
        println!("{} - {} - {}",self.id, self.title, check);
    }
}

fn valider_titre(titre: &str) -> Result<String, String> {
    if titre.len() < 3 {
        Err(String::from("Le titre doit faire au moins 3 caractères."))
    } else {
        Ok(titre.to_string())
    }
}

fn trouver_tache<'a>(liste: &'a Vec<Task>, id_recherche: u32) -> Option<&'a Task> {
    for task in liste {
        if task.id == id_recherche {
            return Some(task); 
        }
    }
    None
}

fn chercher_tache<'a>(liste: &'a [Task], mot_cle: &str) -> Option<&'a Task> {
    for task in liste {
        if task.title == mot_cle {
            return Some(task); 
        }
    }
    None 
}

fn executer_commande(titre: &str, list: &mut Vec<Task>, id: &mut u32) -> Result<(), String> {
    match titre {
        "quitter" => {
            Ok(())
        }

        "liste" => {
            println!("\nListe de Tâches : ");
            for task in list {
                match task {
                    Task { id, title, status: Status::Done } => println!("[TERMINEE] №{} : {}", id, title),
                    Task { id, title, status: Status::Pending } => println!("[EN COURS] №{} : {}", id, title),
                }
            }
            Ok(())
        }

        s if s.starts_with("chercher ") => {
            let reste = s.replace("chercher ", "");
            match chercher_tache(list, reste.trim()) {
                Some(tache) => {
                    println!("Trouvé ! Voici le détail :");
                    tache.display();
                }
                None => println!("Désolé, aucune tâche ne s'appelle '{}'.", reste.trim()),
            }
            Ok(())
        }

        s if s.starts_with("suppr ") => {
            let reste = s.replace("suppr ", "");
            let id_a_supprimer = reste.trim().parse::<u32>().map_err(|_| "L'ID doit être un nombre !")?;
            
            list.retain(|t| t.id != id_a_supprimer);
            println!("Tâche n°{} supprimée !", id_a_supprimer);
            Ok(())
        }

        s if s.starts_with("voir ") => {
            let reste = s.replace("voir ", "");
            let id_cible = reste.trim().parse::<u32>().map_err(|_| "ID invalide pour 'voir'")?;
            
            if let Some(tache) = trouver_tache(list, id_cible) {
                tache.display();
            } else {
                return Err(format!("ID {} introuvable.", id_cible)); 
            }
            Ok(())
        }

        s if s.parse::<u32>().is_ok() => {
            let id_val = s.parse::<u32>().unwrap();
            for task in list {
                if task.id == id_val {
                    task.status = Status::Done;
                    println!("Tâche n°{} terminée !", id_val);
                }
            }
            Ok(())
        }

        s if !s.is_empty() => {
            let titre_valide = valider_titre(s)?; 
            
            list.push(Task::new(*id, titre_valide));
            println!("Tâche ajoutée !");
            *id += 1;
            Ok(())
        }

        _ => Err(String::from("Le titre ne peut pas être vide.")),
    }
}

fn main() {
    let mut list = charger().unwrap_or_else(|_| Vec::new());
    let mut id = list.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    

    println!("BIENVENUE DANS TASK-CLI");

    loop {
        println!("\nEntrez une nouvelle tâche, son ID pour la modifier, 'suppr ID' pour la supprimer ou 'liste' pour voir la liste des tâches 
        (ou tapez 'quitter' pour partir) :");

        
        let mut saisie = String::new();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur de le   cture");

        let titre = saisie.trim();

        
        if titre == "quitter"{
            break;
        }
        if let Err(message_erreur) = executer_commande(titre, &mut list, &mut id) {
            println!("ERREUR : {}", message_erreur);
    }
    
    }
    sauvegarder(&list).expect("Erreur de sauvegarde");
}

            //22.1

use std::io;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::collections::HashMap;

fn sauvegarder(liste: &HashMap<u32, Task>) -> io::Result<()> {
    let mut fichier = File::create("tasks.txt")?; 
    for t in liste.values() {
        let statut_str = match t.status {
            Status::Pending => "P",
            Status::Done => "D",
        };
        writeln!(fichier, "{}|{}|{}", t.id, t.title, statut_str)?;
    }
    Ok(())
}

fn charger() -> io::Result<HashMap<u32, Task>> {
    let mut liste = HashMap::new();
    let fichier = match File::open("tasks.txt") {
        Ok(f) => f,
        Err(_) => return Ok(liste), 
    };

    let lecteur = BufReader::new(fichier);
    for ligne in lecteur.lines() {
        let l = ligne?;
        let parts: Vec<&str> = l.split('|').collect();
        if parts.len() == 3 {
            let id = parts[0].parse::<u32>().unwrap_or(0);
            let title = parts[1].to_string();
            let status = if parts[2] == "D" { Status::Done } else { Status::Pending };
            liste.insert(id, Task { id, title, status });
        }
    }
    Ok(liste)
}

enum Status {
    Pending,
    Done,
}
struct Task {
    id: u32,
    title: String,
    status: Status,
}

impl Task {
    fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            status: Status::Pending,
        }
    }
    
    fn display(&self) {
        let check = match self.status {
            Status::Pending => "En Cours",
            Status::Done => "Complété",
        };
        println!("{} - {} - {}",self.id, self.title, check);
    }
}

fn valider_titre(titre: &str) -> Result<String, String> {
    if titre.len() < 3 {
        Err(String::from("Le titre doit faire au moins 3 caractères."))
    } else {
        Ok(titre.to_string())
    }
}


fn executer_commande(titre: &str, list: &mut HashMap<u32, Task>, id: &mut u32) -> Result<(), String> {
    match titre {

        "liste" => {
            println!("\nListe de Tâches : ");
            for task in list.values() {
                match task {
                    Task { id, title, status: Status::Done } => println!("[TERMINEE] №{} : {}", id, title),
                    Task { id, title, status: Status::Pending } => println!("[EN COURS] №{} : {}", id, title),
                }
            }
            Ok(())
        }


        s if s.starts_with("suppr ") => {
            let reste = s.replace("suppr ", "");
            let id_a_suppr = reste.trim().parse::<u32>().map_err(|_| "ID invalide")?;
            if list.remove(&id_a_suppr).is_some() {
                println!("Tâche n°{} supprimée !", id_a_suppr);
            } else {
                println!("ID introuvable.");
            }
            Ok(())
        }

        s if s.starts_with("voir ") => {
            let reste = s.replace("voir ", "");
            let id_cible = reste.trim().parse::<u32>().map_err(|_| "ID invalide")?;
            if let Some(tache) = list.get(&id_cible) {
                tache.display();
            } else {
                return Err(format!("ID {} introuvable.", id_cible)); 
            }
            Ok(())
        }

        s if s.parse::<u32>().is_ok() => {
            let id_val = s.parse::<u32>().unwrap();
            if let Some(task) = list.get_mut(&id_val) {
                task.status = Status::Done;
                println!("Tâche n°{} terminée !", id_val);
            } else {
                println!("ID {} inconnu.", id_val);
            }
            Ok(())
        }

        s if !s.is_empty() => {
            let titre_valide = valider_titre(s)?; 
            if list.is_empty() {
                    *id = 1;
                }
            let nouvelle = Task::new(*id, titre_valide);
            list.insert(*id, nouvelle);
            println!("Tâche ajoutée avec l'ID {} !", id);
            *id += 1;
            Ok(())
        }

        _ => Err(String::from("Le titre ne peut pas être vide.")),
    }
}

fn main() {
    let mut list: HashMap<u32, Task> = charger().unwrap_or_else(|_| HashMap::new());
    let mut id = list.keys().max().unwrap_or(&0) + 1;
    

    println!("BIENVENUE DANS TASK-CLI");

    loop {
        println!("\nEntrez une nouvelle tâche, son ID pour la modifier, 'suppr ID' pour la supprimer ou 'liste' pour voir la liste des tâches 
        (ou tapez 'quitter' pour partir) :");

        
        let mut saisie = String::new();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur de lecture");

        let titre = saisie.trim();

        
        if titre == "quitter"{
            break;
        }
        if let Err(message_erreur) = executer_commande(titre, &mut list, &mut id) {
            println!("ERREUR : {}", message_erreur);
    }
    
    }
    sauvegarder(&list).expect("Erreur de sauvegarde");
}
*/

        //23.1

mod models;
mod logic;

use std::io;
use models::{Task, Status};
use std::collections::HashMap;


fn executer_commande(titre: &str, list: &mut HashMap<u32, Task>, id: &mut u32) -> Result<(), String> {
    match titre {

        "liste" => {
            println!("\nListe de Tâches : ");
            for task in list.values() {
                match task {
                    Task { id, title, status: Status::Done } => println!("[TERMINEE] №{} : {}", id, title),
                    Task { id, title, status: Status::Pending } => println!("[EN COURS] №{} : {}", id, title),
                }
            }
            Ok(())
        }


        s if s.starts_with("suppr ") => {
            let reste = s.replace("suppr ", "");
            let id_a_suppr = reste.trim().parse::<u32>().map_err(|_| "ID invalide")?;
            if list.remove(&id_a_suppr).is_some() {
                println!("Tâche n°{} supprimée !", id_a_suppr);
            } else {
                println!("ID introuvable.");
            }
            Ok(())
        }

        s if s.starts_with("voir ") => {
            let reste = s.replace("voir ", "");
            let id_cible = reste.trim().parse::<u32>().map_err(|_| "ID invalide")?;
            if let Some(tache) = list.get(&id_cible) {
                tache.display();
            } else {
                return Err(format!("ID {} introuvable.", id_cible)); 
            }
            Ok(())
        }

        s if s.parse::<u32>().is_ok() => {
            let id_val = s.parse::<u32>().unwrap();
            if let Some(task) = list.get_mut(&id_val) {
                task.status = Status::Done;
                println!("Tâche n°{} terminée !", id_val);
            } else {
                println!("ID {} inconnu.", id_val);
            }
            Ok(())
        }

        s if !s.is_empty() => {
            let titre_valide = logic::valider_titre(s)?; 
            if list.is_empty() {
                    *id = 1;
                }
            let nouvelle = Task::new(*id, titre_valide);
            list.insert(*id, nouvelle);
            println!("Tâche ajoutée avec l'ID {} !", id);
            *id += 1;
            Ok(())
        }

        _ => Err(String::from("Le titre ne peut pas être vide.")),
    }
}

fn main() {
    let mut list: HashMap<u32, Task> = logic::charger().unwrap_or_else(|_| HashMap::new());
    let mut id = list.keys().max().unwrap_or(&0) + 1;
    

    println!("BIENVENUE DANS TASK-CLI");

    loop {
        println!("\nEntrez une nouvelle tâche, son ID pour la modifier, 'suppr ID' pour la supprimer ou 'liste' pour voir la liste des tâches 
        (ou tapez 'quitter' pour partir) :");

        
        let mut saisie = String::new();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur de lecture");

        let titre = saisie.trim();

        
        if titre == "quitter"{
            break;
        }
        if let Err(message_erreur) = executer_commande(titre, &mut list, &mut id) {
            println!("ERREUR : {}", message_erreur);
    }
    
    }
    logic::sauvegarder(&list).expect("Erreur de sauvegarde");
}