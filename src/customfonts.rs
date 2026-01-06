//! customfonts
//! Set the font
use eframe::egui;
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
/// Set the font to use the system fonts
/// # Arguments
///
/// * `ctx`: &egui::Context - egui Context
///
/// # Examples
/// ```rust,no_run
///     setup_custom_fonts(&cc.egui_ctx);
/// ```
///
pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    let source = SystemSource::new();
    let family_names = [
        FamilyName::Title("PingFang TC".to_string()),
        FamilyName::Title("Microsoft JhengHei".to_string()),
        FamilyName::SansSerif,
    ];

    if let Ok(handle) = source.select_best_match(&family_names, &Properties::new()) {
        if let Ok(font_data) = handle.load() {
            if let Some(font_arc) = font_data.copy_font_data() {
                let font_vec = font_arc.as_ref().clone();

                // insert font data
                fonts.font_data.insert(
                    "sys_cjk".to_owned(),
                    egui::FontData::from_owned(font_vec).into(),
                );

                // set font families
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "sys_cjk".to_owned());

                // set mono fonts for backup fonts
                let mono = fonts
                    .families
                    .get_mut(&egui::FontFamily::Monospace)
                    .unwrap();
                mono.push("sys_cjk".to_owned());
            }
        }
    }

    ctx.set_fonts(fonts);
}
