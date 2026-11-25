use crate::models::AppTheme;
use egui::{
    Color32, Context, FontId, Frame, Margin, RichText, Rounding, Stroke, Style, SystemTheme,
    TextStyle, Ui, Vec2, ViewportCommand, Visuals,
};

pub struct Palette {
    pub bg_canvas: Color32,
    pub bg_sidebar: Color32,
    pub bg_card: Color32,
    pub bg_elevated: Color32,
    pub bg_hovered: Color32,
    pub border: Color32,
    pub text_primary: Color32,
    pub text_muted: Color32,
    pub accent: Color32,
    pub text_on_accent: Color32,
    pub accent_blue: Color32,
    pub success: Color32,
    pub danger: Color32,
    pub card_rounding: f32,
    pub control_rounding: f32,
    pub is_windows: bool,
}

impl Palette {
    pub fn for_theme(theme: AppTheme) -> Self {
        match theme {
            AppTheme::UbuntuDark => Self {
                bg_canvas: Color32::from_rgb(28, 28, 30),
                bg_sidebar: Color32::from_rgb(22, 22, 24),
                bg_card: Color32::from_rgb(38, 38, 42),
                bg_elevated: Color32::from_rgb(48, 48, 54),
                bg_hovered: Color32::from_rgb(60, 60, 68),
                border: Color32::from_rgb(62, 62, 68),
                text_primary: Color32::from_rgb(242, 242, 244),
                text_muted: Color32::from_rgb(154, 154, 162),
                accent: Color32::from_rgb(233, 84, 32), // Ubuntu Yaru Orange
                text_on_accent: Color32::WHITE,
                accent_blue: Color32::from_rgb(3, 169, 244), // Clockify Blue
                success: Color32::from_rgb(46, 204, 113),
                danger: Color32::from_rgb(231, 76, 60),
                card_rounding: 10.0,
                control_rounding: 6.0,
                is_windows: false,
            },
            AppTheme::UbuntuLight => Self {
                bg_canvas: Color32::from_rgb(246, 245, 244),
                bg_sidebar: Color32::from_rgb(236, 235, 233),
                bg_card: Color32::from_rgb(255, 255, 255),
                bg_elevated: Color32::from_rgb(242, 241, 240),
                bg_hovered: Color32::from_rgb(230, 229, 226),
                border: Color32::from_rgb(214, 212, 208),
                text_primary: Color32::from_rgb(36, 36, 38),
                text_muted: Color32::from_rgb(112, 112, 120),
                accent: Color32::from_rgb(233, 84, 32),
                text_on_accent: Color32::WHITE,
                accent_blue: Color32::from_rgb(2, 136, 209),
                success: Color32::from_rgb(39, 174, 96),
                danger: Color32::from_rgb(214, 58, 46),
                card_rounding: 10.0,
                control_rounding: 6.0,
                is_windows: false,
            },
            AppTheme::WindowsDark => Self {
                bg_canvas: Color32::from_rgb(32, 32, 32),      // #202020 Mica / Win11 Dark
                bg_sidebar: Color32::from_rgb(25, 25, 25),     // #191919
                bg_card: Color32::from_rgb(43, 43, 43),        // #2B2B2B
                bg_elevated: Color32::from_rgb(53, 53, 53),    // #353535
                bg_hovered: Color32::from_rgb(64, 64, 64),     // #404040
                border: Color32::from_rgb(66, 66, 66),         // #424242
                text_primary: Color32::from_rgb(245, 245, 245),// #F5F5F5
                text_muted: Color32::from_rgb(166, 166, 166),  // #A6A6A6
                accent: Color32::from_rgb(96, 205, 255),       // #60CDFF WinUI 3 Dark Accent
                text_on_accent: Color32::from_rgb(16, 16, 16), // #101010 TextOnAccent
                accent_blue: Color32::from_rgb(79, 209, 197),  // #4FD1C5 Fluent Dark Teal
                success: Color32::from_rgb(108, 203, 95),      // #6CCB5F WinUI 3 Dark Success
                danger: Color32::from_rgb(231, 72, 86),        // #E74856 Windows 11 Red
                card_rounding: 8.0,
                control_rounding: 4.0,
                is_windows: true,
            },
            AppTheme::WindowsLight => Self {
                bg_canvas: Color32::from_rgb(243, 243, 243),   // #F3F3F3 Win11 Light
                bg_sidebar: Color32::from_rgb(234, 238, 243),  // #EAEEF3 Cool Mica tint
                bg_card: Color32::from_rgb(255, 255, 255),     // #FFFFFF
                bg_elevated: Color32::from_rgb(249, 249, 249), // #F9F9F9
                bg_hovered: Color32::from_rgb(238, 241, 245),  // #EEF1F5
                border: Color32::from_rgb(225, 228, 232),      // #E1E4E8
                text_primary: Color32::from_rgb(26, 26, 26),   // #1A1A1A
                text_muted: Color32::from_rgb(96, 96, 96),     // #606060
                accent: Color32::from_rgb(0, 103, 192),        // #0067C0 WinUI 3 Light Accent
                text_on_accent: Color32::WHITE,
                accent_blue: Color32::from_rgb(0, 120, 132),   // #007884 Fluent Light Teal
                success: Color32::from_rgb(15, 123, 15),       // #0F7B0F WinUI 3 Light Success
                danger: Color32::from_rgb(196, 43, 28),        // #C42B1C WinUI 3 Light Critical
                card_rounding: 8.0,
                control_rounding: 4.0,
                is_windows: true,
            },
        }
    }
}

pub fn with_alpha(c: Color32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), alpha)
}

/// Picks dark or white text based on WCAG relative luminance so buttons stay legible on pastel accents.
pub fn contrast_text_for(bg: Color32) -> Color32 {
    fn to_linear(channel: u8) -> f32 {
        let s = channel as f32 / 255.0;
        if s <= 0.04045 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    }
    let luminance = 0.2126 * to_linear(bg.r()) + 0.7152 * to_linear(bg.g()) + 0.0722 * to_linear(bg.b());
    if luminance > 0.46 {
        Color32::from_rgb(16, 16, 16)
    } else {
        Color32::WHITE
    }
}

pub fn apply_theme(ctx: &Context, theme: AppTheme) {
    let p = Palette::for_theme(theme);
    let dark = theme.is_dark();
    let mut visuals = if dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };

    visuals.panel_fill = p.bg_canvas;
    visuals.window_fill = p.bg_card;
    visuals.extreme_bg_color = p.bg_elevated;
    visuals.faint_bg_color = p.bg_elevated;
    visuals.window_rounding = Rounding::same(p.card_rounding);
    visuals.window_stroke = Stroke::new(1.0_f32, p.border);
    visuals.menu_rounding = Rounding::same(p.control_rounding + 2.0);
    visuals.hyperlink_color = p.accent_blue;

    visuals.widgets.noninteractive.bg_fill = p.bg_card;
    visuals.widgets.noninteractive.weak_bg_fill = p.bg_elevated;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, p.border);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, p.text_primary);
    visuals.widgets.noninteractive.rounding = Rounding::same(p.control_rounding);

    visuals.widgets.inactive.bg_fill = p.bg_elevated;
    visuals.widgets.inactive.weak_bg_fill = p.bg_elevated;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, p.border);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, p.text_primary);
    visuals.widgets.inactive.rounding = Rounding::same(p.control_rounding);

    visuals.widgets.hovered.bg_fill = p.bg_hovered;
    visuals.widgets.hovered.weak_bg_fill = p.bg_hovered;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, p.accent);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, p.text_primary);
    visuals.widgets.hovered.rounding = Rounding::same(p.control_rounding);

    visuals.widgets.active.bg_fill = p.accent;
    visuals.widgets.active.weak_bg_fill = p.accent;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, p.text_on_accent);
    visuals.widgets.active.rounding = Rounding::same(p.control_rounding);

    visuals.selection.bg_fill = with_alpha(p.accent, 65);
    visuals.selection.stroke = Stroke::new(1.0_f32, p.accent);

    let mut style = Style {
        visuals,
        ..Default::default()
    };
    style.spacing.item_spacing = Vec2::new(8.0, 8.0);
    style.spacing.button_padding = Vec2::new(12.0, 6.0);
    style.spacing.interact_size = Vec2::new(36.0, 30.0);

    style.text_styles.insert(TextStyle::Heading, FontId::proportional(20.0));
    style.text_styles.insert(TextStyle::Body, FontId::proportional(14.0));
    style.text_styles.insert(TextStyle::Button, FontId::proportional(13.5));
    style.text_styles.insert(TextStyle::Small, FontId::proportional(12.0));
    style.text_styles.insert(TextStyle::Monospace, FontId::monospace(14.5));

    ctx.set_style(style);
    ctx.send_viewport_cmd(ViewportCommand::SetTheme(if dark {
        SystemTheme::Dark
    } else {
        SystemTheme::Light
    }));
}

pub fn card_frame(p: &Palette) -> Frame {
    Frame::none()
        .fill(p.bg_card)
        .stroke(Stroke::new(1.0_f32, p.border))
        .rounding(Rounding::same(p.card_rounding))
        .inner_margin(Margin::same(14.0))
}

pub fn color_dot(ui: &mut Ui, color: Color32, radius: f32) {
    let size = Vec2::splat(radius * 2.0 + 2.0);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    ui.painter().circle_filled(rect.center(), radius, color);
}

pub fn pill_badge(ui: &mut Ui, text: &str, color: Color32) {
    let bg = with_alpha(color, 36);
    Frame::none()
        .fill(bg)
        .stroke(Stroke::new(1.0_f32, with_alpha(color, 120)))
        .rounding(Rounding::same(12.0))
        .inner_margin(Margin::symmetric(8.0, 2.0))
        .show(ui, |ui| {
            ui.label(RichText::new(text).size(12.0).color(color).strong());
        });
}

pub fn primary_button(ui: &mut Ui, label: &str, fill: Color32) -> egui::Response {
    let text_color = contrast_text_for(fill);
    let rounding = ui.visuals().widgets.active.rounding;
    ui.add(
        egui::Button::new(RichText::new(label).color(text_color).strong())
            .fill(fill)
            .stroke(Stroke::NONE)
            .rounding(rounding)
            .min_size(Vec2::new(82.0, 34.0)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palettes_for_all_themes() {
        let ubuntu_dark = Palette::for_theme(AppTheme::UbuntuDark);
        let ubuntu_light = Palette::for_theme(AppTheme::UbuntuLight);
        let win_dark = Palette::for_theme(AppTheme::WindowsDark);
        let win_light = Palette::for_theme(AppTheme::WindowsLight);

        assert!(!ubuntu_dark.is_windows);
        assert!(!ubuntu_light.is_windows);
        assert!(win_dark.is_windows);
        assert!(win_light.is_windows);

        assert_ne!(ubuntu_dark.accent, win_dark.accent);
        assert_ne!(ubuntu_light.accent, win_light.accent);
        assert_ne!(win_dark.bg_canvas, win_light.bg_canvas);
        assert_ne!(win_dark.accent, win_dark.accent_blue);
        assert_ne!(win_light.accent, win_light.accent_blue);
    }

    #[test]
    fn test_contrast_text_and_with_alpha() {
        let win_dark = Palette::for_theme(AppTheme::WindowsDark);
        assert_eq!(contrast_text_for(win_dark.accent), Color32::from_rgb(16, 16, 16));
        assert_eq!(contrast_text_for(win_dark.danger), Color32::WHITE);

        let win_light = Palette::for_theme(AppTheme::WindowsLight);
        assert_eq!(contrast_text_for(win_light.accent), Color32::WHITE);

        let tinted = with_alpha(Color32::from_rgb(0, 103, 192), 40);
        assert_eq!(tinted.a(), 40);
        assert_eq!(tinted.r(), 0);
    }
}
