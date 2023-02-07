#[cfg(test)]
mod icon_lookup_test {
    #[test]
    fn test_icon_lookup() {
        use crate::taskbar::icon_lookup;

        assert_eq!(
            icon_lookup::lookup_icon("steam", 24),
            "/usr/share/icons/hicolor/24x24/apps/steam.png"
        );
    }
}
