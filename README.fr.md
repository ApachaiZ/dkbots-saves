<div align="center">

# 💾 dkbots-saves

### Plugin de sauvegardes du monde pour [dkbot-base](https://github.com/ApachaiZ/dkbot-base) — ABI `dkbot-host-api` v1

[![Licence](https://img.shields.io/badge/licence-MIT-blue)](#-licence)
[![ABI](https://img.shields.io/badge/ABI-dkbot--host--api%20v1-purple)](https://github.com/ApachaiZ/dkbot-base)

Archives `tar.gz` horodatées de votre monde de jeu depuis Discord —
**emplacement d'archive entièrement configurable**, écritures atomiques,
rétention avec purge, propriétaire uniquement.

> 🌐 Langues : 🇬🇧 [English](README.md) · 🇫🇷 Français · 🇩🇪 [Deutsch](README.de.md) · 🇪🇸 [Español](README.es.md)

</div>

---

## 📖 Sommaire

1. [📦 Installation](#-installation)
2. [⚙️ Configuration](#️-configuration)
3. [🎮 Commandes](#-commandes)
4. [🔐 Sécurité](#-sécurité)
5. [🔨 Build & tests](#-build--tests)
6. [📄 Licence](#-licence)

---

## 📦 Installation

```bash
dkbot plugin install saves   # télécharge le .so, SHA-256 vérifié
```

Le manifeste du plugin devient le vrai groupe de commandes `/saves` Discord
au démarrage de l'hôte.

```text
   ┌──────────────┐        ┌──────────────────────────────┐
   │  Discord     │───────►│  dkbots-saves (.so)          │
   │  /saves …    │        │  tar -czf … (atomique)       │
   └──────────────┘        └───────┬──────────────────────┘
                                    │
                          ┌─────────▼─────────┐
                          │  dossier saves ──► dossier archives
                          │  (monde de jeu)    (configurable, purgé)
                          └───────────────────┘
```

---

## ⚙️ Configuration

Variables d'environnement du processus hôte (le `.env` du bot) :

| Variable | Rôle |
|---|---|
| `DKBOT_SAVES_PATH` | Dossier source des sauvegardes (obligatoire, absolu) |
| `DKBOT_SAVES_BACKUP_DIR` | **Où vont les archives** — défaut `<SAVES_PATH>-backups` |
| `DKBOT_SAVES_KEEP` | Rétention — défaut 20 archives |

---

## 🎮 Commandes

| Commande | Rôle |
|---|---|
| `/saves backup` | 💾 Archive le monde maintenant — `tar.gz` horodaté UTC, écriture atomique, purge |
| `/saves list` | 🗄️ Archives existantes, plus récentes d'abord, tailles lisibles |
| `/saves restore` | ♻️ Restaure la dernière archive dans le dossier saves (⚠️ le service de jeu doit être arrêté — le plugin ne pilote pas systemd) |

Toutes les commandes sont **réservées au propriétaire** et répondues en
cartes Discord formatées.

---

## 🔐 Sécurité

- 🛡️ Propriétaire uniquement, par conception — les sauvegardes sont des
  opérations sensibles.
- ✍️ Écritures atomiques : archive vers un fichier temporaire, puis `rename`
  — un crash ne peut jamais laisser une archive à moitié écrite.
- 🧰 Aucun shell : `tar` reçoit des arguments séparés, les chemins sont
  validés (chemins absolus requis).

---

## 🔨 Build & tests

```bash
cargo build --release   # → target/release/libdkbots_saves.so
cargo test              # 10 tests unitaires (cartes, tailles, owner-only, commandes)
```

---

## 📄 Licence

MIT.
