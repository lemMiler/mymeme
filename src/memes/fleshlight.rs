use std::time::{SystemTime, UNIX_EPOCH};

use skia_safe::{Color, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{
    common::{VAlign, choose_name, composite_above, composite_below, draw_text_fit},
    options::{NoOptions, RandomFleshlightOptions},
    register_meme,
};

#[derive(Clone, Copy)]
struct FleshlightSpec {
    key: &'static str,
    text_template: &'static str,
    bbox: (i32, i32, i32, i32),
    white_text: bool,
    max_font: f32,
    min_font: f32,
    align: TextAlign,
    fonts: &'static [&'static str],
    valign: VAlign,
    avatar_pos: (i32, i32),
    avatar_size: (i32, i32),
    circle_avatar: bool,
    avatar_below: bool,
}

fn build_fleshlight(
    images: Vec<InputImage>,
    texts: Vec<String>,
    spec: FleshlightSpec,
) -> Result<Vec<u8>, Error> {
    let image_name = images.first().map(|img| img.name.as_str()).unwrap_or("");
    let name = choose_name(&texts, image_name);
    let text = spec.text_template.replace("{name}", &name);

    let template = load_image(format!("{}/0.png", spec.key))?;
    let mut template_surface = template.to_surface();
    let color = if spec.white_text { Color::WHITE } else { Color::BLACK };
    draw_text_fit(
        template_surface.canvas(),
        spec.bbox,
        &text,
        spec.max_font,
        spec.min_font,
        spec.align,
        spec.fonts,
        color,
        spec.valign,
    )
    .map_err(|_| Error::TextOverLength(name.clone()))?;
    let template = template_surface.image_snapshot();

    make_png_or_gif(images, move |imgs: Vec<Image>| {
        let avatar = if spec.circle_avatar {
            imgs[0].circle().resize_exact(spec.avatar_size)
        } else {
            imgs[0].resize_exact(spec.avatar_size)
        };
        Ok(if spec.avatar_below {
            composite_below(&template, &avatar, spec.avatar_pos)
        } else {
            composite_above(&template, &avatar, spec.avatar_pos)
        })
    })
}

macro_rules! fleshlight_fn {
    ($fn_name:ident, $spec:expr) => {
        fn $fn_name(
            images: Vec<InputImage>,
            texts: Vec<String>,
            _: NoOptions,
        ) -> Result<Vec<u8>, Error> {
            build_fleshlight(images, texts, $spec)
        }
    };
}

const FZ_SHAOER: &[&str] = &["FZShaoEr-M11S"];
const FZ_XS14: &[&str] = &["FZXS14"];
const FZ_KATONG: &[&str] = &["FZKaTong-M19S"];
const DEFAULT_FONTS: &[&str] = &[];

fleshlight_fn!(fleshlight_air_play, FleshlightSpec {
    key: "fleshlight_air_play",
    text_template: "{name}の❤️最愛",
    bbox: (305, 1, 800, 133),
    white_text: true,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Left,
    fonts: FZ_XS14,
    valign: VAlign::Center,
    avatar_pos: (252, 133),
    avatar_size: (300, 300),
    circle_avatar: false,
    avatar_below: true,
});
register_meme!(
    "fleshlight_air_play", fleshlight_air_play,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["空气玩法"],
    date_created = local_date(2025, 3, 24),
    date_modified = local_date(2025, 10, 9),
);

fleshlight_fn!(fleshlight_angel, FleshlightSpec {
    key: "fleshlight_angel",
    text_template: "{name}の❤️最愛",
    bbox: (533, 39, 779, 140),
    white_text: true,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (65, 105),
    avatar_size: (675, 675),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_angel", fleshlight_angel,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["天使心"],
    date_created = local_date(2025, 3, 24),
    date_modified = local_date(2025, 3, 24),
);

fleshlight_fn!(fleshlight_cleaning_liquid, FleshlightSpec {
    key: "fleshlight_cleaning_liquid",
    text_template: "{name}の❤️最爱",
    bbox: (13, 1039, 430, 1189),
    white_text: true,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: FZ_KATONG,
    valign: VAlign::Center,
    avatar_pos: (290, 20),
    avatar_size: (920, 920),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_cleaning_liquid", fleshlight_cleaning_liquid,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["清洗液"],
    date_created = local_date(2025, 3, 13),
    date_modified = local_date(2025, 9, 5),
);

fleshlight_fn!(fleshlight_commemorative_edition_saint_sister, FleshlightSpec {
    key: "fleshlight_commemorative_edition_saint_sister",
    text_template: "{name}の❤️最愛",
    bbox: (40, 110, 374, 207),
    white_text: false,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: DEFAULT_FONTS,
    valign: VAlign::Bottom,
    avatar_pos: (202, 252),
    avatar_size: (770, 770),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_commemorative_edition_saint_sister", fleshlight_commemorative_edition_saint_sister,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["纪念版圣修女"],
    date_created = local_date(2024, 12, 21),
    date_modified = local_date(2024, 12, 21),
);

fleshlight_fn!(fleshlight_hoshino_alice, FleshlightSpec {
    key: "fleshlight_hoshino_alice",
    text_template: "{name}の❤️最爱",
    bbox: (683, 1115, 1200, 1200),
    white_text: true,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Left,
    fonts: FZ_KATONG,
    valign: VAlign::Center,
    avatar_pos: (130, 180),
    avatar_size: (920, 920),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_hoshino_alice", fleshlight_hoshino_alice,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 0,
    keywords = &["啦啦队偶像", "拉拉队偶像"],
    date_created = local_date(2025, 3, 13),
    date_modified = local_date(2025, 9, 5),
);

fleshlight_fn!(fleshlight_idol_heartbeat, FleshlightSpec {
    key: "fleshlight_idol_heartbeat",
    text_template: "{name}の❤️最爱",
    bbox: (55, 135, 812, 282),
    white_text: false,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Left,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (340, 340),
    avatar_size: (920, 920),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_idol_heartbeat", fleshlight_idol_heartbeat,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["偶像心跳"],
    date_created = local_date(2024, 12, 21),
    date_modified = local_date(2025, 6, 13),
);

fleshlight_fn!(fleshlight_jissbon, FleshlightSpec {
    key: "fleshlight_jissbon",
    text_template: "{name}の深情❤️推荐",
    bbox: (261, 31, 758, 91),
    white_text: false,
    max_font: 120.0,
    min_font: 20.0,
    align: TextAlign::Left,
    fonts: FZ_XS14,
    valign: VAlign::Center,
    avatar_pos: (475, 180),
    avatar_size: (180, 180),
    circle_avatar: true,
    avatar_below: false,
});
register_meme!(
    "fleshlight_jissbon", fleshlight_jissbon,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["杰士邦"],
    date_created = local_date(2024, 12, 21),
    date_modified = local_date(2024, 12, 21),
);

fleshlight_fn!(fleshlight_kuileishushi, FleshlightSpec {
    key: "fleshlight_kuileishushi",
    text_template: "{name}の❤️最愛",
    bbox: (566, 606, 764, 644),
    white_text: false,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Left,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (499, 611),
    avatar_size: (65, 65),
    circle_avatar: false,
    avatar_below: true,
});
register_meme!(
    "fleshlight_kuileishushi", fleshlight_kuileishushi,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["白丝壁女"],
    date_created = local_date(2025, 7, 20),
    date_modified = local_date(2025, 7, 20),
);

fleshlight_fn!(fleshlight_limited_edition_saint_sister, FleshlightSpec {
    key: "fleshlight_limited_edition_saint_sister",
    text_template: "{name}の❤️最愛",
    bbox: (40, 110, 374, 207),
    white_text: false,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: DEFAULT_FONTS,
    valign: VAlign::Bottom,
    avatar_pos: (202, 252),
    avatar_size: (770, 770),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_limited_edition_saint_sister", fleshlight_limited_edition_saint_sister,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["限定版圣修女"],
    date_created = local_date(2024, 12, 21),
    date_modified = local_date(2024, 12, 21),
);

fleshlight_fn!(fleshlight_liuli_zi, FleshlightSpec {
    key: "fleshlight_liuli_zi",
    text_template: "{name}の❤️最愛",
    bbox: (26, 74, 380, 141),
    white_text: false,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: DEFAULT_FONTS,
    valign: VAlign::Bottom,
    avatar_pos: (145, 180),
    avatar_size: (500, 500),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_liuli_zi", fleshlight_liuli_zi,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["琉璃子"],
    date_created = local_date(2024, 12, 21),
    date_modified = local_date(2024, 12, 21),
);

fleshlight_fn!(fleshlight_machinery, FleshlightSpec {
    key: "fleshlight_machinery",
    text_template: "{name}の挚❤️爱",
    bbox: (9, 708, 577, 790),
    white_text: true,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: FZ_SHAOER,
    valign: VAlign::Bottom,
    avatar_pos: (60, 110),
    avatar_size: (680, 680),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_machinery", fleshlight_machinery,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["机械龙女", "机械龙女EVA", "机械龙女eva"],
    date_created = local_date(2024, 12, 21),
    date_modified = local_date(2024, 12, 21),
);

fleshlight_fn!(fleshlight_mengxin_packs, FleshlightSpec {
    key: "fleshlight_mengxin_packs",
    text_template: "{name}喜爱的萌新礼包",
    bbox: (43, 403, 328, 485),
    white_text: true,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (15, 115),
    avatar_size: (350, 350),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_mengxin_packs", fleshlight_mengxin_packs,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["萌新礼包"],
    date_created = local_date(2025, 6, 1),
    date_modified = local_date(2025, 6, 1),
);

fleshlight_fn!(fleshlight_miyuko_kamimiya, FleshlightSpec {
    key: "fleshlight_miyuko_kamimiya",
    text_template: "{name}の❤️最愛",
    bbox: (13, 1039, 428, 1191),
    white_text: true,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (275, -10),
    avatar_size: (950, 950),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_miyuko_kamimiya", fleshlight_miyuko_kamimiya,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["神宫美优子"],
    date_created = local_date(2025, 3, 24),
    date_modified = local_date(2025, 3, 24),
);

fleshlight_fn!(fleshlight_mizuki_shiranui, FleshlightSpec {
    key: "fleshlight_mizuki_shiranui",
    text_template: "{name}の❤️最愛",
    bbox: (331, 40, 797, 136),
    white_text: true,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Left,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (320, 144),
    avatar_size: (512, 512),
    circle_avatar: false,
    avatar_below: true,
});
register_meme!(
    "fleshlight_mizuki_shiranui", fleshlight_mizuki_shiranui,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["水城不知火"],
    date_created = local_date(2025, 7, 20),
    date_modified = local_date(2025, 7, 20),
);

fleshlight_fn!(fleshlight_nrn, FleshlightSpec {
    key: "fleshlight_nrn",
    text_template: "{name}の❤️最爱",
    bbox: (465, 5, 792, 87),
    white_text: false,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (544, 326),
    avatar_size: (230, 230),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_nrn", fleshlight_nrn,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["乳入娘"],
    date_created = local_date(2025, 9, 2),
    date_modified = local_date(2025, 9, 2),
);

fleshlight_fn!(fleshlight_pure_buttocks, FleshlightSpec {
    key: "fleshlight_pure_buttocks",
    text_template: "{name}最爱❤️",
    bbox: (840, 393, 1200, 464),
    white_text: true,
    max_font: 60.0,
    min_font: 15.0,
    align: TextAlign::Left,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (-64, 81),
    avatar_size: (860, 860),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_pure_buttocks", fleshlight_pure_buttocks,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["纯洁臀"],
    date_created = local_date(2025, 3, 13),
    date_modified = local_date(2025, 9, 5),
);

fleshlight_fn!(fleshlight_purple_spirit, FleshlightSpec {
    key: "fleshlight_purple_spirit",
    text_template: "{name}の❤️最爱",
    bbox: (799, 59, 1168, 211),
    white_text: true,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (107, 180),
    avatar_size: (1000, 1000),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_purple_spirit", fleshlight_purple_spirit,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["紫域精灵"],
    date_created = local_date(2025, 3, 24),
    date_modified = local_date(2025, 3, 24),
);

fleshlight_fn!(fleshlight_qiaobenyouxi, FleshlightSpec {
    key: "fleshlight_qiaobenyouxi",
    text_template: "{name}の❤️最爱",
    bbox: (52, 692, 334, 755),
    white_text: false,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (105, 75),
    avatar_size: (630, 630),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_qiaobenyouxi", fleshlight_qiaobenyouxi,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["桥本友希"],
    date_created = local_date(2025, 5, 30),
    date_modified = local_date(2025, 5, 30),
);

fleshlight_fn!(fleshlight_saint_sister, FleshlightSpec {
    key: "fleshlight_saint_sister",
    text_template: "{name}の❤️最愛",
    bbox: (40, 110, 374, 207),
    white_text: false,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: DEFAULT_FONTS,
    valign: VAlign::Bottom,
    avatar_pos: (202, 252),
    avatar_size: (770, 770),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_saint_sister", fleshlight_saint_sister,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["圣修女"],
    date_created = local_date(2024, 12, 21),
    date_modified = local_date(2024, 12, 21),
);

fleshlight_fn!(fleshlight_saki_haruna, FleshlightSpec {
    key: "fleshlight_saki_haruna",
    text_template: "{name}の❤️最愛",
    bbox: (252, 648, 649, 692),
    white_text: true,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Left,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (663, 575),
    avatar_size: (125, 125),
    circle_avatar: false,
    avatar_below: true,
});
register_meme!(
    "fleshlight_saki_haruna", fleshlight_saki_haruna,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["春奈纱希"],
    date_created = local_date(2025, 7, 20),
    date_modified = local_date(2025, 7, 20),
);

fleshlight_fn!(fleshlight_selena, FleshlightSpec {
    key: "fleshlight_selena",
    text_template: "{name}の❤️最愛",
    bbox: (0, 1037, 339, 1106),
    white_text: false,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: DEFAULT_FONTS,
    valign: VAlign::Bottom,
    avatar_pos: (140, 195),
    avatar_size: (920, 920),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_selena", fleshlight_selena,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["魔女之森"],
    date_created = local_date(2025, 3, 13),
    date_modified = local_date(2025, 3, 13),
);

fleshlight_fn!(fleshlight_starter_pack, FleshlightSpec {
    key: "fleshlight_starter_pack",
    text_template: "{name}の❤️新手礼包",
    bbox: (93, 688, 493, 769),
    white_text: false,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (15, 185),
    avatar_size: (580, 580),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_starter_pack", fleshlight_starter_pack,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["新手礼包"],
    date_created = local_date(2025, 5, 30),
    date_modified = local_date(2025, 5, 30),
);

fleshlight_fn!(fleshlight_summer_liuli_zi, FleshlightSpec {
    key: "fleshlight_summer_liuli_zi",
    text_template: "{name}の❤️最愛",
    bbox: (35, 111, 428, 210),
    white_text: false,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Left,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (210, 265),
    avatar_size: (770, 770),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_summer_liuli_zi", fleshlight_summer_liuli_zi,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["夏日琉璃子"],
    date_created = local_date(2024, 12, 21),
    date_modified = local_date(2025, 7, 23),
);

fleshlight_fn!(fleshlight_taimanin_asgi, FleshlightSpec {
    key: "fleshlight_taimanin_asgi",
    text_template: "{name}の❤️最愛",
    bbox: (35, 111, 428, 210),
    white_text: false,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: DEFAULT_FONTS,
    valign: VAlign::Bottom,
    avatar_pos: (215, 222),
    avatar_size: (780, 780),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_taimanin_asgi", fleshlight_taimanin_asgi,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["对魔忍"],
    date_created = local_date(2024, 12, 21),
    date_modified = local_date(2024, 12, 21),
);

fleshlight_fn!(fleshlight_xingnai, FleshlightSpec {
    key: "fleshlight_xingnai",
    text_template: "{name}の❤️最爱",
    bbox: (22, 638, 739, 737),
    white_text: false,
    max_font: 100.0,
    min_font: 20.0,
    align: TextAlign::Center,
    fonts: FZ_SHAOER,
    valign: VAlign::Center,
    avatar_pos: (130, 150),
    avatar_size: (475, 475),
    circle_avatar: true,
    avatar_below: true,
});
register_meme!(
    "fleshlight_xingnai", fleshlight_xingnai,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["杏奈"],
    date_created = local_date(2025, 5, 30),
    date_modified = local_date(2025, 5, 30),
);

const RANDOM_TEXT_POSITIONS: [(i32, i32, i32, i32); 25] = [
    (305, 1, 800, 133), (533, 39, 779, 140), (13, 1039, 430, 1189),
    (40, 110, 374, 207), (686, 1100, 1200, 1200), (55, 135, 812, 282),
    (261, 31, 758, 91), (566, 606, 764, 644), (40, 110, 374, 207),
    (26, 74, 380, 141), (9, 708, 577, 790), (43, 403, 328, 485),
    (13, 1039, 428, 1191), (331, 40, 797, 136), (465, 5, 792, 87),
    (840, 393, 1200, 464), (799, 59, 1168, 211), (52, 692, 334, 755),
    (40, 110, 374, 207), (252, 648, 649, 692), (0, 1037, 339, 1106),
    (93, 688, 493, 769), (35, 111, 428, 210), (35, 111, 428, 210),
    (22, 638, 739, 737),
];
const RANDOM_WHITE_TEXT: [bool; 25] = [
    true, true, true, false, true, false, false, false, false, false,
    true, true, true, true, false, true, false, true, false, true,
    false, true, false, false, false,
];
const RANDOM_AVATAR_POSITIONS: [(i32, i32); 25] = [
    (252, 133), (65, 105), (290, 20), (202, 252), (130, 180),
    (340, 340), (475, 180), (499, 611), (202, 252), (145, 180),
    (60, 110), (15, 115), (275, -10), (320, 144), (544, 326),
    (-64, 81), (107, 180), (105, 75), (202, 252), (663, 575),
    (140, 195), (15, 185), (210, 265), (215, 222), (130, 150),
];
const RANDOM_AVATAR_SIZES: [i32; 25] = [
    300, 675, 920, 770, 920, 920, 180, 65, 770, 500,
    680, 350, 950, 512, 230, 860, 1000, 630, 770, 125,
    920, 580, 770, 780, 475,
];

fn pseudo_random_index() -> usize {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    (nanos % 25) as usize
}

fn fleshlight_random(
    images: Vec<InputImage>,
    texts: Vec<String>,
    options: RandomFleshlightOptions,
) -> Result<Vec<u8>, Error> {
    let number = options.number.unwrap_or(0);
    let index = if number == 0 {
        pseudo_random_index()
    } else if (1..=25).contains(&number) {
        (number - 1) as usize
    } else {
        return Err(Error::MemeFeedback("图片编号错误，请选择 1~25".to_string()));
    };

    let image_name = images.first().map(|img| img.name.as_str()).unwrap_or("");
    let name = choose_name(&texts, image_name);
    let text = format!("{name}の❤️最爱");
    let frame = load_image(format!("fleshlight_random/{index}.png"))?;
    let mut surface = frame.to_surface();
    draw_text_fit(
        surface.canvas(),
        RANDOM_TEXT_POSITIONS[index],
        &text,
        100.0,
        5.0,
        TextAlign::Left,
        FZ_SHAOER,
        if RANDOM_WHITE_TEXT[index] { Color::WHITE } else { Color::BLACK },
        VAlign::Center,
    )
    .map_err(|_| Error::TextOverLength(name.clone()))?;
    let frame = surface.image_snapshot();
    let avatar_pos = RANDOM_AVATAR_POSITIONS[index];
    let avatar_size = RANDOM_AVATAR_SIZES[index];

    make_png_or_gif(images, move |imgs: Vec<Image>| {
        let avatar = imgs[0].circle().resize_exact((avatar_size, avatar_size));
        Ok(composite_below(&frame, &avatar, avatar_pos))
    })
}

register_meme!(
    "fleshlight_random", fleshlight_random,
    min_images = 1, max_images = 1, min_texts = 0, max_texts = 1,
    keywords = &["随机杯子"],
    date_created = local_date(2025, 9, 2),
    date_modified = local_date(2025, 9, 2),
);
