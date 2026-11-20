use egui::{Color32, Context, FontId, Frame, Margin, RichText, Rounding, Stroke, Style, TextStyle, Ui, Vec2, Visuals};

pub struct Palette {
    pub bg_canvas: Color32,
    pub bg_sidebar: Color32,
    pub bg_card: Color32,
    pub bg_elevated: Color32,
    pub border: Color32,
    pub text_primary: Color32,
    pub text_muted: Color32,
    pub accent: Color32,
    pub accent_blue: Color32,
    pub success: Color32,
    pub danger: Color32,
}

impl Palette {
    pub fn for_mode(dark: bool) -> Self {
        if dark {
            Self {
                bg_canvas: Color32::from_rgb(28, 28, 30),
                bg_sidebar: Color32::from_rgb(22, 22, 24),
                bg_card: Color32::from_rgb(38, 38, 42),
                bg_elevated: Color32::from_rgb(48, 48, 54),
                border: Color32::from_rgb(62, 62, 68),
                text_primary: Color32::from_rgb(242, 242, 244),
                text_muted: Color32::from_rgb(154, 154, 162),
                accent: Color32::from_rgb(233, 84, 32),      // Ubuntu Yaru Orange
                accent_blue: Color32::from_rgb(3, 169, 244), // Clockify Blue
                success: Color32::from_rgb(46, 204, 113),
                danger: Color32::from_rgb(231, 76, 60),
            }
        } else {
            Self {
                bg_canvas: Color32::from_rgb(246, 245, 244),
                bg_sidebar: Color32::from_rgb(236, 235, 233),
                bg_card: Color32::from_rgb(255, 255, 255),
                bg_elevated: Color32::from_rgb(242, 241, 240),
                border: Color32::from_rgb(214, 212, 208),
                text_primary: Color32::from_rgb(36, 36, 38),
                text_muted: Color32::from_rgb(112, 112, 120),
                accent: Color32::from_rgb(233, 84, 32),
                accent_blue: Color32::from_rgb(2, 136, 209),
                success: Color32::from_rgb(39, 174, 96),
                danger: Color32::from_rgb(214, 58, 46),
            }
        }
    }
}

pub fn apply_theme(ctx: &Context, dark: bool) {
    let p = Palette::for_mode(dark);
    let mut visuals = if dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };

    visuals.panel_fill = p.bg_canvas;
    visuals.window_fill = p.bg_card;
    visuals.extreme_bg_color = p.bg_elevated;
    visuals.faint_bg_color = p.bg_elevated;
    visuals.window_rounding = Rounding::same(10.0);
    visuals.window_stroke = Stroke::new(1.0_f32, p.border);
    visuals.menu_rounding = Rounding::same(8.0);

    visuals.widgets.noninteractive.bg_fill = p.bg_card;
    visuals.widgets.noninteractive.weak_bg_fill = p.bg_elevated;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, p.border);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, p.text_primary);
    visuals.widgets.noninteractive.rounding = Rounding::same(6.0);

    visuals.widgets.inactive.bg_fill = p.bg_elevated;
    visuals.widgets.inactive.weak_bg_fill = p.bg_elevated;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, p.border);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, p.text_primary);
    visuals.widgets.inactive.rounding = Rounding::same(6.0);

    visuals.widgets.hovered.bg_fill = if dark {
        Color32::from_rgb(60, 60, 68)
    } else {
        Color32::from_rgb(230, 229, 226)
    };
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, p.accent);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, p.text_primary);
    visuals.widgets.hovered.rounding = Rounding::same(6.0);

    visuals.widgets.active.bg_fill = p.accent;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
    visuals.widgets.active.rounding = Rounding::same(6.0);

    visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(233, 84, 32, 65);
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
}

pub fn card_frame(p: &Palette) -> Frame {
    Frame::none()
        .fill(p.bg_card)
        .stroke(Stroke::new(1.0_f32, p.border))
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::same(14.0))
}

pub fn color_dot(ui: &mut Ui, color: Color32, radius: f32) {
    let size = Vec2::splat(radius * 2.0 + 2.0);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    ui.painter().circle_filled(rect.center(), radius, color);
}

pub fn pill_badge(ui: &mut Ui, text: &str, color: Color32) {
    let bg = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 36);
    Frame::none()
        .fill(bg)
        .stroke(Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 120)))
        .rounding(Rounding::same(12.0))
        .inner_margin(Margin::symmetric(8.0, 2.0))
        .show(ui, |ui| {
            ui.label(RichText::new(text).size(12.0).color(color).strong());
        });
}

pub fn primary_button(ui: &mut Ui, label: &str, fill: Color32) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(label).color(Color32::WHITE).strong())
            .fill(fill)
            .stroke(Stroke::NONE)
            .rounding(Rounding::same(7.0))
            .min_size(Vec2::new(82.0, 34.0)),
    )
}
