use std::sync::{Arc, Mutex};
use notify::RecommendedWatcher;
use tauri::Manager;

mod fs_extra;
mod watch;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let watcher_mutex: Arc<Mutex<Option<RecommendedWatcher>>> = Arc::new(Mutex::new(None));
    let app_watcher_mutex = watcher_mutex.clone();

    // Criamos a base do construtor padrão do Tauri
    let mut builder = tauri::Builder::default();

    // Se estiver compilando para Desktop (PC), adicionamos o plugin do Updater
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_updater::Builder::new().build());
    }

    // Adicionamos os demais plugins e configurações que rodam em todas as plataformas
    builder
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(watcher_mutex)
        .invoke_handler(tauri::generate_handler![
            watch::watch,
            watch::unwatch,
            fs_extra::reveal_in_file_explorer,
        ])
        .setup(move |app| {
            #[cfg(debug_assertions)]
            app.get_webview_window("main").unwrap().open_devtools();

            let mut watcher = app_watcher_mutex.lock().unwrap();
            *watcher = watch::setup_watcher(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
