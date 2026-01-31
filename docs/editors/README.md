# Editor Integration Guide

This guide explains how to configure various editors to use the Dampen LSP server for `.dampen` files.

## Overview

The Dampen LSP server provides:
- **Real-time validation**: Syntax and semantic error detection
- **Intelligent autocompletion**: Context-aware suggestions for widgets, attributes, and values
- **Hover documentation**: Tooltips showing widget and attribute documentation

The server communicates via JSON-RPC over stdio, which is the standard LSP transport mechanism.

---

## VS Code

### Installation

1. Build the LSP server:
   ```bash
   cargo build --release -p dampen-lsp
   ```

2. The binary will be at `target/release/dampen-lsp`

### Configuration

Create or edit `.vscode/settings.json` in your project:

```json
{
  "dampen.lsp.serverPath": "${workspaceFolder}/target/release/dampen-lsp"
}
```

Or configure globally in VS Code settings:

```json
{
  "dampen.lsp.serverPath": "/path/to/dampen-lsp"
}
```

### Extension (Optional)

For a better experience, you can create a minimal VS Code extension:

**package.json:**
```json
{
  "name": "dampen-lsp-client",
  "displayName": "Dampen LSP Client",
  "version": "1.0.0",
  "engines": {
    "vscode": "^1.74.0"
  },
  "categories": ["Programming Languages"],
  "activationEvents": [
    "onLanguage:dampen"
  ],
  "main": "./out/extension.js",
  "contributes": {
    "languages": [{
      "id": "dampen",
      "aliases": ["Dampen", "dampen"],
      "extensions": [".dampen"],
      "configuration": "./language-configuration.json"
    }]
  }
}
```

**extension.js:**
```javascript
const vscode = require('vscode');
const { LanguageClient } = require('vscode-languageclient/node');

let client;

function activate(context) {
    const serverOptions = {
        command: 'dampen-lsp',
        args: []
    };

    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'dampen' }]
    };

    client = new LanguageClient(
        'dampen-lsp',
        'Dampen Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
}

function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

module.exports = { activate, deactivate };
```

---

## Zed

### Configuration

Zed has native LSP support. Add to your `~/.config/zed/settings.json`:

```json
{
  "lsp": {
    "dampen-lsp": {
      "binary": {
        "path": "/path/to/dampen-lsp",
        "arguments": []
      },
      "settings": {}
    }
  },
  "languages": {
    "Dampen": {
      "language_servers": ["dampen-lsp"]
    }
  }
}
```

### File Association

Add to `~/.config/zed/settings.json` to associate `.dampen` files:

```json
{
  "file_types": {
    "Dampen": ["*.dampen"]
  }
}
```

---

## Neovim

### Using nvim-lspconfig

Add to your Neovim configuration:

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

-- Define Dampen LSP if not already defined
if not configs.dampen_lsp then
  configs.dampen_lsp = {
    default_config = {
      cmd = {'dampen-lsp'},
      filetypes = {'dampen'},
      root_dir = function(fname)
        return lspconfig.util.find_git_ancestor(fname) or vim.fn.getcwd()
      end,
      settings = {},
    },
  }
end

-- Setup
lspconfig.dampen_lsp.setup{}
```

### Using mason.nvim

If using Mason for LSP management, you can register a custom server:

```lua
require('mason-lspconfig').setup_handlers({
  function(server_name)
    require('lspconfig')[server_name].setup({})
  end,
  ['dampen_lsp'] = function()
    require('lspconfig').dampen_lsp.setup({
      cmd = {vim.fn.expand('~/path/to/dampen-lsp')},
    })
  end,
})
```

### Filetype Detection

Add to `~/.config/nvim/ftdetect/dampen.vim`:

```vim
au BufRead,BufNewFile *.dampen set filetype=dampen
```

---

## Vim

### Using coc.nvim

Add to `coc-settings.json` (access via `:CocConfig`):

```json
{
  "languageserver": {
    "dampen": {
      "command": "dampen-lsp",
      "filetypes": ["dampen"],
      "rootPatterns": [".git/", "Cargo.toml"],
      "settings": {}
    }
  }
}
```

### Filetype Detection

Add to `~/.vimrc`:

```vim
autocmd BufRead,BufNewFile *.dampen set filetype=dampen
```

---

## Emacs

### Using lsp-mode

Add to your Emacs configuration:

```elisp
(use-package lsp-mode
  :hook (dampen-mode . lsp)
  :commands lsp)

;; Define Dampen mode
(define-derived-mode dampen-mode xml-mode "Dampen"
  "Major mode for editing Dampen UI files.")

(add-to-list 'auto-mode-alist '("\\.dampen\\'" . dampen-mode))

;; Register LSP client
(with-eval-after-load 'lsp-mode
  (add-to-list 'lsp-language-id-configuration '(dampen-mode . "dampen"))
  (lsp-register-client
   (make-lsp-client :new-connection (lsp-stdio-connection "dampen-lsp")
                    :activation-fn (lsp-activate-on "dampen")
                    :server-id 'dampen-lsp)))
```

### Using eglot

```elisp
(require 'eglot)

(add-to-list 'eglot-server-programs
             '(dampen-mode . ("dampen-lsp")))

(define-derived-mode dampen-mode xml-mode "Dampen")
(add-to-list 'auto-mode-alist '("\\.dampen\\'" . dampen-mode))
```

---

## Sublime Text

### Using LSP Package

1. Install the LSP package via Package Control
2. Open LSP settings: **Preferences > Package Settings > LSP > Settings**
3. Add:

```json
{
  "clients": {
    "dampen-lsp": {
      "enabled": true,
      "command": ["/path/to/dampen-lsp"],
      "selector": "source.dampen",
      "file_patterns": ["*.dampen"]
    }
  }
}
```

### Syntax Definition

Create `Dampen.sublime-syntax`:

```yaml
%YAML 1.2
---
name: Dampen
file_extensions:
  - dampen
scope: source.dampen
contexts:
  main:
    - include: scope:text.xml
```

---

## Helix

Add to `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "dampen"
scope = "source.dampen"
injection-regex = "dampen"
file-types = ["dampen"]
roots = ["Cargo.toml"]
comment-token = "<!--"
indent = { tab-width = 4, unit = "    " }

[language-server.dampen-lsp]
command = "dampen-lsp"

[[grammar]]
name = "dampen"
source = { path = "path/to/xml/grammar" }
```

---

## Kate

Add to **Settings > Configure Kate > LSP Client**:

```json
{
  "servers": {
    "dampen": {
      "command": ["dampen-lsp"],
      "url": "https://dampen.io",
      "highlightingModeRegex": "^(Dampen|XML)$"
    }
  }
}
```

---

## Troubleshooting

### Server Not Starting

1. Verify the binary exists and is executable:
   ```bash
   which dampen-lsp
   dampen-lsp --version  # If supported
   ```

2. Check editor logs for LSP errors

3. Test the server manually:
   ```bash
   echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | dampen-lsp
   ```

### No Diagnostics Showing

1. Ensure the file has `.dampen` extension
2. Check that the document was opened (check LSP logs)
3. Verify the server is parsing correctly by checking logs

### Performance Issues

The LSP server has the following performance targets:
- Parse time: <50ms for 1000-line files
- Completion: <100ms
- Hover: <200ms
- Diagnostics: <500ms

If experiencing slowdowns:
1. Check file size (recommended <1000 lines)
2. Verify cache is working (max 50 documents)
3. Check for excessive document updates

---

## Environment Variables

The LSP server respects the following environment variables:

- `RUST_LOG`: Set to `debug` for verbose logging
  ```bash
  RUST_LOG=debug dampen-lsp
  ```

---

## Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/dampen.git
cd dampen

# Build the LSP server
cargo build --release -p dampen-lsp

# The binary will be at:
# target/release/dampen-lsp
```

---

## Additional Resources

- [LSP Specification](https://microsoft.github.io/language-server-protocol/)
- [Dampen Documentation](https://dampen.io/docs)
- [Tower-LSP Documentation](https://docs.rs/tower-lsp/)
