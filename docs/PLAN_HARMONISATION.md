# Plan d'Harmonisation Dampen (Interpreted vs Codegen)

Ce document détaille la stratégie pour atteindre une parité stricte (100%) entre les modes Interpreted (Dev) et Codegen (Prod) de Dampen, suite à l'analyse des divergences et aux décisions architecturales validées.

## 🎯 Objectifs Stratégiques

1.  **Parité Stricte** : Le même code XML doit produire exactement le même rendu visuel et comportemental en mode Dev et Prod.
2.  **Standardisation** : Définition d'un contrat d'attributs unique (Source of Truth) indépendant des limitations actuelles de l'un ou l'autre mode.
3.  **Qualité Visuelle** : Garantie par des tests de non-régression visuelle (pixel-perfect).

## 📋 Décisions Validées

*   **Breaking Changes** : Autorisés pour garantir la cohérence (Option C).
*   **Source de Vérité** : Nouveau contrat standardisé, ni l'un ni l'autre des modes actuels (Option C).
*   **Stratégie de Test** : Complète (Visuel + Structurel + Property-based) (Option D).
*   **Hot-Reload** : Doit rester strictement identique au mode interprété (Option A).
*   **Styling** : Porting complet du state-aware styling vers le codegen (Option A).

---

## 📅 Roadmap Détaillée

### Phase 1 : Fondation & Sécurité (Semaine 1)

Cette phase met en place les outils pour mesurer le succès et prévenir les régressions pendant le refactoring.

#### 1.1 Infrastructure de Tests Visuels (`crates/dampen-visual-tests`)
*   [ ] Créer un harnais de test basé sur `iced_renderer` en mode headless.
*   [ ] Implémenter un comparateur d'images (diffing avec seuil de tolérance).
*   [ ] Créer un script pour générer les snapshots de référence (baseline) à partir du mode Interpreted actuel (considéré comme "visuellement correct" pour l'instant).

#### 1.2 Définition du Contrat Standard (`specs/attributes.md`)
*   [ ] Créer un registre exhaustif des attributs pour chaque widget.
*   [ ] Standardiser les noms (ex: `active` vs `toggled` -> décision unique).
*   [ ] Standardiser les boucles (ex: adoption de `for item in items` comme standard).
*   [ ] Documenter le comportement attendu pour chaque attribut (layout, style, event).

### Phase 2 : Unification du Layout (Semaine 2)

L'objectif est que tous les conteneurs (Column, Row, Container, Scrollable) se comportent exactement de la même manière.

#### 2.1 Refactor de l'IR (`dampen-core`)
*   [ ] Mettre à jour `WidgetNode` pour inclure explicitement tous les attributs de layout standardisés.
*   [ ] Supprimer les hacks spécifiques à un mode dans le parser.

#### 2.2 Mise à niveau du Codegen (`dampen-core/codegen`)
*   [ ] Implémenter la logique de `apply_style_layout` (actuellement dynamique) en génération de code statique.
    *   *Challenge* : Générer automatiquement les wrappers `Container` quand des attributs de width/height/padding sont détectés sur des widgets qui ne les supportent pas nativement.
*   [ ] Ajouter le support `align_x` / `align_y` pour Column et Row en codegen.
*   [ ] Ajouter le support `width` / `height` pour Scrollable en codegen.

### Phase 3 : State-Aware Styling en Codegen (Semaine 3-4) 🌶️

C'est la phase la plus complexe techniquement. Le mode interprété résout les styles dynamiquement à l'exécution. Le codegen doit générer du code Rust statique qui implémente les traits `StyleSheet` d'Iced avec toute la logique conditionnelle.

#### 3.1 Générateur de Styles (`dampen-core/codegen/style.rs`)
*   [ ] Concevoir un générateur de `impl button::StyleSheet` (et autres) capable de mapper les attributs XML (`background:hover="..."`) vers du code Rust.
*   [ ] Supporter les pseudo-classes : `hover`, `active`, `focused`, `disabled`.
*   [ ] Gérer la précédence des styles (Inline > Class > Default).

#### 3.2 Implémentation par Widget
*   [ ] **Button** : Générer styles pour background, text color, border, shadow.
*   [ ] **TextInput** : Générer styles pour active, focused, placeholder.
*   [ ] **Checkbox/Radio/Toggler** : Générer styles pour checked/unchecked + hover.

### Phase 4 : Harmonisation des Widgets (Semaine 5)

Traitement des divergences spécifiques widget par widget identifiées dans l'audit.

*   [ ] **Boucles (`For`)** : Unifier la syntaxe. Mettre à jour parser et codegen pour supporter la nouvelle syntaxe standard.
*   [ ] **TextInput** : Ajouter support `password` et `color` en codegen.
*   [ ] **Slider** : Ajouter support `step` en codegen.
*   [ ] **Image/SVG** : Unifier `src` vs `path`.
*   [ ] **Validation croisée** : Vérifier chaque widget avec la suite de tests visuels.

### Phase 5 : Documentation & Migration (Semaine 6)

*   [ ] Mettre à jour `docs/XML_SCHEMA.md` avec le nouveau standard.
*   [ ] Écrire un guide de migration pour les utilisateurs existants (changements de noms d'attributs).
*   [ ] (Optionnel) Créer un outil CLI `dampen migrate` pour mettre à jour automatiquement les fichiers `.dampen`.

---

## 🛠️ Matrice de Responsabilité Technique

| Composant | Responsabilité | Actions Clés |
|-----------|----------------|--------------|
| **dampen-core** | Parser & IR | Validation stricte des attributs selon le nouveau standard. |
| **dampen-iced** | Runtime (Dev) | Adapter le `Interpreter` pour respecter le nouveau standard (supprimer le laxisme). |
| **dampen-macros** | Codegen (Prod) | Réécriture majeure pour inclure le layout wrapping et la génération de styles complexes. |
| **dampen-cli** | Tooling | Ajout des commandes de test visuel. |

## risk Assessment

*   **Complexité du Codegen** : Le code généré risque de devenir verbeux. Il faudra peut-être introduire des macros helper dans `dampen-iced` pour simplifier le code généré.
*   **Temps de Compilation** : La génération de styles complexes pourrait impacter le temps de compilation des projets utilisateurs. À surveiller.
