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