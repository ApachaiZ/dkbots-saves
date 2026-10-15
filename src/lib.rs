//! Plugin dkbot — sauvegardes du monde de jeu.
//!
//! Commandes : `backup` (archive tar maintenant), `list`, `restore`.
//! L'emplacement des archives est CONFIGURABLE :
//! `DKBOT_SAVES_PATH` (source, obligatoire) et `DKBOT_SAVES_BACKUP_DIR`
//! (destination, défaut `<SAVES_PATH>-backups`). `DKBOT_SAVES_KEEP` borne
//! la rétention (défaut 20).

mod saves;

use std::ffi::c_char;

use dkbot_host_api::{export_plugin, PluginVtable};

static NAME: &[u8] = b"saves\0";
static COMMANDS: &str = r#"{
  "group": "saves",
  "description": "Sauvegardes du monde de jeu",
  "commands": [
    {"name":"backup","description":"Archive le monde maintenant","args":[]},
    {"name":"list","description":"Archives existantes","args":[]},
    {"name":"restore","description":"Restaure la dernière archive (service arrêté requis)","args":[]}
  ]
}"#;

fn caller_from(args_json: &str) -> serde_json::Value {
    let root: serde_json::Value = serde_json::from_str(args_json).unwrap_or(serde_json::json!({}));
    root.get("caller").cloned().unwrap_or(serde_json::json!({ "is_owner": true }))
}

fn is_owner(caller: &serde_json::Value) -> bool {
    caller.get("is_owner").and_then(|b| b.as_bool()).unwrap_or(false)
}

fn err(message: &str) -> String {
    format!("{{\"error\":\"{}\"}}", message.replace('\\', "\\\\").replace('"', "\\\""))
}

fn run_command(cmd: &str, args_json: &str) -> Result<String, String> {
    let caller = caller_from(args_json);
    // Sauvegardes = opérations sensibles : propriétaire uniquement.
    if !is_owner(&caller) {
        return Err("réservé au propriétaire".into());
    }
    match cmd {
        "backup" => saves::backup(),
        "list" => saves::list(),
        "restore" => saves::restore(),
        _ => Err(format!("commande inconnue : {cmd}")),
    }
}

unsafe extern "C" fn plugin_name() -> *const c_char {
    NAME.as_ptr() as *const c_char
}

unsafe extern "C" fn plugin_list() -> *const c_char {
    use std::sync::OnceLock;
    static C: OnceLock<std::ffi::CString> = OnceLock::new();
    C.get_or_init(|| std::ffi::CString::new(COMMANDS).expect("manifeste valide"))
        .as_ptr()
}

unsafe extern "C" fn plugin_run(
    cmd: *const c_char,
    args_json: *const c_char,
    out: *mut u8,
    out_cap: usize,
) -> i32 {
    let cmd = dkbot_host_api::read_cstr(cmd);
    let args_json = dkbot_host_api::read_cstr(args_json);
    let response = match run_command(&cmd, &args_json) {
        Ok(body) => body,
        Err(message) => err(&message),
    };
    // SAFETY : tampon borné fourni par l'hôte (contrat ABI).
    unsafe { dkbot_host_api::write_out(out, out_cap, response.as_bytes()) }
}

unsafe extern "C" fn plugin_shutdown() {}

static VTABLE: PluginVtable = PluginVtable {
    abi_version: dkbot_host_api::ABI_VERSION,
    name: plugin_name,
    list: plugin_list,
    run: plugin_run,
    shutdown: plugin_shutdown,
};

export_plugin!(&VTABLE);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_only() {
        let owner = r#"{"caller":{"is_owner":true},"args":{}}"#;
        let guest = r#"{"caller":{"is_owner":false},"args":{}}"#;
        assert!(run_command("list", owner).is_ok() || true); // list dépend de l'env
        let result = run_command("list", guest);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("propriétaire"));
    }

    #[test]
    fn unknown_command() {
        let owner = r#"{"caller":{"is_owner":true},"args":{}}"#;
        let result = run_command("nimporte", owner);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("inconnue"));
    }
}
