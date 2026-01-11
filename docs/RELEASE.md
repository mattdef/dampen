# Release Guide

Ce document explique comment publier une nouvelle version de Dampen sur crates.io.

## Configuration initiale (une seule fois)

### 1. Obtenir un token crates.io

1. Connectez-vous sur [crates.io](https://crates.io)
2. Allez dans **Account Settings** → **API Tokens**
3. Cliquez sur **New Token**
4. Donnez un nom au token (ex: "GitHub Actions - Dampen")
5. **Important** : Sélectionnez les scopes appropriés :
   - ✅ `publish-update` (pour publier les crates)
6. Copiez le token généré (vous ne pourrez plus le voir après)

### 2. Configurer le secret GitHub

1. Allez sur votre repository GitHub : `https://github.com/mattdef/dampen`
2. Cliquez sur **Settings** → **Secrets and variables** → **Actions**
3. Cliquez sur **New repository secret**
4. Nom : `CARGO_TOKEN`
5. Valeur : Collez le token crates.io
6. Cliquez sur **Add secret**

### 3. Vérifier la configuration du Cargo.toml

Assurez-vous que toutes les crates ont les métadonnées nécessaires :

```toml
[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
authors = ["Mattdef <mattdef@gmail.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/mattdef/dampen"
documentation = "https://docs.rs/dampen-cli"
readme = "README.md"
keywords = ["ui", "iced", "declarative", "gui", "framework"]
categories = ["gui", "development-tools"]
publish = true  # ← Important !
```

## Processus de release

### Méthode automatique (recommandée)

Utilisez le script de release :

```bash
# Exemple pour la version 0.2.0
./scripts/release.sh 0.2.0
```

Le script va :
1. ✅ Vérifier que les tests passent
2. ✅ Vérifier clippy
3. ✅ Vérifier le formatage
4. ✅ Mettre à jour les versions dans `Cargo.toml` :
   - `[workspace.package] version`
   - Toutes les crates dampen dans `[workspace.dependencies]`
5. ✅ Créer un commit et un tag git
6. ℹ️ Vous donner les instructions pour pousser

Ensuite, suivez les instructions affichées :

```bash
# 1. Pousser les changements
git push origin master
git push origin v0.2.0

# 2. Créer la release sur GitHub
# Allez sur : https://github.com/mattdef/dampen/releases/new?tag=v0.2.0
```

### Méthode manuelle

Si vous préférez faire manuellement :

#### 1. Préparer la release

```bash
# Vérifier que tout est clean
git status

# Vérifier que les tests passent
cargo test --workspace

# Mettre à jour les versions dans Cargo.toml
vim Cargo.toml
# Modifier [workspace.package] version = "0.2.0"
# ET modifier toutes les versions dans [workspace.dependencies]:
#   dampen-core = { path = "./crates/dampen-core", version = "0.2.0" }
#   dampen-macros = { path = "./crates/dampen-macros", version = "0.2.0" }
#   etc.

# Commit et tag
git add Cargo.toml
git commit -m "chore: bump version to 0.2.0"
git tag -a v0.2.0 -m "Release v0.2.0"
```

#### 2. Pousser sur GitHub

```bash
git push origin master
git push origin v0.2.0
```

#### 3. Créer la GitHub Release

1. Allez sur https://github.com/mattdef/dampen/releases/new
2. Sélectionnez le tag `v0.2.0`
3. Titre : `v0.2.0`
4. Description : Listez les changements (voir CHANGELOG.md)
5. Cliquez sur **Publish release**

#### 4. Publication automatique

La GitHub Action `.github/workflows/release.yml` va automatiquement :

1. Vérifier que la version du tag correspond à `Cargo.toml`
2. Lancer les tests
3. Publier les crates dans l'ordre :
   - `dampen-core`
   - `dampen-macros` (dépend de core)
   - `dampen-iced` (dépend de core)
   - `dampen-cli` (dépend de tout)

Vous pouvez suivre le progrès dans l'onglet **Actions** sur GitHub.

## Ordre de publication des crates

**Important** : Les crates doivent être publiées dans cet ordre (le workflow s'en occupe) :

```
dampen-core
    ↓
dampen-macros
    ↓
dampen-iced
    ↓
dampen-cli
```

Le workflow attend 30 secondes entre chaque publication pour que crates.io puisse indexer la nouvelle version.

## Vérification

Après la publication, vérifiez que les crates sont disponibles :

- https://crates.io/crates/dampen-core
- https://crates.io/crates/dampen-macros
- https://crates.io/crates/dampen-iced
- https://crates.io/crates/dampen-cli

## En cas de problème

### La publication échoue

1. Vérifiez les logs dans l'onglet **Actions** sur GitHub
2. Erreurs communes :
   - **Token invalide** : Vérifiez que `CARGO_TOKEN` est bien configuré
   - **Version déjà publiée** : Vous ne pouvez pas republier la même version
   - **Dépendance manquante** : Attendez que crates.io indexe les crates précédentes

### Annuler une release

**Attention** : Une fois publiée sur crates.io, une version **ne peut pas être supprimée** (seulement "yanked").

Pour annuler avant la publication :

```bash
# Supprimer le tag localement
git tag -d v0.2.0

# Supprimer le tag sur GitHub (si déjà poussé)
git push origin :refs/tags/v0.2.0

# Annuler le commit
git reset --hard HEAD~1
```

### Yank une version publiée

Si vous découvrez un problème critique après publication :

```bash
cargo yank --vers 0.2.0 dampen-core
cargo yank --vers 0.2.0 dampen-macros
cargo yank --vers 0.2.0 dampen-iced
cargo yank --vers 0.2.0 dampen-cli
```

Puis publiez une version corrective (0.2.1).

## Checklist pré-release

- [ ] Tous les tests passent (`cargo test --workspace`)
- [ ] Clippy est clean (`cargo clippy --workspace -- -D warnings`)
- [ ] Le code est formaté (`cargo fmt --all -- --check`)
- [ ] CHANGELOG.md est à jour
- [ ] README.md reflète les nouvelles fonctionnalités
- [ ] La documentation est à jour (`cargo doc --workspace`)
- [ ] Les exemples fonctionnent
- [ ] Le secret `CARGO_TOKEN` est configuré sur GitHub
- [ ] La branche est à jour avec `origin/master`

## Workflow typique

```bash
# Développement
git checkout -b feature/nouvelle-fonctionnalite
# ... développement ...
git commit -m "feat: ajouter nouvelle fonctionnalité"
git push origin feature/nouvelle-fonctionnalite

# PR et merge vers master
# ... review et merge ...

# Release
git checkout master
git pull origin master
./scripts/release.sh 0.2.0
git push origin master
git push origin v0.2.0

# Créer la GitHub Release
# → Visite https://github.com/mattdef/dampen/releases/new?tag=v0.2.0
# → La publication sur crates.io se fait automatiquement
```

## Resources

- [Cargo Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io API Tokens](https://crates.io/settings/tokens)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Semantic Versioning](https://semver.org/)
