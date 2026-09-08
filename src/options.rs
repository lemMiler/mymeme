use meme_generator_utils::builder::MemeOptions;

#[derive(MemeOptions)]
pub(crate) struct NoOptions {}

#[derive(MemeOptions)]
pub(crate) struct RandomFleshlightOptions {
    /// 图片编号，0 为随机，1~25 为指定图片
    #[option(short, long, default = 0)]
    pub number: Option<i32>,
}
