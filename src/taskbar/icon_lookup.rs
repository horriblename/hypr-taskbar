use std::fs;
use std::path::PathBuf;

use super::DEFAULT_ICON;
use freedesktop_desktop_entry::{self, default_paths, DesktopEntry};

const ICON_SIZE: u16 = 24;

/// Guess the icon path of the given app class (a.k.a. app_id or window class), or `DEFAULT_ICON`
/// if no suitable icon is found.
pub fn guess_icon(app_class: &str) -> PathBuf {
    if let Some(desktop_file) = guess_desktop_file(app_class) {
        if let Some(icon_name) = get_icon_from_desktop_entry(&desktop_file) {
            if let Some(found_icon) = lookup_icon_path(&icon_name, ICON_SIZE) {
                return found_icon;
            };
        }
    };

    PathBuf::from(DEFAULT_ICON)
}

// follows the same strategy as waybar
// https://github.com/Alexays/Waybar/tree/master/src/modules/wlr/taskbar.cpp
fn guess_desktop_file(app_class: &str) -> Option<PathBuf> {
    let mut desktop_files = freedesktop_desktop_entry::Iter::new(default_paths());

    // find a desktop file named "app_name.desktop"
    let mut match_desktop_file = |app_name: &str| {
        desktop_files.find(|fpath| {
            if let Some(name) = fpath.file_stem() {
                if let Some(name) = name.to_str() {
                    return name == app_name;
                }
            };
            false
        })
    };

    if let Some(desktop) = match_desktop_file(app_class) {
        return Some(desktop);
    }

    if let Some(desktop) = match_desktop_file(&app_class.to_lowercase()) {
        return Some(desktop);
    }

    // "org.domain.app_name"
    if let Some(start) = app_class.rfind('.').and_then(|n| Some(n + 1)) {
        if let Some(desktop) = match_desktop_file(&app_class[start..]) {
            return Some(desktop);
        }
    }

    // "app_name-blah-blah-blah"
    if let Some(stop) = app_class.find('-') {
        if let Some(desktop) = match_desktop_file(&app_class[..stop]) {
            return Some(desktop);
        }
    }

    None
}

/// Look up the icon name in a given `desktop_file`
fn get_icon_from_desktop_entry(desktop_file: &PathBuf) -> Option<String> {
    if let Ok(bytes) = fs::read_to_string(desktop_file) {
        if let Ok(entry) = DesktopEntry::decode(desktop_file, &bytes) {
            return Some(entry.icon()?.to_string());
        }
    }

    None
}

/// Look up the icon path
fn lookup_icon_path(icon_name: &str, size: u16) -> Option<PathBuf> {
    freedesktop_icons::lookup(icon_name)
        .with_size(size)
        .with_cache()
        .find()
}
