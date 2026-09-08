use std::collections::HashSet;

use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn finish_gif<F>(frame_num: usize, duration: f32, mut make: F) -> Result<Vec<u8>, Error>
where
    F: FnMut(usize) -> Result<Image, Error>,
{
    let mut encoder = GifEncoder::new();
    for i in 0..frame_num {
        encoder.add_frame(make(i)?, duration)?;
    }
    encoder.finish()
}

fn behind_do(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let self_locs = [(72, 5), (72, 5), (71, 2), (70, 3), (66, 5), (66, 5), (66, 5), (61, 7), (61, 7), (69, 5)];
    let user_locs = [(174, 91), (174, 91), (173, 86), (171, 87), (170, 85), (170, 85), (167, 82), (170, 85), (170, 85), (172, 88)];
    let self_head = images[0]
        .image
        .resize_bound((110, 110), Fit::Contain)
        .circle()
        .rotate(15.0);
    let user_head = images[1]
        .image
        .resize_bound((116, 116), Fit::Contain)
        .circle();

    finish_gif(10, 0.07, |i| {
        let frame = load_image(format!("behind_do/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        // old below=True for user head: user -> template -> self
        canvas.draw_image(&user_head, user_locs[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&self_head, self_locs[i], None);
        Ok(surface.image_snapshot())
    })
}
register_meme!(
    "behind_do", behind_do,
    min_images = 2, max_images = 2,
    keywords = &["后撅"],
    date_created = local_date(2025, 12, 6),
    date_modified = local_date(2025, 12, 6),
);

fn huochailu(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let head = images[0].image.square().resize_exact((110, 110)).circle();
    let locs = [
        (155, 155, 63, 28), (155, 155, 63, 28), (155, 155, 83, 38),
        (155, 155, 94, 40), (155, 155, 97, 45), (155, 155, 97, 45),
    ];
    finish_gif(6, 0.05, |i| {
        let frame = load_image(format!("huochailu/{i}.png"))?;
        let (w, h, x, y) = locs[i];
        let avatar = head.resize_exact((w, h));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&avatar, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    })
}
register_meme!(
    "huochailu", huochailu,
    min_images = 1, max_images = 1,
    keywords = &["火柴撸"],
    tags = HashSet::from(["stickman".to_string()]),
    date_created = local_date(2025, 5, 27),
    date_modified = local_date(2025, 5, 27),
);

fn laydown_do(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let head = images[0].image.resize_exact((110, 110));
    let positions = [
        (135, 18), (135, 18), (136, 33), (136, 33), (136, 33),
        (133, 61), (133, 61), (133, 61), (138, 26), (138, 26), (138, 26),
    ];
    finish_gif(11, 0.03, |i| {
        let frame = load_image(format!("laydown_do/{}.png", i + 1))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&head, positions[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    })
}
register_meme!(
    "laydown_do", laydown_do,
    min_images = 1, max_images = 1,
    keywords = &["躺撅"],
    date_created = local_date(2025, 8, 21),
    date_modified = local_date(2025, 8, 21),
);

fn masturbate(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let head = images[0].image.square().resize_exact((110, 110)).circle();
    let locs = [(207, 178, 156, 17), (194, 172, 159, 27)];
    finish_gif(2, 0.05, |i| {
        let frame = load_image(format!("masturbate/{i}.png"))?;
        let (w, h, x, y) = locs[i];
        let avatar = head.resize_exact((w, h));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&avatar, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    })
}
register_meme!(
    "masturbate", masturbate,
    min_images = 1, max_images = 1,
    keywords = &["导", "打飞机"],
    date_created = local_date(2025, 5, 27),
    date_modified = local_date(2025, 6, 14),
);

fn nailoong_do(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    const POSITIONS: [(i32, i32); 52] = [
        (194,87),(193,86),(198,78),(201,73),(201,71),(196,68),(186,76),(178,89),
        (172,103),(163,113),(162,113),(175,107),(184,98),(194,90),(199,84),(203,80),
        (203,80),(199,82),(194,91),(187,102),(177,119),(164,140),(152,142),(161,141),
        (174,132),(189,121),(203,109),(210,101),(208,93),(204,90),(203,90),(191,97),
        (181,105),(171,118),(156,135),(153,138),(167,134),(180,122),(196,113),(205,107),
        (211,96),(207,93),(205,92),(200,98),(191,107),(185,124),(175,136),(157,140),
        (157,140),(171,136),(186,128),(197,117),
    ];
    // PIL's old rotate call used expand=False here. rotate_crop preserves that canvas size.
    let head = images[0].image.resize_exact((76, 76)).rotate_crop(-45.0);
    finish_gif(52, 0.02, |i| {
        let frame = load_image(format!("nailoong_do/{}.png", i + 1))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&head, POSITIONS[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    })
}
register_meme!(
    "nailoong_do", nailoong_do,
    min_images = 1, max_images = 1,
    keywords = &["奶龙撅"],
    date_created = local_date(2026, 5, 19),
    date_modified = local_date(2026, 5, 19),
);

fn oral_sex(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let self_locs = [(30, 37), (36, 42)];
    let user_locs = [(67, 99), (71, 98)];
    let self_head = images[0]
        .image
        .resize_bound((58, 58), Fit::Contain)
        .circle()
        .rotate(15.0);
    let user_head = images[1]
        .image
        .resize_bound((48, 48), Fit::Contain)
        .circle();
    finish_gif(2, 0.05, |i| {
        let frame = load_image(format!("oral_sex/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, user_locs[i], None);
        canvas.draw_image(&self_head, self_locs[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    })
}
register_meme!(
    "oral_sex", oral_sex,
    min_images = 2, max_images = 2,
    keywords = &["口"],
    date_created = local_date(2025, 5, 27),
    date_modified = local_date(2025, 6, 14),
);

fn sitdown_do(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let head = images[0].image.resize_exact((215, 215));
    let positions = [(180, 55), (180, 68), (181, 111)];
    finish_gif(3, 0.15, |i| {
        let frame = load_image(format!("sitdown_do/{}.png", i + 1))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&head, positions[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    })
}
register_meme!(
    "sitdown_do", sitdown_do,
    min_images = 1, max_images = 1,
    keywords = &["坐撅"],
    date_created = local_date(2025, 8, 21),
    date_modified = local_date(2025, 9, 4),
);

fn spraypee(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let head = images[0].image.resize_exact((185, 185)).circle();
    finish_gif(30, 0.04, |i| {
        let frame = load_image(format!("spraypee/{}.png", i + 1))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&head, (425, 261), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    })
}
register_meme!(
    "spraypee", spraypee,
    min_images = 1, max_images = 1,
    keywords = &["滋你"],
    date_created = local_date(2026, 4, 17),
    date_modified = local_date(2026, 4, 17),
);
