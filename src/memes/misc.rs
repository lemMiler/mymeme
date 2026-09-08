use std::collections::HashSet;

use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn mihoyo_elysia_come(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let template = load_image("mihoyo_elysia_come/0.png")?;
    make_png_or_gif(images, move |imgs: Vec<Image>| {
        let right = imgs[1].circle().resize_exact((190, 190));
        let left = imgs[0].circle().resize_exact((130, 130));
        let mut surface = new_surface(template.dimensions());
        let canvas = surface.canvas();
        // Both old paste calls used below=True, so template stays on top.
        canvas.draw_image(&right, (410, 380), None);
        canvas.draw_image(&left, (92, 310), None);
        canvas.draw_image(&template, (0, 0), None);
        Ok(surface.image_snapshot())
    })
}
register_meme!(
    "mihoyo_elysia_come", mihoyo_elysia_come,
    min_images = 2, max_images = 2,
    keywords = &["爱莉希雅降临"],
    tags = HashSet::from(["mihoyo".to_string()]),
    date_created = local_date(2025, 5, 25),
    date_modified = local_date(2025, 5, 25),
);

fn moistening_water(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let template = load_image("moistening_water/0.png")?;
    make_png_or_gif(images, move |imgs: Vec<Image>| {
        let avatar = imgs[0].resize_exact((165, 125)).rotate(-25.0);
        let mut surface = new_surface(template.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&avatar, (485, 290), None);
        canvas.draw_image(&template, (0, 0), None);
        Ok(surface.image_snapshot())
    })
}
register_meme!(
    "moistening_water", moistening_water,
    min_images = 1, max_images = 1,
    min_texts = 0, max_texts = 1,
    keywords = &["滋水"],
    date_created = local_date(2025, 12, 19),
    date_modified = local_date(2025, 12, 19),
);
