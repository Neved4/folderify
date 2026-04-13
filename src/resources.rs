use std::path::PathBuf;

use include_dir::{include_dir, Dir};

use crate::{
    args::{Badge, ColorScheme, FolderColor, FolderStyle},
    icon_conversion::IconResolution,
};

static RESOURCES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/resources");

pub struct IconInputs {
    pub folder_style: FolderStyle,
    pub color_scheme: ColorScheme,
    pub folder_color: FolderColor,
    pub resolution: IconResolution,
    pub empty_folder: bool,
}

fn tahoe_iconset_name(folder_color: FolderColor, empty_folder: bool) -> String {
    let folder_state = if empty_folder { "empty" } else { "non-empty" };
    if folder_color == FolderColor::Multicolor {
        return format!("GenericFolderIcon.{folder_state}.Tahoe.iconset");
    }

    format!(
        "GenericFolderIcon.{folder_state}.Tahoe.{}.iconset",
        folder_color.as_str()
    )
}

pub fn get_folder_icon(icon_inputs: &IconInputs) -> &'static [u8] {
    let mut path = PathBuf::new();
    path.push("folders");
    match (icon_inputs.color_scheme, icon_inputs.folder_style) {
        (ColorScheme::Light, FolderStyle::BigSur) => {
            path.push("GenericFolderIcon.BigSur.iconset");
        }
        (ColorScheme::Dark, FolderStyle::BigSur) => {
            path.push("GenericFolderIcon.BigSur.dark.iconset");
        }
        (_, FolderStyle::Tahoe) => {
            path.push(tahoe_iconset_name(
                icon_inputs.folder_color,
                icon_inputs.empty_folder,
            ));
        }
    }
    path.push(icon_inputs.resolution.icon_file());
    RESOURCES_DIR.get_file(&path).unwrap().contents()
}

pub fn get_badge_icon(badge: Badge, resolution: &IconResolution) -> &'static [u8] {
    let mut path = PathBuf::new();
    path.push("badges");
    path.push(match badge {
        Badge::Alias => "AliasBadgeIcon.iconset",
        Badge::Locked => "LockedBadgeIcon.iconset",
    });
    path.push(resolution.icon_file());
    RESOURCES_DIR.get_file(&path).unwrap().contents()
}
