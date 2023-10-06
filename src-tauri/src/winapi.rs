#[cfg(target_os = "windows")]
use windows::UI::ViewManagement;

#[cfg(target_os = "windows")]
use windows::UI::ViewManagement::UIColorType;

#[cfg(target_os = "windows")]
pub async fn get_windows_accent_color() -> Result<[u8; 4], String> {
    let ui_self: Result<ViewManagement::UISettings, windows::core::Error> =
        ViewManagement::UISettings::new();
    if ui_self.is_err() {
        return Err(String::from("Cannot Instantiate UISettings"));
    }

    let color_res: Result<windows::UI::Color, windows::core::Error> =
        ui_self.unwrap().GetColorValue(UIColorType::Accent);

    if color_res.is_err() {
        return Err(String::from("Color Res is Error"));
    }
    let c: windows::UI::Color = color_res.unwrap();
    Ok([c.R, c.G, c.B, c.A])
}
