# aka

*Also Known As* — a small command-line tool for managing bash aliases from a single source of truth.

Define your aliases once, and `aka` keeps a generated `aliases.sh` in sync that your shell sources on startup. No more hand-editing `.bashrc` every time you want a new shortcut.

```bash
aka add g "git status"
aka add l "ls -ltr" --description "list files, newest last"
aka list
```

Set up a new machine in one step by importing a profile you exported elsewhere:

```bash
aka export my-aliases.json     # on your old machine
aka import my-aliases.json     # on the new one
```

## Contents

- [Install](#install)
  - [Prebuilt binary (recommended)](#prebuilt-binary-recommended)
  - [Build from source](#build-from-source)
  - [PATH note](#path-note)
- [Shell setup](#shell-setup)
- [Usage](#usage)
  - [Add an alias](#add-an-alias)
  - [Remove an alias](#remove-an-alias)
  - [List aliases](#list-aliases)
  - [Regenerate](#regenerate)
  - [Export a profile](#export-a-profile)
  - [Import a profile](#import-a-profile)
- [Updating](#updating)
- [How it works](#how-it-works)
- [Uninstall](#uninstall)
- [License](#license)

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
aka add <name> <command> [--description <text>]
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

### Export a profile

Dump the current aliases to a portable JSON file — useful for backing them up, committing to a dotfiles repo, or moving to another machine.

```bash
aka export <file>
```

```bash
aka export my-aliases.json
```

### Import a profile

Merge aliases from a profile file into the current store.

```bash
aka import <file>              # merge; abort if any name already exists
aka import <file> --overwrite  # on conflict, the imported alias wins
aka import <file> --keep       # on conflict, the existing alias wins
```

By default, if an incoming alias name already exists locally, the import **aborts** and lists the conflicts — nothing is overwritten without you choosing. Use `--overwrite` or `--keep` to resolve conflicts in either direction. Non-conflicting aliases always import cleanly.

After a successful import, reload your current shell to use the new aliases:

```bash
source ~/.bashrc
```

**Want a starter set?** Download this [premade profile](https://github.com/steffensmartinsen/config/blob/main/aka/profile.json) and import it:

```bash
curl -fsSL https://raw.githubusercontent.com/steffensmartinsen/config/main/aka/profile.json -o profile.json
aka import profile.json
```

## Updating

Check your installed version:

​```bash
aka --version
​```

Update to the latest release:

​```bash
aka update
​```

`aka update` checks the latest published release, and if a newer version exists, downloads and installs it over your current binary. If you're already on the latest (or ahead, during local development), it does nothing.

## How it works

`aka` stores your aliases as JSON at `~/.config/aka/aliases.json` and generates a bash script alongside it:

```
~/.config/aka/
├── aliases.json   # source of truth (edit via aka, not by hand)
└── aliases.sh     # generated — sourced by your ~/.bashrc
```

Every `add` or `remove` rewrites `aliases.sh`. Your shell reads that generated file, so your aliases live in one place and stay in sync.

A **profile** (from `aka export`) is just a copy of this store's JSON structure, so it's portable and safe to commit to a dotfiles repo. `aka import` reads one back in.

## Uninstall

```bash
rm ~/.local/bin/aka        # the binary
rm -rf ~/.config/aka       # aliases and generated files
```

Optionally remove the source line from your `~/.bashrc`.

## License

MIT
