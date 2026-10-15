<div align="center">

# 💾 dkbots-saves

### Welt-Save-Plugin für [dkbot-base](https://github.com/ApachaiZ/dkbot-base) — ABI `dkbot-host-api` v1

[![Lizenz](https://img.shields.io/badge/lizenz-MIT-blue)](#-lizenz)
[![ABI](https://img.shields.io/badge/ABI-dkbot--host--api%20v1-purple)](https://github.com/ApachaiZ/dkbot-base)

Zeitgestempelte `tar.gz`-Archive deiner Spielwelt aus Discord —
**Archivort vollständig konfigurierbar**, atomare Schreibvorgänge,
Aufbewahrung mit Bereinigung, nur Owner.

> 🌐 Sprachen: 🇬🇧 [English](README.md) · 🇫🇷 [Français](README.fr.md) · 🇩🇪 Deutsch · 🇪🇸 [Español](README.es.md)

</div>

---

## 📖 Inhalt

1. [📦 Installation](#-installation)
2. [⚙️ Konfiguration](#️-konfiguration)
3. [🎮 Befehle](#-befehle)
4. [🔐 Sicherheit](#-sicherheit)
5. [🔨 Build & Tests](#-build--tests)
6. [📄 Lizenz](#-lizenz)

---

## 📦 Installation

```bash
dkbot plugin install saves   # lädt das .so, SHA-256-geprüft
```

Das Plugin-Manifest wird beim Host-Start zur echten `/saves`-Befehlsgruppe
in Discord.

```text
   ┌──────────────┐        ┌──────────────────────────────┐
   │  Discord     │───────►│  dkbots-saves (.so)          │
   │  /saves …    │        │  tar -czf … (atomar)         │
   └──────────────┘        └───────┬──────────────────────┘
                                    │
                          ┌─────────▼─────────┐
                          │  Saves-Verz. ────► Archiv-Verz.
                          │  (Spielwelt)       (konfigurierbar, bereinigt)
                          └───────────────────┘
```

---

## ⚙️ Konfiguration

Umgebungsvariablen des Host-Prozesses (die `.env` des Bots):

| Variable | Rolle |
|---|---|
| `DKBOT_SAVES_PATH` | Quellverzeichnis der Saves (erforderlich, absolut) |
| `DKBOT_SAVES_BACKUP_DIR` | **Wohin die Archive gehen** — Standard `<SAVES_PATH>-backups` |
| `DKBOT_SAVES_KEEP` | Aufbewahrung — Standard 20 Archive |

---

## 🎮 Befehle

| Befehl | Rolle |
|---|---|
| `/saves backup` | 💾 Welt jetzt archivieren — UTC-zeitgestempeltes `tar.gz`, atomares Schreiben, Bereinigung |
| `/saves list` | 🗄️ Vorhandene Archive, neueste zuerst, lesbare Größen |
| `/saves restore` | ♻️ Neuestes Archiv ins Saves-Verzeichnis zurückspielen (⚠️ Spiel-Dienst muss gestoppt sein — das Plugin steuert kein systemd) |

Alle Befehle sind **nur für den Owner** und werden als formatierte
Embed-Karten beantwortet.

---

## 🔐 Sicherheit

- 🛡️ Nur Owner, per Design — Backups sind sensible Operationen.
- ✍️ Atomare Schreibvorgänge: Archiv in eine Temp-Datei, dann `rename` — ein
  Absturz kann nie ein halb geschriebenes Archiv hinterlassen.
- 🧰 Keine Shell: `tar` erhält separate Argumente, Pfade werden validiert
  (absolute Pfade erforderlich).

---

## 🔨 Build & Tests

```bash
cargo build --release   # → target/release/libdkbots_saves.so
cargo test              # 10 Unit-Tests (Karten, Größen, Owner-only, Befehle)
```

---

## 📄 Lizenz

MIT.
