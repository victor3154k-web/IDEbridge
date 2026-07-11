use std::process::Command;

#[tauri::command]
pub async fn reveal_in_file_explorer(path: &str) -> Result<(), String> {
    // Se estiver rodando no Android, apenas ignore o comando de abrir janela do PC e retorne sucesso
    if cfg!(target_os = "android") {
        return Ok(());
    }

    if cfg!(target_os = "windows") {
        Command::new("explorer")
            .args(["/select,", &path.replace("/", "\\")]) // The comma after select is not a typo
            .spawn()
            .map_err(|e| e.to_string())?;
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .args(["-R", path])
            .spawn()
            .map_err(|e| e.to_string())?;
    } else {
        Command::new("xdg-open")
            .args([path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
