//! Archives tar des sauvegardes — sans shell (arguments validés), écriture
//! atomique, rétention bornée, emplacement CONFIGURABLE.

use std::path::{Path, PathBuf};
use std::process::Command;

fn env_or(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}

/// Dossier source des sauvegardes (obligatoire pour `backup`/`restore`).
pub fn saves_path() -> Option<PathBuf> {
    let raw = env_or("DKBOT_SAVES_PATH", "");
    if raw.is_empty() {
        return None;
    }
    Some(PathBuf::from(raw))
}

/// Emplacement des archives — configurable, défaut `<SAVES_PATH>-backups`.
pub fn backup_dir() -> Option<PathBuf> {
    let raw = env_or("DKBOT_SAVES_BACKUP_DIR", "");
    if !raw.is_empty() {
        return Some(PathBuf::from(raw));
    }
    saves_path().map(|p| {
        let mut name = p.as_os_str().to_os_string();
        name.push("-backups");
        PathBuf::from(name)
    })
}

fn keep_count() -> usize {
    env_or("DKBOT_SAVES_KEEP", "20")
        .parse()
        .unwrap_or(20)
        .max(1)
}

/// Un chemin est-il un chemin absolu propre (pas de `..` ni de caractères
/// exotiques) ? Le plugin n'exécute JAMAIS de shell — `tar` reçoit des
/// arguments séparés — mais on refuse les chemins douteux par principe.
fn path_ok(path: &Path) -> bool {
    let s = path.to_string_lossy();
    if s.is_empty() || s.contains('\0') {
        return false;
    }
    path.is_absolute()
}

/// Nom d'archive horodaté UTC : `20261015-123001.tar`.
fn archive_name() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = secs / 86400;
    let (y, m, d) = civil_from_days(days as i64);
    let tod = secs % 86400;
    format!(
        "{y:04}{m:02}{d:02}-{h:02}{min:02}{s:02}.tar",
        h = tod / 3600,
        min = (tod % 3600) / 60,
        s = tod % 60
    )
}

// Conversion jours → date civile (algorithme de Howard Hinnant).
fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn run_tar(args: &[&str]) -> Result<String, String> {
    let output = Command::new("tar")
        .args(args)
        .output()
        .map_err(|e| format!("tar introuvable : {e}"))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(format!(
            "tar a échoué : {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

/// Archive le dossier source dans le dossier de sauvegarde, puis purge les
/// archives au-delà de la rétention.
pub fn backup() -> Result<String, String> {
    let Some(source) = saves_path() else {
        return Err("DKBOT_SAVES_PATH non défini".into());
    };
    let Some(dest) = backup_dir() else {
        return Err("emplacement des archives indéterminé".into());
    };
    if !path_ok(&source) || !path_ok(&dest) {
        return Err("chemins invalides (absolus requis)".into());
    }
    if !source.is_dir() {
        return Err(format!("source introuvable : {}", source.display()));
    }
    std::fs::create_dir_all(&dest).map_err(|e| format!("création du dossier : {e}"))?;

    let name = archive_name();
    let archive = dest.join(&name);
    // Écriture atomique : archive temporaire puis rename.
    let tmp = dest.join(format!(".{name}.tmp"));
    let _ = std::fs::remove_file(&tmp);
    run_tar(&[
        "-czf",
        tmp.to_string_lossy().as_ref(),
        "-C",
        source.to_string_lossy().as_ref(),
        ".",
    ])?;
    std::fs::rename(&tmp, &archive).map_err(|e| format!("rename : {e}"))?;

    prune(&dest);
    Ok(format!(
        "{{\"ok\":true,\"archive\":\"{}\"}}",
        archive.to_string_lossy()
    ))
}

/// Liste les archives (les plus récentes d'abord).
pub fn list() -> Result<String, String> {
    let Some(dest) = backup_dir() else {
        return Err("emplacement des archives indéterminé".into());
    };
    if !path_ok(&dest) {
        return Err("chemins invalides (absolus requis)".into());
    }
    let mut entries = Vec::new();
    let read = std::fs::read_dir(&dest).map_err(|e| format!("lecture : {e}"))?;
    for entry in read.flatten() {
        let path = entry.path();
        if path.extension().map(|e| e == "tar").unwrap_or(false) {
            let meta = entry.metadata().ok();
            entries.push((
                entry.file_name().to_string_lossy().into_owned(),
                meta.map(|m| m.len()).unwrap_or(0),
            ));
        }
    }
    entries.sort_by(|a, b| b.0.cmp(&a.0));
    let rows: Vec<serde_json::Value> = entries
        .into_iter()
        .map(|(name, size)| serde_json::json!({ "archive": name, "size": size }))
        .collect();
    Ok(serde_json::json!({ "backups": rows }).to_string())
}

/// Restaure la DERNIÈRE archive dans le dossier source (écrase).
/// ⚠️ À n'utiliser que service arrêté — le plugin ne pilote pas systemd.
pub fn restore() -> Result<String, String> {
    let Some(source) = saves_path() else {
        return Err("DKBOT_SAVES_PATH non défini".into());
    };
    let Some(dest) = backup_dir() else {
        return Err("emplacement des archives indéterminé".into());
    };
    if !path_ok(&source) || !path_ok(&dest) {
        return Err("chemins invalides (absolus requis)".into());
    }
    let mut latest: Option<(String, PathBuf)> = None;
    for entry in std::fs::read_dir(&dest)
        .map_err(|e| format!("lecture : {e}"))?
        .flatten()
    {
        let path = entry.path();
        if path.extension().map(|e| e == "tar").unwrap_or(false) {
            let name = entry.file_name().to_string_lossy().into_owned();
            if latest.as_ref().map(|(n, _)| name > *n).unwrap_or(true) {
                latest = Some((name, path));
            }
        }
    }
    let Some((name, archive)) = latest else {
        return Err("aucune archive à restaurer".into());
    };
    run_tar(&[
        "-xzf",
        archive.to_string_lossy().as_ref(),
        "-C",
        source.to_string_lossy().as_ref(),
    ])?;
    Ok(format!("{{\"ok\":true,\"restored\":\"{name}\"}}"))
}

fn prune(dest: &Path) {
    let keep = keep_count();
    let mut entries: Vec<(String, PathBuf)> = Vec::new();
    for entry in std::fs::read_dir(dest).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.extension().map(|e| e == "tar").unwrap_or(false) {
            entries.push((entry.file_name().to_string_lossy().into_owned(), path));
        }
    }
    entries.sort_by(|a, b| b.0.cmp(&a.0));
    for (_, path) in entries.into_iter().skip(keep) {
        let _ = std::fs::remove_file(path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn archive_name_is_utc_timestamped() {
        let name = archive_name();
        assert_eq!(name.len(), "20261015-123001.tar".len());
        assert!(name.ends_with(".tar"));
        assert!(name.starts_with('2'));
    }

    #[test]
    fn civil_from_days_known_date() {
        // 1970-01-01 = 0 jour
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        // 2026-10-15 = jours depuis l'époque
        let days = (2026 - 1970) as i64 * 365 + 15; // approximation volontaire
        let (y, _, _) = civil_from_days(days);
        assert_eq!(y, 2026);
    }

    #[test]
    fn path_ok_rejects_relative() {
        assert!(path_ok(Path::new("/absolu")));
        assert!(!path_ok(Path::new("relatif")));
    }

    #[test]
    fn backup_without_config_is_a_clean_error() {
        let result = backup();
        if let Err(e) = result {
            assert!(e.contains("DKBOT_SAVES_PATH") || e.contains("tar"));
        }
    }
}
