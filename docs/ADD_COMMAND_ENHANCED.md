# Plan d'Implémentation : Amélioration de `dampen add --ui`

**Date** : 2026-01-13  
**Statut** : 📋 Planifié  
**Phase** : 1 + 2 (Mise à jour automatique de `mod.rs`)  
**Auteur** : Plan collaboratif avec l'utilisateur

---

## 🎯 Objectif

Automatiser l'ajout de `pub mod <window_name>;` dans le fichier `mod.rs` approprié lors de l'exécution de `dampen add --ui <window_name>`.

### Portée de la Phase 1 + 2

- ✅ Mise à jour automatique de `mod.rs` (répertoire cible)
- ✅ Support des chemins personnalisés (`--path`)
- ✅ Création automatique de la hiérarchie de `mod.rs` si nécessaire
- ✅ Option `--no-integrate` pour désactiver l'automatisation
- ❌ **PAS** de modification de `main.rs` (sera implémenté dans Phase 3 ultérieure)

### Décisions de Design

1. **Répertoires personnalisés** : Le module est déclaré dans le `mod.rs` du répertoire où il est créé
   - Exemple : `--path "src/ui/admin"` → module ajouté dans `src/ui/admin/mod.rs`

2. **Message de succès** : Afficher le chemin exact du `mod.rs` modifié
   - Exemple : "Updated src/ui/admin/mod.rs" (pas seulement "Updated src/ui/mod.rs")

3. **Création automatique** : Créer automatiquement la hiérarchie de `mod.rs` si nécessaire
   - Crée `src/ui/admin/mod.rs` s'il n'existe pas
   - Enregistre automatiquement les sous-modules dans les `mod.rs` parents

---

## 📋 Résumé des Changements

### Fichiers à Créer

```
crates/dampen-cli/src/commands/add/
└── integration.rs          # ➕ NOUVEAU : Logique d'intégration avec mod.rs
```

### Fichiers à Modifier

```
crates/dampen-cli/src/commands/add/
├── mod.rs                  # ✏️ Ajouter `--no-integrate` flag
├── generation.rs           # ✏️ Appeler l'intégration après génération
├── validation.rs           # ✏️ Étendre TargetPath avec project_root
└── errors.rs              # ✏️ Ajouter IntegrationError

crates/dampen-cli/Cargo.toml # ✏️ Ajouter dépendance `regex`
```

---

## 🗂️ Architecture des Types

### Nouvelles Erreurs (`errors.rs`)

```rust
/// Erreurs lors de l'intégration automatique du module
#[derive(Debug, thiserror::Error)]
pub enum IntegrationError {
    /// Échec de lecture d'un fichier mod.rs
    #[error("Failed to read {path}: {source}")]
    ModFileRead {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    
    /// Échec d'écriture d'un fichier mod.rs
    #[error("Failed to write {path}: {source}")]
    ModFileWrite {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    
    /// Échec de création du répertoire
    #[error("Failed to create directory {path}: {source}")]
    DirectoryCreation {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}
```

### Mise à Jour de `TargetPath` (`validation.rs`)

```rust
pub struct TargetPath {
    /// Absolute path to target directory
    pub absolute: PathBuf,
    
    /// Relative path from project root
    pub relative: PathBuf,
    
    /// Project root directory (NOUVEAU)
    pub project_root: PathBuf,
}
```

### Mise à Jour de `GeneratedFiles` (`generation.rs`)

```rust
pub struct GeneratedFiles {
    pub rust_file: PathBuf,
    pub dampen_file: PathBuf,
    pub window_name: WindowName,
    pub target_dir: PathBuf,
    
    /// Path to mod.rs that was updated (NOUVEAU)
    pub updated_mod_file: Option<PathBuf>,
}
```

---

## 📝 Implémentation Détaillée

### Tâche 1 : Créer `integration.rs`

#### 1.1 Fonction Principale

```rust
//! Module d'intégration automatique pour les nouvelles fenêtres UI.
//!
//! Ce module gère :
//! - L'ajout automatique des déclarations de modules dans `mod.rs`
//! - La création automatique de la hiérarchie de `mod.rs` si nécessaire
//! - L'enregistrement des sous-répertoires dans les `mod.rs` parents

use std::fs;
use std::path::{Path, PathBuf};
use crate::commands::add::errors::IntegrationError;

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
/// ```
pub fn add_module_to_mod_rs(
    project_root: &Path,
    target_dir: &Path,
    window_name: &str,
) -> Result<PathBuf, IntegrationError> {
    let mod_path = target_dir.join("mod.rs");
    
    // 1. Créer le répertoire cible s'il n'existe pas
    if !target_dir.exists() {
        fs::create_dir_all(target_dir).map_err(|source| {
            IntegrationError::DirectoryCreation {
                path: target_dir.to_path_buf(),
                source,
            }
        })?;
    }
    
    // 2. Lire ou créer le contenu de mod.rs
    let content = if mod_path.exists() {
        fs::read_to_string(&mod_path).map_err(|source| {
            IntegrationError::ModFileRead {
                path: mod_path.clone(),
                source,
            }
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
    fs::write(&mod_path, new_content).map_err(|source| {
        IntegrationError::ModFileWrite {
            path: mod_path.clone(),
            source,
        }
    })?;
    
    // 6. Mettre à jour récursivement les mod.rs parents si nécessaire
    ensure_parent_modules_registered(project_root, target_dir)?;
    
    Ok(mod_path)
}
```

#### 1.2 Enregistrement Récursif des Sous-Modules

```rust
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
        // Arrêter à src/ui
        if parent_dir == ui_root || parent_dir < ui_root.as_path() {
            break;
        }
        
        let parent_mod_path = parent_dir.join("mod.rs");
        let subdir_name = current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| IntegrationError::DirectoryCreation {
                path: current_dir.clone(),
                source: io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid directory name",
                ),
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
        
        current_dir = parent_dir.to_path_buf();
    }
    
    Ok(())
}
```

#### 1.3 Fonctions Auxiliaires

```rust
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
    let pattern = format!(
        r"(?m)^\s*pub\s+mod\s+{}\s*;",
        regex::escape(module_name)
    );
    
    regex::Regex::new(&pattern)
        .unwrap()
        .is_match(content)
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
```

---

### Tâche 2 : Étendre `TargetPath` avec `project_root`

```rust
// Dans validation.rs - Modification de TargetPath

impl TargetPath {
    pub fn resolve(project_root: &Path, custom_path: Option<&str>) -> Result<Self, PathError> {
        // ... code existant ...
        
        Ok(Self {
            absolute,
            relative,
            project_root: project_root.to_path_buf(), // NOUVEAU
        })
    }
}
```

---

### Tâche 3 : Ajouter la Flag `--no-integrate`

```rust
// Dans add/mod.rs - Mise à jour de AddArgs

#[derive(Debug, Args)]
pub struct AddArgs {
    /// Add a new UI window
    #[arg(long)]
    pub ui: Option<String>,

    /// Custom output directory path (relative to project root)
    #[arg(long)]
    pub path: Option<String>,
    
    /// Disable automatic integration (do not update mod.rs)
    ///
    /// By default, the command automatically adds `pub mod <window_name>;`
    /// to the appropriate mod.rs file. Use this flag to disable automatic
    /// integration and handle module registration manually.
    ///
    /// Example:
    ///   dampen add --ui settings --no-integrate
    #[arg(long)]
    pub no_integrate: bool,
}
```

---

### Tâche 4 : Intégrer dans `generation.rs`

```rust
// Dans generation.rs - Modification de generate_window_files

use crate::commands::add::integration::add_module_to_mod_rs;

pub fn generate_window_files(
    target_path: &TargetPath,
    window_name: &WindowName,
    enable_integration: bool, // NOUVEAU paramètre
) -> Result<GeneratedFiles, GenerationError> {
    // 1-4. Code existant de génération des fichiers...
    
    // === NOUVEAU : Post-generation integration ===
    
    let updated_mod_file = if enable_integration {
        match add_module_to_mod_rs(
            &target_path.project_root,
            &target_path.absolute,
            &window_name.snake
        ) {
            Ok(mod_path) => Some(mod_path),
            Err(e) => {
                // Non-fatal : afficher un warning mais continuer
                eprintln!("⚠ Warning: Failed to update mod.rs: {}", e);
                eprintln!("  Please manually add: pub mod {};", window_name.snake);
                None
            }
        }
    } else {
        None
    };

    Ok(GeneratedFiles {
        rust_file,
        dampen_file,
        window_name: window_name.clone(),
        target_dir: target_path.absolute.clone(),
        updated_mod_file, // NOUVEAU champ
    })
}
```

---

### Tâche 5 : Mettre à Jour `GeneratedFiles::success_message()`

```rust
// Dans generation.rs - Mise à jour du message de succès

impl GeneratedFiles {
    pub fn success_message(&self) -> String {
        let mut message = format!(
            "✓ Created UI window '{}'\n  → {}\n  → {}",
            self.window_name.snake,
            self.rust_file.display(),
            self.dampen_file.display()
        );
        
        // Ajouter une ligne si l'intégration a été effectuée
        if let Some(mod_path) = &self.updated_mod_file {
            message.push_str(&format!("\n  → Updated {}", mod_path.display()));
        }
        
        // Instructions suivantes
        message.push_str("\n\nNext steps:");
        
        if self.updated_mod_file.is_none() {
            message.push_str(&format!(
                "\n  1. Add `pub mod {};` to the appropriate mod.rs file",
                self.window_name.snake
            ));
            message.push_str("\n  2. Run `dampen check` to validate");
            message.push_str("\n  3. Run your application to see the new window");
        } else {
            message.push_str("\n  1. Run `dampen check` to validate");
            message.push_str("\n  2. Run your application to see the new window");
        }
        
        message
    }
}
```

---

### Tâche 6 : Mettre à Jour `execute()`

```rust
// Dans add/mod.rs - Mise à jour de execute()

pub fn execute(args: &AddArgs) -> Result<(), String> {
    // Code existant de validation...
    
    // T076: Generate files (avec nouveau paramètre enable_integration)
    let enable_integration = !args.no_integrate;
    let generated = generate_window_files(&target_path, &window_name, enable_integration)
        .map_err(|e| e.to_string())?;

    // T077: Print success message
    println!("{}", generated.success_message());

    Ok(())
}
```

---

### Tâche 7 : Ajouter la Dépendance `regex`

```toml
# Dans crates/dampen-cli/Cargo.toml

[dependencies]
# ... dépendances existantes ...
regex = "1.10"  # Pour le parsing des déclarations de modules
```

---

## 🧪 Tests à Implémenter

### Tests Unitaires (`integration.rs`)

```rust
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
```

### Tests d'Intégration E2E

```rust
// Dans crates/dampen-cli/tests/integration_add_ui.rs

use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn setup_dampen_project() -> TempDir {
    let temp = TempDir::new().unwrap();
    // Créer un projet Dampen minimal
    // (utiliser `dampen new` ou créer manuellement)
    temp
}

fn run_dampen_add(project_root: &Path, window_name: &str, extra_args: &[&str]) {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--bin")
        .arg("dampen-cli")
        .arg("--")
        .arg("add")
        .arg("--ui")
        .arg(window_name)
        .current_dir(project_root);
    
    for arg in extra_args {
        cmd.arg(arg);
    }
    
    let output = cmd.output().unwrap();
    assert!(output.status.success());
}

#[test]
fn test_add_ui_updates_mod_rs_by_default() {
    let temp = setup_dampen_project();
    let project_root = temp.path();
    
    run_dampen_add(project_root, "settings", &[]);
    
    let mod_content = fs::read_to_string(
        project_root.join("src/ui/mod.rs")
    ).unwrap();
    assert!(mod_content.contains("pub mod settings;"));
}

#[test]
fn test_add_ui_with_no_integrate_flag() {
    let temp = setup_dampen_project();
    let project_root = temp.path();
    
    run_dampen_add(project_root, "settings", &["--no-integrate"]);
    
    let mod_content = fs::read_to_string(
        project_root.join("src/ui/mod.rs")
    ).unwrap();
    assert!(!mod_content.contains("pub mod settings;"));
}

#[test]
fn test_add_ui_in_subdirectory() {
    let temp = setup_dampen_project();
    let project_root = temp.path();
    
    run_dampen_add(project_root, "dashboard", &["--path", "src/ui/admin"]);
    
    // Vérifier src/ui/admin/mod.rs
    let admin_mod_content = fs::read_to_string(
        project_root.join("src/ui/admin/mod.rs")
    ).unwrap();
    assert!(admin_mod_content.contains("pub mod dashboard;"));
    
    // Vérifier src/ui/mod.rs
    let ui_mod_content = fs::read_to_string(
        project_root.join("src/ui/mod.rs")
    ).unwrap();
    assert!(ui_mod_content.contains("pub mod admin;"));
}

#[test]
fn test_add_ui_multiple_windows() {
    let temp = setup_dampen_project();
    let project_root = temp.path();
    
    run_dampen_add(project_root, "settings", &[]);
    run_dampen_add(project_root, "profile", &[]);
    run_dampen_add(project_root, "admin", &[]);
    
    let mod_content = fs::read_to_string(
        project_root.join("src/ui/mod.rs")
    ).unwrap();
    assert!(mod_content.contains("pub mod window;"));
    assert!(mod_content.contains("pub mod settings;"));
    assert!(mod_content.contains("pub mod profile;"));
    assert!(mod_content.contains("pub mod admin;"));
}

#[test]
fn test_add_ui_compiles_successfully() {
    let temp = setup_dampen_project();
    let project_root = temp.path();
    
    run_dampen_add(project_root, "settings", &[]);
    
    let output = Command::new("cargo")
        .arg("build")
        .current_dir(project_root)
        .output()
        .unwrap();
    
    assert!(
        output.status.success(),
        "Project should compile after adding window"
    );
}

#[test]
fn test_add_ui_nested_subdirectory() {
    let temp = setup_dampen_project();
    let project_root = temp.path();
    
    run_dampen_add(
        project_root,
        "sales_report",
        &["--path", "src/ui/admin/reports"]
    );
    
    // Vérifier src/ui/admin/reports/mod.rs
    let reports_mod = fs::read_to_string(
        project_root.join("src/ui/admin/reports/mod.rs")
    ).unwrap();
    assert!(reports_mod.contains("pub mod sales_report;"));
    
    // Vérifier src/ui/admin/mod.rs
    let admin_mod = fs::read_to_string(
        project_root.join("src/ui/admin/mod.rs")
    ).unwrap();
    assert!(admin_mod.contains("pub mod reports;"));
    
    // Vérifier src/ui/mod.rs
    let ui_mod = fs::read_to_string(
        project_root.join("src/ui/mod.rs")
    ).unwrap();
    assert!(ui_mod.contains("pub mod admin;"));
}
```

---

## 📊 Matrice de Tests

| Scenario | Test Type | Assertion Clé |
|----------|-----------|---------------|
| Ajouter 1ère fenêtre | E2E | `mod.rs` contient le module |
| Ajouter 2ème fenêtre | E2E | `mod.rs` contient les 2 modules |
| Ajouter module existant | Unitaire | Pas de duplication |
| Flag `--no-integrate` | E2E | `mod.rs` non modifié |
| `mod.rs` inexistant | Unitaire | Fichier créé automatiquement |
| Sous-répertoire (1 niveau) | E2E | `admin/mod.rs` + `ui/mod.rs` mis à jour |
| Sous-répertoire (nested) | E2E | 3 niveaux de `mod.rs` mis à jour |
| Whitespace variations | Unitaire | Détection correcte |
| Compilation après ajout | E2E | `cargo build` succès |
| Répertoire inexistant | Unitaire | Créé automatiquement |
| Préservation modules existants | Unitaire | Modules existants intacts |

---

## 🎨 Expérience Utilisateur

### Cas 1 : Ajout Simple (Défaut)

```bash
$ dampen add --ui settings
✓ Created UI window 'settings'
  → src/ui/settings.rs
  → src/ui/settings.dampen
  → Updated src/ui/mod.rs

Next steps:
  1. Run `dampen check` to validate
  2. Run your application to see the new window

$ cargo build
   Compiling tuto v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 2.34s
```

### Cas 2 : Sous-Répertoire

```bash
$ dampen add --ui dashboard --path "src/ui/admin"
✓ Created UI window 'dashboard'
  → src/ui/admin/dashboard.rs
  → src/ui/admin/dashboard.dampen
  → Updated src/ui/admin/mod.rs
  → Updated src/ui/mod.rs (registered 'admin' module)

Next steps:
  1. Run `dampen check` to validate
  2. Run your application to see the new window
```

### Cas 3 : Avec `--no-integrate`

```bash
$ dampen add --ui settings --no-integrate
✓ Created UI window 'settings'
  → src/ui/settings.rs
  → src/ui/settings.dampen

Next steps:
  1. Add `pub mod settings;` to the appropriate mod.rs file
  2. Run `dampen check` to validate
  3. Run your application to see the new window
```

### Cas 4 : Erreur (Warning Non-Fatal)

```bash
$ dampen add --ui settings
✓ Created UI window 'settings'
  → src/ui/settings.rs
  → src/ui/settings.dampen
⚠ Warning: Failed to update src/ui/mod.rs: Permission denied
  Please manually add: pub mod settings;

Next steps:
  1. Add `pub mod settings;` to the appropriate mod.rs file
  2. Run `dampen check` to validate
  3. Run your application to see the new window
```

---

## 📚 Documentation à Mettre à Jour

### 1. `AGENTS.md`

Mettre à jour la section "Adding New UI Windows" :

```markdown
### Adding New UI Windows

Use `dampen add` to scaffold additional UI windows:

```bash
# Add a settings window (automatically integrated)
dampen add --ui settings

# Add a window in a subdirectory
dampen add --ui dashboard --path "src/ui/admin"

# Add a window without automatic integration
dampen add --ui admin_panel --no-integrate

# Window names are auto-converted to snake_case
dampen add --ui UserProfile
# → Creates: user_profile.rs, user_profile.dampen
```

**What it creates:**

```
src/ui/
├── settings.rs         # Model with #[derive(UiModel)], handlers
└── settings.dampen    # XML UI with basic layout
```

**Automatic Integration:**

By default, the command automatically:
- Adds `pub mod <window_name>;` to the appropriate `mod.rs` file
- Creates the `mod.rs` file if it doesn't exist
- Registers parent modules in the hierarchy (e.g., `admin/` in `ui/mod.rs`)

Use `--no-integrate` to disable automatic integration and handle module registration manually.

**Examples:**

```bash
# Simple window
dampen add --ui settings
# → Updates src/ui/mod.rs with `pub mod settings;`

# Nested subdirectory
dampen add --ui sales_report --path "src/ui/admin/reports"
# → Updates src/ui/admin/reports/mod.rs with `pub mod sales_report;`
# → Creates and updates src/ui/admin/mod.rs with `pub mod reports;`
# → Updates src/ui/mod.rs with `pub mod admin;`
```
```

### 2. CLI Help Text

```rust
/// Add a new UI window
///
/// This command generates a new UI window with all necessary boilerplate
/// and automatically integrates it into your project.
///
/// Generated files:
///   - <window_name>.rs      Rust module with Model and handlers
///   - <window_name>.dampen  XML UI definition
///
/// Automatic integration (disable with --no-integrate):
///   - Adds module declaration to the appropriate mod.rs file
///   - Creates mod.rs files if they don't exist
///   - Registers parent modules in the hierarchy
///
/// Examples:
///   dampen add --ui settings
///   dampen add --ui admin_panel --path "src/ui/admin"
///   dampen add --ui settings --no-integrate
```

### 3. `specs/002-cli-add-ui-command/spec.md`

Ajouter une section "Automatic Integration" :

```markdown
## Automatic Integration (Phase 1+2)

The command performs automatic module registration:

### Module Registration

When creating a window, the command:
1. Adds `pub mod <window_name>;` to the appropriate `mod.rs` file
2. Creates the `mod.rs` file if it doesn't exist
3. Prevents duplicate declarations

### Hierarchical Registration

For nested directories (`--path "src/ui/admin/reports"`):
1. Creates `src/ui/admin/reports/mod.rs` with the window module
2. Creates/updates `src/ui/admin/mod.rs` with `pub mod reports;`
3. Creates/updates `src/ui/mod.rs` with `pub mod admin;`

### Disable Integration

Use `--no-integrate` to skip automatic integration:

```bash
dampen add --ui settings --no-integrate
```

### Error Handling

Integration failures are non-fatal:
- Files are created successfully
- A warning is displayed with manual instructions
- The command exits with code 0 (success)
```

---

## ✅ Checklist de Validation

### Développement
- [ ] Créer `crates/dampen-cli/src/commands/add/integration.rs`
- [ ] Implémenter `add_module_to_mod_rs()`
- [ ] Implémenter `ensure_parent_modules_registered()`
- [ ] Implémenter `is_module_declared()`
- [ ] Implémenter `append_module_declaration()`
- [ ] Ajouter `IntegrationError` dans `errors.rs`
- [ ] Étendre `TargetPath` avec `project_root`
- [ ] Ajouter flag `--no-integrate` dans `AddArgs`
- [ ] Modifier `generate_window_files()` pour appeler l'intégration
- [ ] Ajouter champ `updated_mod_file` à `GeneratedFiles`
- [ ] Mettre à jour `success_message()` avec chemin exact
- [ ] Mettre à jour `execute()` pour passer `enable_integration`
- [ ] Ajouter dépendance `regex` dans `Cargo.toml`

### Tests Unitaires (integration.rs) - 16 tests
- [ ] `is_module_declared()` : cas simples
- [ ] `is_module_declared()` : multiples modules
- [ ] `is_module_declared()` : whitespace et indentation
- [ ] `is_module_declared()` : ignore les commentaires
- [ ] `is_module_declared()` : noms similaires
- [ ] `append_module_declaration()` : fichier vide
- [ ] `append_module_declaration()` : ajout à l'existant
- [ ] `append_module_declaration()` : sans newline finale
- [ ] `add_module_to_mod_rs()` : crée nouveau `mod.rs`
- [ ] `add_module_to_mod_rs()` : append à l'existant
- [ ] `add_module_to_mod_rs()` : prévient les doublons
- [ ] `add_module_to_mod_rs()` : crée répertoire si manquant
- [ ] `ensure_parent_modules_registered()` : 1 niveau
- [ ] `ensure_parent_modules_registered()` : niveaux imbriqués
- [ ] `ensure_parent_modules_registered()` : préserve existant

### Tests d'Intégration E2E - 7 tests
- [ ] Ajouter 1ère fenêtre → `mod.rs` mis à jour
- [ ] Flag `--no-integrate` → `mod.rs` non modifié
- [ ] Sous-répertoire (1 niveau) → 2 `mod.rs` mis à jour
- [ ] Ajouter plusieurs fenêtres → tous dans `mod.rs`
- [ ] Compilation réussie après ajout
- [ ] Sous-répertoire imbriqué → 3 niveaux de `mod.rs`
- [ ] Warning affiché si erreur d'I/O

### Documentation
- [ ] Mettre à jour `AGENTS.md`
- [ ] Mettre à jour aide CLI (`--help`)
- [ ] Ajouter commentaires rustdoc complets
- [ ] Mettre à jour `specs/002-cli-add-ui-command/spec.md`
- [ ] Créer `docs/ADD_COMMAND_ENHANCED.md` (ce document)

### Validation Finale
- [ ] `cargo test --workspace` passe
- [ ] `cargo clippy --workspace -- -D warnings` passe
- [ ] `cargo fmt --all -- --check` passe
- [ ] Test manuel : `dampen new test && cd test && dampen add --ui settings`
- [ ] Test manuel : Vérifier que le projet compile
- [ ] Test manuel : Tester `--no-integrate`
- [ ] Test manuel : Tester sous-répertoire `--path "src/ui/admin"`

---

## 🚀 Estimation de Temps

| Tâche | Complexité | Estimation |
|-------|-----------|-----------|
| Tâche 1 : `integration.rs` (core logic) | Moyenne | 2h |
| Tâche 2-6 : Intégration (flags, generate, execute) | Faible | 1h |
| Tâche 7 : Dépendance | Triviale | 5min |
| Tests unitaires (16 tests) | Moyenne | 2h |
| Tests E2E (7 tests) | Moyenne | 1.5h |
| Documentation | Faible | 45min |
| Validation et debugging | Moyenne | 1h |
| **TOTAL** | | **~8-9 heures** |

---

## 📌 Prochaines Étapes (Phase 3 - Future)

La **Phase 3** implémentera l'activation automatique du view switching dans `main.rs` :

1. Détection du nombre de fenêtres existantes
2. Décommenter `SwitchToView(CurrentView)` dans l'enum `Message` (2ème fenêtre uniquement)
3. Décommenter `switch_view_variant` et `default_view` dans `#[dampen_app]`
4. Détection dynamique du premier module pour `default_view`
5. Demande de confirmation avant modification de `main.rs`

Cette phase sera planifiée séparément une fois la Phase 1+2 complétée et validée.

---

## 🎉 Résumé des Bénéfices

### Pour les Développeurs

**Avant** :
```bash
dampen add --ui settings
# 1. Ouvrir src/ui/mod.rs
# 2. Ajouter manuellement `pub mod settings;`
# 3. Compiler → erreur si oubli
```

**Après** :
```bash
dampen add --ui settings
# ✅ Tout est automatique, compile immédiatement
```

### Réduction du Temps

- Création manuelle : ~2-3 minutes (ouvrir fichier, éditer, sauvegarder)
- Création automatique : <1 seconde
- **Gain : ~2+ minutes par fenêtre**

### Réduction des Erreurs

- Élimination des oublis de déclaration de modules
- Élimination des fautes de frappe dans les noms de modules
- Élimination des erreurs de compilation dues aux modules manquants

---

**Fin du document**
