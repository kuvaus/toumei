// src/platforms/freedesktop.rs

use zbus::proxy;
use gio::prelude::*;
use crate::{Error, Mode};

const GNOME_SHELL_SCHEMA: &str = "org.gnome.shell";
const TRANSPARENT_EXTENSIONS: &[&str] = &[
    "transparent-top-bar@zhanghai.me",
    "transparent-top-bar@ftpix.com",
];

const PORTAL_DESTINATION: &str = "org.freedesktop.portal.Desktop";
const APPEARANCE: &str = "org.freedesktop.appearance";
const TRANSPARENCY_KEY: &str = "transparency-enabled";

#[proxy(
    interface = "org.freedesktop.portal.Settings",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait XdgPortalSettings {
    fn read_one(&self, namespace: &str, key: &str) -> zbus::Result<zvariant::OwnedValue>;
}

// xdg
fn try_xdg_portal() -> Result<Mode, Error> {
    let conn = zbus::blocking::Connection::session()
        .map_err(|e| Error::XdgDesktopPortal(e.to_string()))?;

    let portal = XdgPortalSettingsProxyBlocking::new(&conn, PORTAL_DESTINATION)
        .map_err(|e| Error::XdgDesktopPortal(e.to_string()))?;

    match portal.read_one(APPEARANCE, TRANSPARENCY_KEY) {
        Ok(value) => match TryInto::<bool>::try_into(value) {
            Ok(enabled) => Ok(bool::from(enabled).into()),
            Err(_) => Ok(Mode::Opaque),
        },
        Err(_) => Ok(Mode::Opaque),
    }
}

pub fn detect_tray_transparency() -> Result<Mode, Error> {
    if has_transparency_extension() {
        return Ok(Mode::Transparent);
    }

    try_xdg_portal().or(Ok(Mode::Opaque))
}

// gnome
fn has_transparency_extension() -> bool {
    // Check if schema exists first
    let schema_source = match gio::SettingsSchemaSource::default() {
        Some(source) => source,
        None => return false,
    };

    // Verify the schema contains the key
    let schema = match schema_source.lookup(GNOME_SHELL_SCHEMA, true) {
        Some(s) => s,
        None => return false,
    };

    if !schema.has_key("enabled-extensions") {
        return false;
    }

    // Now safely access the settings
    let settings = gio::Settings::new(GNOME_SHELL_SCHEMA);
    let extensions = settings.get::<Vec<String>>("enabled-extensions");

    extensions.iter().any(|ext| TRANSPARENT_EXTENSIONS.contains(&ext.as_str()))
}