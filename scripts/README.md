# Scripts

Ce répertoire contient des scripts utiles pour le développement et la maintenance de Dampen.

## Scripts disponibles

### `release.sh`

Script de release automatisé pour publier une nouvelle version de Dampen.

**Usage :**
```bash
./scripts/release.sh <version>
```

**Exemple :**
```bash
./scripts/release.sh 0.2.0
```

**Ce qu'il fait :**
1. ✅ Vérifie que le répertoire de travail est propre
2. ✅ Lance les tests complets (`cargo test --workspace`)
3. ✅ Vérifie clippy (`cargo clippy --workspace`)
4. ✅ Vérifie le formatage du code
5. ✅ Met à jour les versions dans `Cargo.toml` :
   - `[workspace.package] version = "X.X.X"`
   - Toutes les crates dampen dans `[workspace.dependencies]`
6. ✅ Génère la documentation
7. ✅ Crée un commit git avec le message `chore: bump version to X.X.X`
8. ✅ Crée un tag git `vX.X.X`
9. ℹ️ Affiche les prochaines étapes pour finaliser la release

**Après l'exécution :**

Le script vous donnera les commandes pour :
- Pousser les changements vers GitHub
- Créer la GitHub Release
- La publication sur crates.io se fera automatiquement via GitHub Actions

**Annuler une release (avant push) :**
```bash
git tag -d v0.2.0
git reset --hard HEAD~1
```

### `test-release.sh`

Script de test pour vérifier les mises à jour de version (dry-run, sans modification).

**Usage :**
```bash
./scripts/test-release.sh <version>
```

**Exemple :**
```bash
./scripts/test-release.sh 0.2.0
```

**Ce qu'il fait :**
- 🔍 Affiche les versions actuelles
- 🔍 Simule la mise à jour vers la nouvelle version
- ✅ Vérifie que toutes les versions seraient correctement mises à jour
- ℹ️ N'effectue **aucune modification** (dry-run uniquement)

**Utilité :**
Utilisez ce script avant `release.sh` pour vérifier que les regex de remplacement fonctionnent correctement.

## Ajouter un nouveau script

Lors de l'ajout d'un nouveau script :

1. Créez le fichier dans ce répertoire
2. Ajoutez le shebang : `#!/bin/bash`
3. Rendez-le exécutable : `chmod +x scripts/votre-script.sh`
4. Documentez-le dans ce README
5. Ajoutez des commentaires dans le script

**Structure recommandée :**
```bash
#!/bin/bash
# Description du script
# Usage: ./scripts/mon-script.sh <args>

set -e  # Exit on error

# ... votre code ...
```
