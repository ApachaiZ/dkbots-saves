//! Cards: turns each command's raw JSON into a `{"card": …}` object that
//! the host renders as a standard Discord embed.

use serde_json::{json, Value};

pub fn cardify(cmd: &str, raw: &str) -> String {
    let parsed: Value = serde_json::from_str(raw).unwrap_or_else(|_| json!({}));
    if let Some(error) = parsed.get("error").and_then(|v| v.as_str()) {
        return card(&format!("❌ {}", title(cmd)), error, "bad", &[]);
    }
    match cmd {
        "backup" => backup_card(&parsed),
        "list" => list_card(&parsed),
        "restore" => restore_card(&parsed),
        _ => card(&title(cmd), &parsed.to_string(), "info", &[]),
    }
}

fn title(cmd: &str) -> String {
    match cmd {
        "backup" => "💾 Sauvegarde du monde".into(),
        "list" => "🗄️ Archives".into(),
        "restore" => "♻️ Restauration".into(),
        _ => "💾 Sauvegardes".into(),
    }
}

fn card(title: &str, description: &str, tone: &str, fields: &[(String, String, bool)]) -> String {
    let mut card = serde_json::Map::new();
    card.insert("title".into(), json!(title));
    card.insert("description".into(), json!(description));
    card.insert("tone".into(), json!(tone));
    let fields: Vec<Value> = fields
        .iter()
        .map(|(name, value, inline)| {
            json!({ "name": name, "value": value, "inline": inline })
        })
        .collect();
    card.insert("fields".into(), json!(fields));
    card.insert("footer".into(), json!("Saves • DakkaBot"));
    json!({ "card": card }).to_string()
}

fn f_str(v: &Value, key: &str) -> String {
    v.get(key).and_then(|x| x.as_str()).unwrap_or("").to_string()
}

fn human_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let b = bytes as f64;
    if b >= GB {
        format!("{:.1} Go", b / GB)
    } else if b >= MB {
        format!("{:.1} Mo", b / MB)
    } else if b >= KB {
        format!("{:.1} Ko", b / KB)
    } else {
        format!("{bytes} o")
    }
}

fn backup_card(v: &Value) -> String {
    let archive = f_str(v, "archive");
    let name = archive.rsplit('/').next().unwrap_or(&archive);
    card(
        &title("backup"),
        &format!("✅ Archive créée : `{name}`\n📁 {archive}"),
        "good",
        &[],
    )
}

fn list_card(v: &Value) -> String {
    let list = v.get("backups").and_then(|b| b.as_array()).cloned().unwrap_or_default();
    if list.is_empty() {
        return card(&title("list"), "Aucune archive pour le moment.", "info", &[]);
    }
    let lines: Vec<String> = list
        .iter()
        .filter_map(|b| {
            let name = f_str(b, "archive");
            let size = b.get("size").and_then(|s| s.as_u64()).unwrap_or(0);
            Some(format!("🗜️ {name} · {}", human_size(size)))
        })
        .collect();
    card(
        &format!("🗄️ Archives ({})", list.len()),
        &lines.join("\n"),
        "good",
        &[],
    )
}

fn restore_card(v: &Value) -> String {
    let restored = f_str(v, "restored");
    card(
        &title("restore"),
        &format!("✅ Monde restauré depuis `{restored}`.\n⚠️ Redémarre le service de jeu si ce n'est pas déjà fait."),
        "good",
        &[],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backup_card_shows_archive() {
        let out = cardify("backup", r#"{"ok":true,"archive":"/home/steam/saves-backups/saves-2026.tar"}"#);
        let v: Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["card"]["title"], "💾 Sauvegarde du monde");
        assert!(v["card"]["description"]
            .as_str()
            .unwrap()
            .contains("saves-2026.tar"));
    }

    #[test]
    fn list_card_formats_sizes() {
        let out = cardify(
            "list",
            r#"{"backups":[{"archive":"a.tar","size":1048576},{"archive":"b.tar","size":2048}]}"#,
        );
        let v: Value = serde_json::from_str(&out).unwrap();
        let d = v["card"]["description"].as_str().unwrap();
        assert!(d.contains("1.0 Mo") && d.contains("2.0 Ko"));
    }

    #[test]
    fn error_becomes_bad_card() {
        let out = cardify("backup", r#"{"error":"DKBOT_SAVES_PATH non défini"}"#);
        let v: Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["card"]["tone"], "bad");
    }

    #[test]
    fn human_sizes() {
        assert_eq!(human_size(500), "500 o");
        assert_eq!(human_size(2048), "2.0 Ko");
        assert_eq!(human_size(5 * 1024 * 1024), "5.0 Mo");
    }
}
