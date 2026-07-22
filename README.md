# aka

*Also Known As* — a small command-line tool for managing bash aliases from a single source of truth.

Define your aliases once, and `aka` keeps a generated `aliases.sh` in sync that your shell sources on startup. No more hand-editing `.bashrc` every time you want a new shortcut.

```bash
aka add g "git status"
aka add l "ls -ltr" --description "list files, newest last"
aka list
```

## Install

### Prebuilt binary (recommended)

No dependencies beyond `curl`. Downloads the right binary for your platform and installs it to `~/.local/bin`.

```bash
curl -fsSL https://raw.githubusercontent.com/steffensmartinsen/aka/master/remote-install.sh | bash
```

Prefer to read the script before running it? (Always a good habit with `curl | bash`.)  
You can check it out [here](https://github.com/steffensmartinsen/aka/blob/master/remote-install.sh).

Supported platforms: Linux (x86_64), macOS (Intel and Apple Silicon).

### Build from source

Requires [Rust](https://rustup.rs). Use this if you're on an unsupported platform or want to hack on the tool.

```bash
git clone https://github.com/steffensmartinsen/aka.git
cd aka
./install.sh
```

### PATH note

Both installers place the binary in `~/.local/bin`. This is on your `PATH` by default on most modern systems, but if the installer warns that it isn't, add this to your `~/.bashrc` (or `~/.zshrc` on macOS):

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Then run `source ~/.bashrc`.

## Shell setup

For your aliases to load automatically in new shells, source the generated file from your `~/.bashrc`. Add this once:

```bash
echo '[ -f ~/.config/aka/aliases.sh ] && source ~/.config/aka/aliases.sh' >> ~/.bashrc
source ~/.bashrc
```

The `[ -f ... ] &&` guard means nothing breaks if the file doesn't exist yet.

## Usage

### Add an alias

```bash
aka add <name> "<command>" [--description <text>]
```

```bash
aka add g "git status"
aka add gp "git push" --description "push current branch"
```

Adding an alias updates the store and regenerates `aliases.sh`. To use the new alias in your **current** shell, reload it:

```bash
source ~/.bashrc
```

New shells pick it up automatically.

### Remove an alias

```bash
aka remove <name>
```

### List aliases

```bash
aka list
```

Filter by a search term (matches names and commands):

```bash
aka list --search git
```

### Regenerate

Rewrite `aliases.sh` from the current store without changing anything. Rarely needed — `add` and `remove` do this automatically.

```bash
aka generate
```

## How it works

`aka` stores your aliases as JSON at `~/.config/aka/aliases.json` and generates a bash script alongside it:

```
~/.config/aka/
├── aliases.json   # source of truth (edit via aka, not by hand)
└── aliases.sh     # generated — sourced by your ~/.bashrc
```

Every `add` or `remove` rewrites `aliases.sh`. Your shell reads that generated file, so your aliases live in one place and stay in sync.

## Uninstall

```bash
rm ~/.local/bin/aka        # the binary
rm -rf ~/.config/aka       # aliases and generated files
```

Optionally remove the source line from your `~/.bashrc`.

## License

MIT