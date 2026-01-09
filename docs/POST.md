# Dampen - A declarative UI framework for Rust

Hey folks,

I've been working on **Dampen**, a declarative UI framework for Rust that might interest some of you.

## What is it?

It's a framework that lets you build native desktop apps using XML definitions for your UI structure, backed by Iced. The idea is to separate UI layout from logic in a type-safe way.

## Why XML?

I wanted a clear separation between UI structure and application logic. XML provides a familiar declarative format that's easy to parse and validate at build time.

## Features

- 📝 **Declarative UI definitions** - XML-based widget layouts
- 🔥 **Type-safe bindings** - Derive macros connect your Rust structs to the UI
- ⚡ **Build-time code generation** - Zero runtime overhead
- 🎨 **Iced backend** - Leverages existing widget ecosystem
- 🛠️ **CLI tooling** - Project scaffolding and validation

## Example

```xml
<column padding="20" spacing="10">
    <text value="Counter: {count}" size="24" />
    <button label="Increment" on_click="increment" />
</column>
```

```rust
#[derive(UiModel)]
struct Model {
    count: i32,
}
```

## Status

The core features are working. There are examples (todo app, counter, settings) demonstrating the main patterns. It's usable but still evolving.

## Try it

```bash
cargo install dampen-cli
dampen new my-app
cd my-app
cargo run
```

Check out the repo: [Your GitHub link here]

Feedback and contributions welcome.
