# 💾 dkbots-saves

Plugin **saves** pour [dkbot-base](https://github.com/ApachaiZ/dkbot-base)
(ABI `dkbot-host-api` v1) — archives du monde de jeu, **emplacement
configurable**.

## Installation

```bash
dkbot plugin install saves
```

Puis `/plugin saves <commande>` (owner).

## Configuration (variables d'environnement du processus hôte)

| Variable | Rôle |
|---|---|
| `DKBOT_SAVES_PATH` | dossier source des sauvegardes (obligatoire) |
| `DKBOT_SAVES_BACKUP_DIR` | **emplacement des archives** (défaut `<SAVES_PATH>-backups`) |
| `DKBOT_SAVES_KEEP` | rétention (défaut 20) |

## Commandes

- `backup` : archive `tar.gz` horodatée UTC (écriture atomique) + purge.
- `list` : archives existantes, les plus récentes d'abord.
- `restore` : restaure la dernière archive (⚠️ service arrêté requis — le
  plugin ne pilote pas systemd).

Sans shell : `tar` reçoit des arguments séparés, chemins absolus validés.

Licence : MIT.
