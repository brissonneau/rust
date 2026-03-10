use crate::models::{Task, Status};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write, BufReader, BufRead};

pub fn sauvegarder(map: &HashMap<u32, Task>) -> io::Result<()> {
    let mut fichier = File::create("tasks.txt")?;
    for t in map.values() {
        let statut_str = match t.status {
            Status::Pending => "P",
            Status::Done => "D",
        };
        writeln!(fichier, "{}|{}|{}", t.id, t.title, statut_str)?;
    }
    Ok(())
}

pub fn charger() -> io::Result<HashMap<u32, Task>> {
    let mut map = HashMap::new();
    let fichier = match File::open("tasks.txt") {
        Ok(f) => f,
        Err(_) => return Ok(map),
    };

    let lecteur = BufReader::new(fichier);
    for ligne in lecteur.lines() {
        let l = ligne?;
        let parts: Vec<&str> = l.split('|').collect();
        if parts.len() == 3 {
            let id = parts[0].parse::<u32>().unwrap_or(0);
            let title = parts[1].to_string();
            let status = if parts[2] == "D" { Status::Done } else { Status::Pending };
            map.insert(id, Task { id, title, status });
        }
    }
    Ok(map)
}

pub fn valider_titre(titre: &str) -> Result<String, String> {
    if titre.len() < 3 {
        Err(String::from("Le titre doit faire au moins 3 caractères."))
    } else {
        Ok(titre.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_titre_trop_court() {
        let resultat = valider_titre("ab");
        assert!(resultat.is_err()); 
    }

    #[test]
    fn test_validation_titre_valide() {
        let resultat = valider_titre("Apprendre Rust");
        assert!(resultat.is_ok());
        assert_eq!(resultat.unwrap(), "Apprendre Rust");
    }
}