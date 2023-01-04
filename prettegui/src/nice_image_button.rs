use bevy_egui::egui::{
    pos2, widgets, Rect, Response, Sense, TextStyle, TextureId, Ui, Vec2, Widget, WidgetInfo,
    WidgetText, WidgetType,
};

/// A clickable image within a frame.
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
#[derive(Clone)]
pub struct TextImageButton {
    pub pressed_image: Option<widgets::Image>,
    pub default_image: Option<widgets::Image>,
    pub hovered_image: Option<widgets::Image>,
    pub text: Option<WidgetText>,
    pub selected: bool,
    pub frame: bool,
    pub size: Vec2,
    pub image_offset: Vec2,
}

impl TextImageButton {
    pub fn new(
        pressed_texture_id: impl Into<TextureId>,
        default_texture_id: impl Into<TextureId>,
        size: impl Into<Vec2> + Clone,
        text: Option<WidgetText>,
    ) -> Self {
        let pressed_image = widgets::Image::new(pressed_texture_id, size.clone());
        let default_image = widgets::Image::new(default_texture_id, size);
        Self {
            pressed_image: Some(pressed_image),
            default_image: Some(default_image),
            hovered_image: Some(pressed_image),
            text,
            selected: false,
            frame: false,
            size: default_image.size(),
            image_offset: Vec2::ZERO,
        }
    }
}

impl Widget for TextImageButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            pressed_image,
            default_image,
            hovered_image,
            text,
            selected,
            frame,
            size,
            image_offset,
        } = self;
        //TODO remove these
        let wrap = None;
        // ??
        let sense = Sense::click();

        let ui_button_padding = ui.spacing().button_padding;
        let mut final_padded_size = size + ui_button_padding * 2.0;

        let total_extra_space = ui_button_padding + ui_button_padding;
        let allowable_wrap_width = ui.available_width() - total_extra_space.x;

        let text_widget_galley = if let Some(text) = text {
            let text_galley = text.into_galley(ui, wrap, allowable_wrap_width, TextStyle::Button);
            final_padded_size = final_padded_size.max(text_galley.size() + ui_button_padding * 2.0);
            Some(text_galley)
        } else {
            None
        };

        let (rect, widget) = ui.allocate_at_least(final_padded_size, sense);

        if let Some(text_widget_galley) = text_widget_galley.clone() {
            widget
                .widget_info(|| WidgetInfo::labeled(WidgetType::Button, text_widget_galley.text()));
        } else {
            widget.widget_info(|| WidgetInfo::new(WidgetType::ImageButton));
        }

        let used_image =
            if pressed_image.is_some() && (widget.is_pointer_button_down_on() || selected) {
                pressed_image
            } else if hovered_image.is_some() && widget.hovered() {
                hovered_image
            } else {
                default_image
            };

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&widget);
            let expansion = Vec2::splat(visuals.expansion);

            if frame {
                // Draw frame background (for transparent images):
                ui.painter()
                    .rect_filled(rect.expand2(expansion), visuals.rounding, {
                        if selected {
                            ui.style().visuals.selection.bg_fill
                        } else {
                            visuals.bg_fill
                        }
                    });
            }

            if let Some(image) = used_image {
                let image_rect = Rect::from_center_size(rect.center() + image_offset, image.size());
                image.paint_at(ui, image_rect);
            }

            let visuals = ui.style().interact(&widget);

            if let Some(text_widget_galley) = text_widget_galley {
                let text_pos = pos2(
                    rect.center().x - text_widget_galley.size().x / 2.0,
                    rect.center().y - 0.5 * text_widget_galley.size().y,
                );
                text_widget_galley.paint_with_visuals(ui.painter(), text_pos, visuals);
            }
            if frame {
                // Draw frame outline:
                ui.painter()
                    .rect_stroke(rect.expand2(expansion), visuals.rounding, {
                        if selected {
                            // ui.style().visuals.selection.stroke
                            visuals.bg_stroke
                        } else {
                            visuals.bg_stroke
                        }
                    });
            }
        }

        widget
    }
}
