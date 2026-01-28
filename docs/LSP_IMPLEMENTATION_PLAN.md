# Plan d'implementation du Language Server Protocol (LSP) pour Dampen

**Date de creation** : 28 Janvier 2026  
**Version** : 1.0  
**Statut** : Plan de conception

---

## Table des matieres

1. [Vue d'ensemble](#vue-densemble)
2. [Architecture](#architecture)
3. [Structure du crate](#structure-du-crate)
4. [Fonctionnalites V1](#fonctionnalites-v1)
5. [Implementation detaillee](#implementation-detaillee)
6. [Strategie de test](#strategie-de-test)
7. [Dependances](#dependances)
8. [Checklist d'implementation](#checklist-dimplementation)

---

## Vue d'ensemble

### Objectif

Implementer un serveur LSP complet pour le langage Dampen (fichiers `.dampen`) offrant une experience de developpement IDE de qualite professionnelle.

### Fonctionnalites V1 ciblees

| Fonctionnalite | Priorite | Description |
|----------------|----------|-------------|
| Validation XML temps reel | Haute | Detection immediate des erreurs de syntaxe et semantique |
| Autocompletion | Haute | Suggestions contextuelles de widgets, attributs et valeurs |
| Diagnostics d'erreur | Haute | Messages d'erreur precis avec positions et suggestions |
| Hover Info | Haute | Documentation au survol des elements |

### Analyse du parser existant

Dampen utilise **roxmltree** comme parser XML, offrant :

- Parsing pull/streaming rapide et fiable
- Positions exactes (line/column) pour chaque noeud
- Gestion robuste des erreurs de syntaxe XML
- Pretraitement des attributs d'etat

Point d'entree principal : `dampen_core::parser::parse(xml: &str)`

Structure d'erreur existante :
- kind: ParseErrorKind
- message: String
- span: Span (start, end, line, column)
- suggestion: Option<String>

---

## Architecture

### Diagramme de haut niveau

```
Client LSP (VS Code/Zed)
         |
         | JSON-RPC over stdio
         v
   dampen-lsp (serveur)
   +------------------+
   |  Server          |
   |  Document Cache  |
   |  Analyzer        |
   +--------+---------+
            |
            v
      dampen-core
   (parse, validation)
```

### Flux de donnees

1. Document ouvert/modifie
2. Mise a jour du cache DocumentState
3. Parsing via dampen-core::parse()
   - Succes : Stockage AST + Validation semantique
   - Echec : Conversion erreurs vers Diagnostics LSP
4. Publication diagnostics vers client
5. Reponses aux requetes (completion, hover)

---

## Structure du crate

```
crates/dampen-lsp/
├── Cargo.toml
├── src/
│   ├── main.rs                 # Point d'entree (stdio)
│   ├── server.rs               # LspServer - orchestration
│   ├── document.rs             # Gestion documents ouverts
│   ├── analyzer.rs             # Analyse semantique
│   ├── capabilities.rs         # Capacites serveur LSP
│   ├── converters.rs           # Conversions types LSP
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── text_document.rs    # did_open, did_change, did_close
│   │   ├── diagnostics.rs      # publish_diagnostics
│   │   ├── completion.rs       # completion, completion_resolve
│   │   └── hover.rs            # hover
│   └── schema_data.rs          # Donnees schema pour LSP
└── tests/
    ├── integration_tests.rs
    ├── handlers_tests.rs
    ├── analyzer_tests.rs
    └── fixtures/
        ├── valid_simple.dampen
        ├── invalid_syntax.dampen
        ├── invalid_widget.dampen
        └── complex_document.dampen
```

---

## Fonctionnalites V1

### 1. Validation XML en temps reel

Declencheur : A chaque modification du document (textDocument/didChange)

Processus :
- Parser le document avec dampen-core::parse()
- En cas de succes : stocker l'AST, valider semantiquement
- En cas d'echec : convertir les ParseError en Diagnostics LSP
- Publier les diagnostics vers le client

Types d'erreurs detectees :
- Syntaxe XML (roxmltree)
- Widget inconnu
- Attribut invalide
- Valeur invalide
- Expression binding malformee
- Version non supportee
- Contrainte semantique

### 2. Autocompletion

Declencheur : textDocument/completion

Contextes de completion :
- WidgetName : position `<|` suggere tous les widgets
- AttributeName : position `<button |>` suggere attributs du widget
- AttributeValue : position `class="|"` suggere valeurs selon type
- BindingExpression : position `value="{|}"` suggere variables

Algorithme de detection :
- Convertir position LSP en offset texte
- Analyser le texte avant le curseur
- Detecter si dans expression binding, valeur d'attribut, ou tag
- Retourner le contexte approprie

Sources de suggestions :
- Widgets : dampen_core::schema::get_widget_schema()
- Attributs : champs attributes du schema du widget
- Valeurs : types primitifs, couleurs, classes, options

### 3. Diagnostics d'erreur

Format LSP :
- range : position de l'erreur (start/end line/character)
- severity : ERROR, WARNING, INFO, HINT
- code : identifiant de l'erreur
- source : "dampen"
- message : description de l'erreur
- related_information : suggestions de correction

Gravite des diagnostics :
- XmlSyntax : ERROR
- UnknownWidget : ERROR
- UnknownAttribute : WARNING
- InvalidValue : ERROR
- InvalidExpression : ERROR
- MissingAttribute : ERROR
- UnsupportedVersion : ERROR
- DeprecatedAttribute : WARNING

### 4. Hover Info

Declencheur : textDocument/hover

Contenu selon le type :

Widget :
- Nom du widget
- Description
- Liste des attributs avec types
- Exemple d'utilisation
- Version minimale requise

Attribut :
- Nom de l'attribut
- Widget parent
- Type attendu
- Description
- Exemples de valeurs

Valeur :
- Selon le contexte (couleur, nombre, etc.)
- Apercu si applicable
- Format attendu

---

## Implementation detaillee

### Module server.rs

Structure LspServer :
- client : Client
- documents : HashMap<Url, DocumentState>

Methodes principales :
- new() : initialisation
- on_change() : gestion changements document
- validate_document() : validation complete

### Module document.rs

Structure DocumentState :
- uri : Url
- version : i32
- content : String
- ast : Option<DampenDocument>
- diagnostics : Vec<Diagnostic>
- parse_errors : Vec<ParseError>

Methodes :
- update_content() : mise a jour contenu et re-parsing
- get_ast() : acces a l'AST parse
- get_diagnostics() : liste des diagnostics

### Module analyzer.rs

Fonctions principales :
- analyze_document() : analyse complete
- get_completion_context() : detection contexte completion
- get_hover_info() : recuperation info hover
- find_widget_at_position() : widget sous le curseur
- find_attribute_at_position() : attribut sous le curseur

### Module converters.rs

Fonctions de conversion :
- span_to_range() : Span vers LSP Range
- range_to_span() : LSP Range vers Span
- parse_error_to_diagnostic() : ParseError vers Diagnostic
- position_to_offset() : Position LSP vers offset texte
- offset_to_position() : offset texte vers Position LSP

### Module handlers/text_document.rs

Handlers LSP :
- did_open() : textDocument/didOpen
- did_change() : textDocument/didChange
- did_close() : textDocument/didClose
- did_save() : textDocument/didSave

### Module handlers/diagnostics.rs

Fonctions :
- publish_diagnostics() : conversion et envoi
- convert_errors() : Vec<ParseError> vers Vec<Diagnostic>

### Module handlers/completion.rs

Handlers LSP :
- completion() : textDocument/completion
- completion_resolve() : completionItem/resolve

Fonctions de completion :
- complete_widget_names() : liste tous les WidgetKind
- complete_attributes() : attributs pour un widget donne
- complete_values() : valeurs selon type d'attribut

### Module handlers/hover.rs

Handlers LSP :
- hover() : textDocument/hover

Fonctions d'hover :
- hover_widget() : documentation widget
- hover_attribute() : documentation attribut
- hover_value() : documentation valeur

### Module schema_data.rs

Donnees statiques pour LSP :
- WIDGET_DOCUMENTATION : HashMap<WidgetKind, String>
- ATTRIBUTE_DOCUMENTATION : HashMap<(WidgetKind, String), String>
- VALUE_SUGGESTIONS : suggestions par type d'attribut

---

## Strategie de test

### Tests unitaires

Par module :
- converters_tests.rs : tests conversions types
- analyzer_tests.rs : tests analyse semantique
- completion_tests.rs : tests autocompletion
- hover_tests.rs : tests hover

### Tests d'integration

integration_tests.rs :
- Test end-to-end avec client LSP simule
- Scenarios complets : ouverture, modification, completion, hover
- Tests avec fixtures

### Fixtures de test

Fichiers .dampen de test :
- valid_simple.dampen : document valide simple
- invalid_syntax.dampen : erreurs de syntaxe XML
- invalid_widget.dampen : widget inconnu
- invalid_attribute.dampen : attribut invalide
- complex_document.dampen : document complexe avec themes
- all_widgets.dampen : exemple de tous les widgets

### Couverture de test

Objectifs :
- Validation XML : 100% des cas d'erreur
- Autocompletion : tous les contextes
- Diagnostics : toutes les gravites
- Hover : tous les types de cibles

---

## Dependances

Cargo.toml :

```toml
[package]
name = "dampen-lsp"
version = { workspace = true }
edition = { workspace = true }

[dependencies]
# LSP framework
tower-lsp = "0.20"
lsp-types = "0.95"

# Core Dampen
dampen-core = { path = "../dampen-core" }

# Async runtime
tokio = { version = "1", features = ["full"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# URL handling
url = "2"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Utilities
regex = "1"

[dev-dependencies]
tempfile = "3"
insta = "1"
```

---

## Checklist d'implementation

### Phase 1 : Infrastructure (Semaine 1)

- [ ] Creer le crate dampen-lsp
- [ ] Configurer Cargo.toml avec dependances
- [ ] Implementer main.rs avec tower-lsp
- [ ] Implementer server.rs structure de base
- [ ] Implementer document.rs cache documents
- [ ] Implementer converters.rs conversions types
- [ ] Ecrire tests unitaires converters

### Phase 2 : Validation et Diagnostics (Semaine 2)

- [ ] Implementer handlers/text_document.rs
- [ ] Implementer handlers/diagnostics.rs
- [ ] Connecter dampen-core::parse() au LSP
- [ ] Convertir ParseError vers Diagnostic
- [ ] Publier diagnostics en temps reel
- [ ] Ecrire tests integration validation
- [ ] Creer fixtures de test erreurs

### Phase 3 : Autocompletion (Semaine 3)

- [ ] Implementer analyzer.rs detection contexte
- [ ] Implementer handlers/completion.rs
- [ ] Implementer complete_widget_names()
- [ ] Implementer complete_attributes()
- [ ] Implementer complete_values()
- [ ] Integrer schema dampen-core
- [ ] Ecrire tests completion

### Phase 4 : Hover (Semaine 4)

- [ ] Implementer schema_data.rs documentation
- [ ] Implementer handlers/hover.rs
- [ ] Implementer hover_widget()
- [ ] Implementer hover_attribute()
- [ ] Implementer hover_value()
- [ ] Ecrire tests hover

### Phase 5 : Tests et Polish (Semaine 5)

- [ ] Atteindre 90% couverture de test
- [ ] Tests end-to-end complets
- [ ] Documentation utilisateur
- [ ] Configuration CI/CD
- [ ] Benchmarks performance
- [ ] Revue de code

### Livrables

- [ ] Crate dampen-lsp fonctionnel
- [ ] Tests complets passant
- [ ] Documentation README.md
- [ ] Guide d'installation
- [ ] Configuration pour VS Code
- [ ] Configuration pour Zed

---

## Notes supplementaires

### Performance

- Parsing complet a chaque modification acceptable pour fichiers < 1000 lignes
- Optimisation incrémentale a envisager pour fichiers plus grands
- Cache AST en memoire pour reponses rapides

### Extensions futures

V2 potentielles :
- Go-to-definition pour bindings
- Renommage de symboles
- Formatage de code
- Code actions (quick fixes)
- Analyse Rust pour validation bindings

### Compatibilite

- MSRV : 1.85 (comme dampen-core)
- Protocole LSP : 3.17
- Editeurs supportes : VS Code, Zed, Vim/Neovim, Emacs

---

**Fin du document**
