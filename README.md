<div align="center">

# 💾 dkbots-saves

### World saves plugin for [dkbot-base](https://github.com/ApachaiZ/dkbot-base) — ABI `dkbot-host-api` v1

[![License](https://img.shields.io/badge/licence-MIT-blue)](#-license)
[![ABI](https://img.shields.io/badge/ABI-dkbot--host--api%20v1-purple)](https://github.com/ApachaiZ/dkbot-base)

Timed `tar.gz` archives of your game world from Discord — **archive location
fully configurable**, atomic writes, retention pruning, owner-only.

> 🌐 Languages: 🇫🇷 [Français](README.fr.md) · 🇩🇪 [Deutsch](README.de.md) · 🇪🇸 [Español](README.es.md)

</div>

---

## 📖 Contents

1. [📦 Install](#-install)
2. [⚙️ Configuration](#️-configuration)
3. [🎮 Commands](#-commands)
4. [🔐 Safety](#-safety)
5. [🔨 Build & tests](#-build--tests)
6. [📄 License](#-license)

---

## 📦 Install

```bash
dkbot plugin install saves   # downloads the .so, SHA-256 verified
```

The plugin's manifest becomes the real `/saves` Discord command group at host
startup.

```text
   ┌──────────────┐        ┌──────────────────────────────┐
   │  Discord     │───────►│  dkbots-saves (.so)          │
   │  /saves …    │        │  tar -czf … (atomic)         │
   └──────────────┘        └───────┬──────────────────────┘
                                    │
                          ┌─────────▼─────────┐
                          │  saves dir  ──► backups dir
                          │  (game world)     (configurable, pruned)
                          └───────────────────┘
```

---

## ⚙️ Configuration

Environment variables of the host process (the bot's `.env`):

| Variable | Role |
|---|---|
| `DKBOT_SAVES_PATH` | Source saves directory (required, absolute) |
| `DKBOT_SAVES_BACKUP_DIR` | **Where archives go** — default `<SAVES_PATH>-backups` |
| `DKBOT_SAVES_KEEP` | Retention — default 20 archives |

---

## 🎮 Commands

| Command | Role |
|---|---|
| `/saves backup` | 💾 Archive the world now — UTC-timestamped `tar.gz`, atomic write, prune |
| `/saves list` | 🗄️ Existing archives, newest first, human sizes |
| `/saves restore` | ♻️ Restore the latest archive into the saves dir (⚠️ game service must be stopped — the plugin does not drive systemd) |

All commands are **owner-only** and answered as formatted embed cards.

---

## 🔐 Safety

- 🛡️ Owner-only by design — backups are sensitive operations.
- ✍️ Atomic writes: archive to a temp file, then `rename` — a crash can never
  leave a half-written archive.
- 🧰 No shell: `tar` receives separate arguments, paths are validated
  (absolute paths required).

---

## 🔨 Build & tests

```bash
cargo build --release   # → target/release/libdkbots_saves.so
cargo test              # 10 unit tests (cards, sizes, owner-only, commands)
```

---

## 📄 License

MIT.
