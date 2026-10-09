use tauri::Window;

pub fn minimize(#[allow(unused_variables)] window: &Window) {
    #[cfg(not(target_os = "android"))]
    let _ = window.minimize();
}

pub fn maximize_or_unmaximize(#[allow(unused_variables)] window: &Window) {
    #[cfg(not(target_os = "android"))]
    if let Ok(is_max) = window.is_maximized() {
        if is_max {
            let _ = window.unmaximize();
        } else {
            let _ = window.maximize();
        }
    }
}

pub fn close(#[allow(unused_variables)] window: &Window) {
    #[cfg(not(target_os = "android"))]
    let _ = window.close();
}

pub fn start_dragging(#[allow(unused_variables)] window: &Window) {
    #[cfg(not(target_os = "android"))]
    let _ = window.start_dragging();
}
