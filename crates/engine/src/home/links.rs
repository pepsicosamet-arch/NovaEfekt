//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! header's Discord button).

pub const APP_NAME: &str = "NOVA Efekt";
pub const APP_SLUG: &str = "effectcraft";
pub const DISCORD: &str = "https://www.instagram.com/xsametsahinx";
pub const WEBSITE: &str = "https://getartcraft.com";
pub const APP_PAGE: &str = "https://github.com/pepsicosamet-arch/NovaEfekt";
pub const GITHUB: &str = "https://github.com/pepsicosamet-arch/NovaEfekt";
pub const ISSUES: &str = "https://github.com/pepsicosamet-arch/NovaEfekt/issues";

/// Sibling apps of the ArtCraft family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("NOVA Foto", "photocraft"),
    ("NOVA Vektör", "vectorcraft"),
    ("NOVA Film", "filmcraft"),
    ("NOVA Işık", "lightcraft"),
    ("NOVA PDF", "pdfcraft"),
    ("NOVA Tasarım", "designcraft"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/storytold/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://getartcraft.com/apps/{slug}")
}
