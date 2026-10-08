use tauri::Window;

pub fn minimize(window: &Window) {
    let _ = window.minimize();
}

pub fn maximize_or_unmaximize(window: &Window) {
    if let Ok(is_max) = window.is_maximized() {
        if is_max {
            let _ = window.unmaximize();
        } else {
            let _ = window.maximize();
        }
    }
}

pub fn close(window: &Window) {
    let _ = window.close();
}

pub fn start_dragging(window: &Window) {
    let _ = window.start_dragging();
}
