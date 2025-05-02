use std::io::{self, Write};
use std::fs::{self, File};
use rand::{Rng, thread_rng};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

// Structure pour TP1
struct Produit {
    nom: String,
    quantite: u32,
}

// Fonction principale qui gère le menu principal
fn main() {
    loop {
        println!("\n--- MENU PRINCIPAL ---");
        println!("1. TP1 - Gestion d'inventaire");
        println!("2. TP2 - Générateur de mot de passe");
        println!("3. TP3 - Jeu de devinette");
        println!("4. TP4 - Envoi d'e-mail");
        println!("5. Quitter");
        print!("Choisissez une option (1-5): ");
        io::stdout().flush().unwrap();

        let choix = lire_entree();

        match choix.trim() {
            "1" => tp1_gestion_inventaire(),
            "2" => tp2_generateur_mot_de_passe(),
            "3" => tp3_jeu_devinette(),
            "4" => tp4_envoi_email(),
            "5" => {
                println!("Au revoir!");
                break;
            }
            _ => println!("Option invalide, veuillez réessayer."),
        }
    }
}

// Fonction pour lire l'entrée utilisateur
fn lire_entree() -> String {
    let mut entree = String::new();
    io::stdin().read_line(&mut entree).expect("Erreur de lecture");
    entree
}

// TP1 - Gestion d'inventaire
fn tp1_gestion_inventaire() {
    let mut inventaire: Vec<Produit> = Vec::new();
    
    // charge l'inventaire depuis un fichier
    if let Ok(contenu) = fs::read_to_string("inventaire.txt") {
        for ligne in contenu.lines() {
            let parties: Vec<&str> = ligne.split(',').collect();
            if parties.len() == 2 {
                if let Ok(quantite) = parties[1].parse::<u32>() {
                    inventaire.push(Produit {
                        nom: parties[0].to_string(),
                        quantite,
                    });
                }
            }
        }
        println!("Inventaire chargé depuis le fichier.");
    }

    loop {
        println!("\n--- GESTION D'INVENTAIRE ---");
        println!("1. Ajouter un produit");
        println!("2. Modifier un produit");
        println!("3. Supprimer un produit");
        println!("4. Afficher l'inventaire");
        println!("5. Sauvegarder et retourner au menu principal");
        print!("Choisissez une option (1-5): ");
        io::stdout().flush().unwrap();

        let choix = lire_entree();

        match choix.trim() {
            "1" => {
                print!("Nom du produit: ");
                io::stdout().flush().unwrap();
                let nom = lire_entree().trim().to_string();
                
                print!("Quantité: ");
                io::stdout().flush().unwrap();
                let quantite_str = lire_entree();
                
                if let Ok(quantite) = quantite_str.trim().parse::<u32>() {
                    inventaire.push(Produit { nom, quantite });
                    println!("Produit ajouté!");
                } else {
                    println!("Quantité invalide!");
                }
            }
            "2" => {
                if inventaire.is_empty() {
                    println!("L'inventaire est vide!");
                    continue;
                }
                
                println!("Produits disponibles:");
                for (i, produit) in inventaire.iter().enumerate() {
                    println!("{}. {} - {} unités", i+1, produit.nom, produit.quantite);
                }
                
                print!("Entrez le numéro du produit à modifier: ");
                io::stdout().flush().unwrap();
                let index_str = lire_entree();
                
                if let Ok(mut index) = index_str.trim().parse::<usize>() {
                    index -= 1; 
                    
                    if index < inventaire.len() {
                        print!("Nouvelle quantité (laissez vide pour ne pas changer): ");
                        io::stdout().flush().unwrap();
                        let nouvelle_quantite = lire_entree();
                        
                        if !nouvelle_quantite.trim().is_empty() {
                            if let Ok(quantite) = nouvelle_quantite.trim().parse::<u32>() {
                                inventaire[index].quantite = quantite;
                                println!("Quantité mise à jour!");
                            } else {
                                println!("Quantité invalide!");
                            }
                        }
                        
                        print!("Nouveau nom (laissez vide pour ne pas changer): ");
                        io::stdout().flush().unwrap();
                        let nouveau_nom = lire_entree();
                        
                        if !nouveau_nom.trim().is_empty() {
                            inventaire[index].nom = nouveau_nom.trim().to_string();
                            println!("Nom mis à jour!");
                        }
                    } else {
                        println!("Index invalide!");
                    }
                } else {
                    println!("Entrée invalide!");
                }
            }
            "3" => {
                if inventaire.is_empty() {
                    println!("L'inventaire est vide!");
                    continue;
                }
                
                println!("Produits disponibles:");
                for (i, produit) in inventaire.iter().enumerate() {
                    println!("{}. {} - {} unités", i+1, produit.nom, produit.quantite);
                }
                
                print!("Entrez le numéro du produit à supprimer: ");
                io::stdout().flush().unwrap();
                let index_str = lire_entree();
                
                if let Ok(mut index) = index_str.trim().parse::<usize>() {
                    index -= 1;
                    
                    if index < inventaire.len() {
                        let produit = inventaire.remove(index);
                        println!("Produit '{}' supprimé!", produit.nom);
                    } else {
                        println!("Index invalide!");
                    }
                } else {
                    println!("Entrée invalide!");
                }
            }
            "4" => {
                if inventaire.is_empty() {
                    println!("L'inventaire est vide!");
                } else {
                    println!("Inventaire actuel:");
                    for (i, produit) in inventaire.iter().enumerate() {
                        println!("{}. {} - {} unités", i+1, produit.nom, produit.quantite);
                    }
                }
            }
            "5" => {
                // Sauvegarde l'inventaire dans un fichier
                let mut fichier = File::create("inventaire.txt").expect("Erreur lors de la création du fichier");
                for produit in &inventaire {
                    writeln!(fichier, "{},{}", produit.nom, produit.quantite)
                        .expect("Erreur lors de l'écriture dans le fichier");
                }
                println!("Inventaire sauvegardé dans 'inventaire.txt'");
                break;
            }
            _ => println!("Option invalide, veuillez réessayer."),
        }
    }
}

// TP2 - Générateur de mot de passe
fn tp2_generateur_mot_de_passe() {
    println!("\n--- GÉNÉRATEUR DE MOT DE PASSE ---");
    
    print!("Longueur du mot de passe: ");
    io::stdout().flush().unwrap();
    let longueur_str = lire_entree();
    
    let longueur = match longueur_str.trim().parse::<usize>() {
        Ok(l) if l > 0 => l,
        _ => {
            println!("Longueur invalide! Utilisation de la valeur par défaut: 12");
            12
        }
    };
    
    print!("Exclure certains caractères? (o/n): ");
    io::stdout().flush().unwrap();
    let exclure = lire_entree().trim().to_lowercase() == "o";
    
    let mut caracteres_exclus = String::new();
    if exclure {
        print!("Entrez les caractères à exclure (sans espaces): ");
        io::stdout().flush().unwrap();
        caracteres_exclus = lire_entree().trim().to_string();
    }
    
    // Caractères possibles pour le mot de passe
    let lettres = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let chiffres = "0123456789";
    let speciaux = "!@#$%^&*()-_=+[]{}|;:,.<>?";
    
    let mut tous_caracteres = String::new();
    tous_caracteres.push_str(lettres);
    tous_caracteres.push_str(chiffres);
    tous_caracteres.push_str(speciaux);
    
    // Filtre des caractères exclus
    let caracteres_disponibles: String = tous_caracteres
        .chars()
        .filter(|c| !caracteres_exclus.contains(*c))
        .collect();
    
    if caracteres_disponibles.is_empty() {
        println!("Tous les caractères ont été exclus! Impossible de générer un mot de passe.");
        return;
    }
    
    // Génére le mot de passe
    let mut rng = thread_rng();
    let mdp: String = (0..longueur)
        .map(|_| {
            let idx = rng.gen_range(0..caracteres_disponibles.len());
            caracteres_disponibles.chars().nth(idx).unwrap()
        })
        .collect();
    
    println!("Mot de passe généré: {}", mdp);
    println!("Appuyez sur Entrée pour continuer...");
    lire_entree();
}

// TP3 - Jeu de devinette
fn tp3_jeu_devinette() {
    println!("\n--- JEU DE DEVINETTE ---");
    println!("Je pense à un nombre entre 1 et 100");
    
    let nombre_secret = thread_rng().gen_range(1..=100);
    let mut essais = 0;
    
    loop {
        print!("Votre guess: ");
        io::stdout().flush().unwrap();
        let guess_str = lire_entree();
        
        match guess_str.trim().parse::<u32>() {
            Ok(guess) => {
                essais += 1;
                
                if guess < nombre_secret {
                    println!("Trop petit!");
                } else if guess > nombre_secret {
                    println!("Trop grand!");
                } else {
                    println!("GAGNÉ! Le nombre était {}.", nombre_secret);
                    println!("Vous avez trouvé en {} essais.", essais);
                    break;
                }
            }
            Err(_) => println!("Veuillez entrer un nombre valide!"),
        }
    }
    
    println!("Appuyez sur Entrée pour continuer...");
    lire_entree();
}

// TP4 - Envoi d'e-mail
fn tp4_envoi_email() {
    println!("\n--- ENVOI D'E-MAIL ---");
    
    print!("Votre adresse e-mail: ");
    io::stdout().flush().unwrap();
    let email_expediteur = lire_entree().trim().to_string();
    
    print!("Votre mot de passe: ");
    io::stdout().flush().unwrap();
    let mot_de_passe = lire_entree().trim().to_string();
    
    print!("Destinataire: ");
    io::stdout().flush().unwrap();
    let destinataire = lire_entree().trim().to_string();
    
    print!("Sujet: ");
    io::stdout().flush().unwrap();
    let sujet = lire_entree().trim().to_string();
    
    println!("Corps du message (terminez par une ligne contenant uniquement '.'):");
    let mut corps = String::new();
    loop {
        let ligne = lire_entree();
        if ligne.trim() == "." {
            break;
        }
        corps.push_str(&ligne);
    }
    
    print!("Serveur SMTP (ex: smtp.gmail.com): ");
    io::stdout().flush().unwrap();
    let serveur_smtp = lire_entree().trim().to_string();
    
    println!("Envoi de l'e-mail en cours...");
    
    // Création du message
    let email = match Message::builder()
        .from(email_expediteur.parse().unwrap())
        .to(destinataire.parse().unwrap())
        .subject(sujet)
        .body(corps) {
            Ok(email) => email,
            Err(err) => {
                println!("Erreur lors de la création du message: {}", err);
                return;
            }
        };
    
    // Configuration du transport SMTP
    let creds = Credentials::new(email_expediteur.clone(), mot_de_passe);
    
    let transport = SmtpTransport::relay(&serveur_smtp)
        .unwrap()
        .credentials(creds)
        .build();
    
    // Envoi de l'e-mail
    match transport.send(&email) {
        Ok(_) => println!("E-mail envoyé avec succès!"),
        Err(e) => println!("Erreur lors de l'envoi de l'e-mail: {}", e),
    }
    
    println!("Appuyez sur Entrée pour continuer...");
    lire_entree();
}