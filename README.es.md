<div align="center">

# 💾 dkbots-saves

### Plugin de saves del mundo para [dkbot-base](https://github.com/ApachaiZ/dkbot-base) — ABI `dkbot-host-api` v1

[![Licencia](https://img.shields.io/badge/licencia-MIT-blue)](#-licencia)
[![ABI](https://img.shields.io/badge/ABI-dkbot--host--api%20v1-purple)](https://github.com/ApachaiZ/dkbot-base)

Archivos `tar.gz` con marca de tiempo de tu mundo de juego desde Discord —
**ubicación del archivo totalmente configurable**, escrituras atómicas,
retención con poda, solo propietario.

> 🌐 Idiomas: 🇬🇧 [English](README.md) · 🇫🇷 [Français](README.fr.md) · 🇩🇪 [Deutsch](README.de.md) · 🇪🇸 Español

</div>

---

## 📖 Índice

1. [📦 Instalación](#-instalación)
2. [⚙️ Configuración](#️-configuración)
3. [🎮 Comandos](#-comandos)
4. [🔐 Seguridad](#-seguridad)
5. [🔨 Build y tests](#-build-y-tests)
6. [📄 Licencia](#-licencia)

---

## 📦 Instalación

```bash
dkbot plugin install saves   # descarga el .so, SHA-256 verificado
```

El manifiesto del plugin se convierte en el grupo real de comandos `/saves`
de Discord al arrancar el host.

```text
   ┌──────────────┐        ┌──────────────────────────────┐
   │  Discord     │───────►│  dkbots-saves (.so)          │
   │  /saves …    │        │  tar -czf … (atómico)        │
   └──────────────┘        └───────┬──────────────────────┘
                                    │
                          ┌─────────▼─────────┐
                          │  dir de saves ──► dir de archivos
                          │  (mundo de juego)  (configurable, podado)
                          └───────────────────┘
```

---

## ⚙️ Configuración

Variables de entorno del proceso host (el `.env` del bot):

| Variable | Rol |
|---|---|
| `DKBOT_SAVES_PATH` | Directorio fuente de los saves (obligatorio, absoluto) |
| `DKBOT_SAVES_BACKUP_DIR` | **Dónde van los archivos** — por defecto `<SAVES_PATH>-backups` |
| `DKBOT_SAVES_KEEP` | Retención — por defecto 20 archivos |

---

## 🎮 Comandos

| Comando | Rol |
|---|---|
| `/saves backup` | 💾 Archiva el mundo ahora — `tar.gz` con marca UTC, escritura atómica, poda |
| `/saves list` | 🗄️ Archivos existentes, más recientes primero, tamaños legibles |
| `/saves restore` | ♻️ Restaura el último archivo en el directorio de saves (⚠️ el servicio del juego debe estar detenido — el plugin no controla systemd) |

Todos los comandos son **solo para el propietario** y se responden como
tarjetas Discord formateadas.

---

## 🔐 Seguridad

- 🛡️ Solo propietario, por diseño — los backups son operaciones sensibles.
- ✍️ Escrituras atómicas: archivo a un temporal, luego `rename` — un cuelgue
  jamás puede dejar un archivo a medio escribir.
- 🧰 Sin shell: `tar` recibe argumentos separados, las rutas se validan
  (rutas absolutas requeridas).

---

## 🔨 Build y tests

```bash
cargo build --release   # → target/release/libdkbots_saves.so
cargo test              # 10 tests unitarios (tarjetas, tamaños, solo-propietario, comandos)
```

---

## 📄 Licencia

MIT.
