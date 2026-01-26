//! Module d'intégration automatique pour les nouvelles fenêtres UI.
//!
//! Ce module gère :
//! - L'ajout automatique des déclarations de modules dans `mod.rs`
//! - La création automatique de la hiérarchie de `mod.rs` si nécessaire
//! - L'enregistrement des sous-répertoires dans les `mod.rs` parents

use crate::commands::add::errors::IntegrationError;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Ajoute automatiquement `pub mod <window_name>;` dans le mod.rs approprié
///
/// # Arguments
///
/// * `project_root` - Racine du projet Dampen
/// * `target_dir` - Répertoire cible où les fichiers sont créés
/// * `window_name` - Nom de la fenêtre en snake_case (ex: "settings")
///
/// # Comportement
///
/// 1. Détermine le fichier `mod.rs` approprié (dans target_dir)
/// 2. Crée le fichier `mod.rs` s'il n'existe pas
/// 3. Ajoute `pub mod <window_name>;` si pas déjà présent
/// 4. Crée et met à jour récursivement les `mod.rs` parents si nécessaire
///
/// # Returns
///
/// `Ok(PathBuf)` - Chemin du fichier `mod.rs` mis à jour
/// `Err(IntegrationError)` - Si une erreur I/O survient
///
/// # Examples
///
/// ```no_run
/// use dampen_cli::commands::add::integration::add_module_to_mod_rs;
/// use std::path::Path;
///
/// // Ajouter un module dans src/ui/
/// let mod_path = add_module_to_mod_rs(
///     Path::new("/project"),
///     Path::new("/project/src/ui"),
///     "settings"
/// )?;
/// // → Met à jour /project/src/ui/mod.rs
///
/// // Ajouter un module dans un sous-répertoire
/// let mod_path = add_module_to_mod_rs(
///     Path::new("/project"),
///     Path::new("/project/src/ui/admin"),
///     "dashboard"
/// )?;
/// // → Met à jour /project/src/ui/admin/mod.rs
/// // → Crée et met à jour /project/src/ui/mod.rs si nécessaire
/// # Ok::<(), dampen_cli::commands::add::errors::IntegrationError>(())
/// ```
pub fn add_module_to_mod_rs(
    project_root: &Path,
    target_dir: &Path,
    window_name: &str,
) -> Result<PathBuf, IntegrationError> {
    let mod_path = target_dir.join("mod.rs");

    // 1. Créer le répertoire cible s'il n'existe pas
    if !target_dir.exists() {
        fs::create_dir_all(target_dir).map_err(|source| IntegrationError::DirectoryCreation {
            path: target_dir.to_path_buf(),
            source,
        })?;
    }

    // 2. Lire ou créer le contenu de mod.rs
    let content = if mod_path.exists() {
        fs::read_to_string(&mod_path).map_err(|source| IntegrationError::ModFileRead {
            path: mod_path.clone(),
            source,
        })?
    } else {
        // Créer un nouveau mod.rs avec en-tête
        String::from("// UI module exports\n\n")
    };

    // 3. Vérifier si le module est déjà déclaré
    if is_module_declared(&content, window_name) {
        return Ok(mod_path); // Déjà présent, rien à faire
    }

    // 4. Ajouter la déclaration du module
    let new_content = append_module_declaration(&content, window_name);

    // 5. Écrire le fichier mis à jour
    fs::write(&mod_path, new_content).map_err(|source| IntegrationError::ModFileWrite {
        path: mod_path.clone(),
        source,
    })?;

    // 6. Mettre à jour récursivement les mod.rs parents si nécessaire
    ensure_parent_modules_registered(project_root, target_dir)?;

    Ok(mod_path)
}

/// Assure que tous les mod.rs parents enregistrent leurs sous-répertoires
///
/// # Exemple
///
/// Si on crée `src/ui/admin/dashboard.rs`, cette fonction :
/// 1. Crée `src/ui/admin/mod.rs` (avec `pub mod dashboard;`)
/// 2. Crée/met à jour `src/ui/mod.rs` (avec `pub mod admin;`)
///
/// # Arguments
///
/// * `project_root` - Racine du projet
/// * `target_dir` - Répertoire où le module a été créé
fn ensure_parent_modules_registered(
    project_root: &Path,
    target_dir: &Path,
) -> Result<(), IntegrationError> {
    // Remonter jusqu'à src/ui (ou src/ selon la structure)
    let mut current_dir = target_dir.to_path_buf();
    let ui_root = project_root.join("src/ui");

    // Arrêter à src/ui (pas besoin d'aller au-delà)
    if current_dir == ui_root {
        return Ok(());
    }

    // Remonter et enregistrer chaque niveau
    while let Some(parent_dir) = current_dir.parent() {
        // Arrêter si on remonte au-dessus de src/ui
        if parent_dir < ui_root.as_path() {
            break;
        }

        let parent_mod_path = parent_dir.join("mod.rs");
        let subdir_name = current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| IntegrationError::DirectoryCreation {
                path: current_dir.clone(),
                source: io::Error::new(io::ErrorKind::InvalidInput, "Invalid directory name"),
            })?;

        // Lire ou créer le mod.rs parent
        let content = if parent_mod_path.exists() {
            fs::read_to_string(&parent_mod_path).map_err(|source| {
                IntegrationError::ModFileRead {
                    path: parent_mod_path.clone(),
                    source,
                }
            })?
        } else {
            String::from("// UI module exports\n\n")
        };

        // Ajouter la déclaration du sous-module si nécessaire
        if !is_module_declared(&content, subdir_name) {
            let new_content = append_module_declaration(&content, subdir_name);
            fs::write(&parent_mod_path, new_content).map_err(|source| {
                IntegrationError::ModFileWrite {
                    path: parent_mod_path.clone(),
                    source,
                }
            })?;
        }

        // Arrêter après avoir traité src/ui
        if parent_dir == ui_root {
            break;
        }

        current_dir = parent_dir.to_path_buf();
    }

    Ok(())
}

/// Vérifie si un module est déjà déclaré dans le contenu
///
/// Cherche le pattern : `pub mod <module_name>;`
///
/// # Arguments
///
/// * `content` - Contenu du fichier mod.rs
/// * `module_name` - Nom du module à chercher
///
/// # Returns
///
/// `true` si le module est déjà déclaré, `false` sinon
fn is_module_declared(content: &str, module_name: &str) -> bool {
    // Pattern : cherche `pub mod <module_name>;` sur une ligne
    // (?m) = mode multiline
    // ^\s* = début de ligne + espaces optionnels
    // pub\s+mod\s+ = "pub mod" avec espaces flexibles
    // <module_name>\s*; = nom du module + espaces optionnels + point-virgule
    let pattern = format!(r"(?m)^\s*pub\s+mod\s+{}\s*;", regex::escape(module_name));

    // The pattern is constructed with regex::escape, so it should always be valid
    // If it fails, we conservatively return false (module not declared)
    match regex::Regex::new(&pattern) {
        Ok(re) => re.is_match(content),
        Err(_) => false,
    }
}

/// Ajoute une déclaration de module à la fin du fichier
///
/// # Arguments
///
/// * `content` - Contenu actuel du fichier
/// * `module_name` - Nom du module à ajouter
///
/// # Returns
///
/// Nouveau contenu avec la déclaration ajoutée
///
/// # Format
///
/// La déclaration est ajoutée au format : `pub mod <module_name>;\n`
fn append_module_declaration(content: &str, module_name: &str) -> String {
    let mut new_content = content.to_string();

    // Assurer qu'on termine par une nouvelle ligne si le fichier n'est pas vide
    if !new_content.is_empty() && !new_content.ends_with('\n') {
        new_content.push('\n');
    }

    // Ajouter la déclaration du module
    new_content.push_str(&format!("pub mod {};\n", module_name));

    new_content
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // === Tests de is_module_declared ===

    #[test]
    fn test_is_module_declared_simple() {
        let content = "pub mod window;\n";
        assert!(is_module_declared(content, "window"));
        assert!(!is_module_declared(content, "settings"));
    }

    #[test]
    fn test_is_module_declared_multiple_modules() {
        let content = "pub mod window;\npub mod settings;\npub mod profile;\n";
        assert!(is_module_declared(content, "window"));
        assert!(is_module_declared(content, "settings"));
        assert!(is_module_declared(content, "profile"));
        assert!(!is_module_declared(content, "admin"));
    }

    #[test]
    fn test_is_module_declared_with_whitespace() {
        let content = "pub mod window ;\n";
        assert!(is_module_declared(content, "window"));
    }

    #[test]
    fn test_is_module_declared_with_indentation() {
        let content = "    pub mod window;\n";
        assert!(is_module_declared(content, "window"));
    }

    #[test]
    fn test_is_module_declared_ignores_comments() {
        let content = "// pub mod commented;\npub mod window;\n";
        assert!(is_module_declared(content, "window"));
        assert!(!is_module_declared(content, "commented"));
    }

    #[test]
    fn test_is_module_declared_with_similar_names() {
        let content = "pub mod settings;\npub mod settings_page;\n";
        assert!(is_module_declared(content, "settings"));
        assert!(is_module_declared(content, "settings_page"));
        assert!(!is_module_declared(content, "setting"));
    }

    // === Tests de append_module_declaration ===

    #[test]
    fn test_append_module_declaration_to_empty() {
        let content = "";
        let result = append_module_declaration(content, "settings");
        assert_eq!(result, "pub mod settings;\n");
    }

    #[test]
    fn test_append_module_declaration_to_existing() {
        let content = "pub mod window;\n";
        let result = append_module_declaration(content, "settings");
        assert_eq!(result, "pub mod window;\npub mod settings;\n");
    }

    #[test]
    fn test_append_module_declaration_without_trailing_newline() {
        let content = "pub mod window;";
        let result = append_module_declaration(content, "settings");
        assert_eq!(result, "pub mod window;\npub mod settings;\n");
    }

    // === Tests d'intégration de add_module_to_mod_rs ===

    #[test]
    fn test_add_module_creates_new_mod_file() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let ui_dir = project_root.join("src/ui");
        fs::create_dir_all(&ui_dir).unwrap();

        let result = add_module_to_mod_rs(project_root, &ui_dir, "settings");

        assert!(result.is_ok());
        let mod_path = ui_dir.join("mod.rs");
        assert!(mod_path.exists());

        let content = fs::read_to_string(mod_path).unwrap();
        assert!(content.contains("pub mod settings;"));
    }

    #[test]
    fn test_add_module_appends_to_existing() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let ui_dir = project_root.join("src/ui");
        fs::create_dir_all(&ui_dir).unwrap();

        let mod_path = ui_dir.join("mod.rs");
        fs::write(&mod_path, "pub mod window;\n").unwrap();

        let result = add_module_to_mod_rs(project_root, &ui_dir, "settings");

        assert!(result.is_ok());
        let content = fs::read_to_string(mod_path).unwrap();
        assert!(content.contains("pub mod window;"));
        assert!(content.contains("pub mod settings;"));
    }

    #[test]
    fn test_add_module_prevents_duplicates() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let ui_dir = project_root.join("src/ui");
        fs::create_dir_all(&ui_dir).unwrap();

        let mod_path = ui_dir.join("mod.rs");
        fs::write(&mod_path, "pub mod settings;\n").unwrap();

        let result = add_module_to_mod_rs(project_root, &ui_dir, "settings");

        assert!(result.is_ok());
        let content = fs::read_to_string(mod_path).unwrap();

        let count = content.matches("pub mod settings;").count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_add_module_creates_directory_if_missing() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let ui_dir = project_root.join("src/ui/admin");
        // Ne pas créer le répertoire

        let result = add_module_to_mod_rs(project_root, &ui_dir, "dashboard");

        assert!(result.is_ok());
        assert!(ui_dir.exists());
        assert!(ui_dir.join("mod.rs").exists());

        let content = fs::read_to_string(ui_dir.join("mod.rs")).unwrap();
        assert!(content.contains("pub mod dashboard;"));
    }

    // === Tests de ensure_parent_modules_registered ===

    #[test]
    fn test_ensure_parent_modules_single_level() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let ui_dir = project_root.join("src/ui");
        fs::create_dir_all(&ui_dir).unwrap();

        let target_dir = ui_dir.join("admin");
        fs::create_dir_all(&target_dir).unwrap();
        fs::write(target_dir.join("mod.rs"), "pub mod dashboard;\n").unwrap();

        let result = ensure_parent_modules_registered(project_root, &target_dir);

        assert!(result.is_ok());

        // Vérifier que src/ui/mod.rs contient `pub mod admin;`
        let ui_mod_path = ui_dir.join("mod.rs");
        assert!(ui_mod_path.exists());
        let content = fs::read_to_string(ui_mod_path).unwrap();
        assert!(content.contains("pub mod admin;"));
    }

    #[test]
    fn test_ensure_parent_modules_nested_levels() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let ui_dir = project_root.join("src/ui");
        fs::create_dir_all(&ui_dir).unwrap();

        let target_dir = ui_dir.join("admin/reports");
        fs::create_dir_all(&target_dir).unwrap();
        fs::write(target_dir.join("mod.rs"), "pub mod sales;\n").unwrap();

        let result = ensure_parent_modules_registered(project_root, &target_dir);

        assert!(result.is_ok());

        // Vérifier src/ui/admin/mod.rs
        let admin_mod_path = ui_dir.join("admin/mod.rs");
        assert!(admin_mod_path.exists());
        let content = fs::read_to_string(&admin_mod_path).unwrap();
        assert!(content.contains("pub mod reports;"));

        // Vérifier src/ui/mod.rs
        let ui_mod_path = ui_dir.join("mod.rs");
        assert!(ui_mod_path.exists());
        let content = fs::read_to_string(ui_mod_path).unwrap();
        assert!(content.contains("pub mod admin;"));
    }

    #[test]
    fn test_ensure_parent_modules_preserves_existing() {
        let temp = TempDir::new().unwrap();
        let project_root = temp.path();
        let ui_dir = project_root.join("src/ui");
        fs::create_dir_all(&ui_dir).unwrap();
        fs::write(ui_dir.join("mod.rs"), "pub mod window;\n").unwrap();

        let target_dir = ui_dir.join("admin");
        fs::create_dir_all(&target_dir).unwrap();

        let result = ensure_parent_modules_registered(project_root, &target_dir);

        assert!(result.is_ok());

        let ui_mod_path = ui_dir.join("mod.rs");
        let content = fs::read_to_string(ui_mod_path).unwrap();
        assert!(content.contains("pub mod window;"));
        assert!(content.contains("pub mod admin;"));
    }
}
