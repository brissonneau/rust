fn main() {
    let ma_nouvelle_tache = create_task("Maîtriser le Rust");
    
    println!("[Notification] {}", ma_nouvelle_tache);
}

fn create_task(titre: &str) -> String {
    println!("Création d'une nouvelle tâche en cours...");
    
    format!("Nouvelle tâche : {}", titre)
}