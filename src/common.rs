use skia_safe::{Canvas, Color, Image, textlayout::TextAlign};

use meme_generator_utils::{
    text::{Text2Image, TextParams},
    tools::{new_paint, new_surface},
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum VAlign {
    Center,
    Bottom,
}

pub(crate) fn choose_name(texts: &[String], image_name: &str) -> String {
    if let Some(text) = texts.first().filter(|s| !s.trim().is_empty()) {
        return text.clone();
    }
    let image_name = image_name.trim();
    if !image_name.is_empty() {
        image_name.to_string()
    } else {
        "他".to_string()
    }
}

pub(crate) fn draw_text_fit(
    canvas: &Canvas,
    bbox: (i32, i32, i32, i32),
    text: &str,
    max_font_size: f32,
    min_font_size: f32,
    text_align: TextAlign,
    font_families: &[&str],
    color: Color,
    valign: VAlign,
) -> Result<(), ()> {
    let (x1, y1, x2, y2) = bbox;
    let width = (x2 - x1).max(1) as f32;
    let height = (y2 - y1).max(1) as f32;
    let mut font_size = max_font_size;

    while font_size + f32::EPSILON >= min_font_size {
        let params = TextParams {
            font_families: font_families.iter().map(|s| (*s).to_string()).collect(),
            text_align,
            paint: new_paint(color),
            ..Default::default()
        };
        let mut rendered = Text2Image::from_text(text, font_size, params);
        rendered.layout(width);
        let rendered_height = rendered.height();
        if rendered_height <= height {
            let y = match valign {
                VAlign::Center => y1 as f32 + (height - rendered_height) / 2.0,
                VAlign::Bottom => y2 as f32 - rendered_height,
            };
            rendered.draw_on_canvas(canvas, (x1 as f32, y));
            return Ok(());
        }
        font_size -= 1.0;
    }
    Err(())
}

pub(crate) fn composite_below(template: &Image, avatar: &Image, pos: (i32, i32)) -> Image {
    let mut surface = new_surface(template.dimensions());
    let canvas = surface.canvas();
    canvas.draw_image(avatar, pos, None);
    canvas.draw_image(template, (0, 0), None);
    surface.image_snapshot()
}

pub(crate) fn composite_above(template: &Image, avatar: &Image, pos: (i32, i32)) -> Image {
    let mut surface = new_surface(template.dimensions());
    let canvas = surface.canvas();
    canvas.draw_image(template, (0, 0), None);
    canvas.draw_image(avatar, pos, None);
    surface.image_snapshot()
}
