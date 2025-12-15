# jinab

A command-line tool to read and search the web using [Jina AI's Reader API](https://jina.ai/reader). Renders JavaScript-heavy pages to clean markdown, outputs to stdout for easy piping and scripting.

## Installation

### From releases

Download the latest binary for your platform from [Releases](https://github.com/ominiverdi/jinab/releases).

### From source

```bash
cargo install --path .
```

## Setup

Get a free API key from [jina.ai](https://jina.ai/reader), then:

```bash
jinab key YOUR_API_KEY
```

This saves the key to `~/.config/jinab/config`. Alternatively, set the `JINA_API_KEY` environment variable.

## Usage

### Read a webpage

```bash
jinab read https://example.com
```

Output as JSON:

```bash
jinab read https://example.com --json
```

### Search the web

```bash
jinab search "rust async programming"
```

Output as JSON:

```bash
jinab search "rust async programming" --json
```

### Pipe output

```bash
jinab read https://docs.rs/tokio | grep "runtime"
jinab search "rust error handling" | head -50
```

## Shell Completions

Generate completions for your shell:

```bash
# Bash
jinab completions bash > ~/.local/share/bash-completion/completions/jinab

# Zsh
jinab completions zsh > ~/.zfunc/_jinab

# Fish
jinab completions fish > ~/.config/fish/completions/jinab.fish

# PowerShell
jinab completions powershell > _jinab.ps1
```

## License

MIT
