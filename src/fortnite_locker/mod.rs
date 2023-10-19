use crate::fortnite_api_io::models::items::Item;
use crate::fortnite_api_io::models::items::Rarity;
use crate::fortnite_api_io::models::items::Series;
use lazy_static::lazy_static;
use reqwest;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use skia_safe::{
    encode, surfaces, Color, Data, EncodedImageFormat, Font, Image, Paint, Rect, Typeface,
};

lazy_static! {
    static ref FONT_DATA: Vec<u8> =
        fs::read("./assets/fonts/BurbankBigRegular-Black.otf").expect("Failed to read font file");
    static ref TYPEFACE: Typeface = Typeface::from_data(Data::new_copy(&FONT_DATA), None)
        .expect("Failed to create typeface")
        .into();
    static ref MARVEL_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/Marvel.png").unwrap()
    ))
    .unwrap();
    static ref MARVEL_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/Marvel.png").unwrap()
    ))
    .unwrap();
    static ref CREATOR_COLLAB_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/Icon.png").unwrap()
    ))
    .unwrap();
    static ref CREATOR_COLLAB_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/Icon.png").unwrap()
    ))
    .unwrap();
    static ref DCU_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/DC.png").unwrap()
    ))
    .unwrap();
    static ref DCU_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/DC.png").unwrap()
    ))
    .unwrap();
    static ref SHADOW_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/Shadow.png").unwrap()
    ))
    .unwrap();
    static ref SHADOW_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/Shadow.png").unwrap()
    ))
    .unwrap();
    static ref SLURP_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/Slurp.png").unwrap()
    ))
    .unwrap();
    static ref SLURP_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/Slurp.png").unwrap()
    ))
    .unwrap();
    static ref LAVA_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/Lava.png").unwrap()
    ))
    .unwrap();
    static ref LAVA_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/Lava.png").unwrap()
    ))
    .unwrap();
    static ref FRONZEN_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/Frozen.png").unwrap()
    ))
    .unwrap();
    static ref FRONZEN_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/Frozen.png").unwrap()
    ))
    .unwrap();
    static ref GAMING_LEGENDS_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/GamingLegends.png").unwrap()
    ))
    .unwrap();
    static ref GAMING_LEGENDS_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/GamingLegends.png").unwrap()
    ))
    .unwrap();
    static ref COLUMBUS_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/StarWars.png").unwrap()
    ))
    .unwrap();
    static ref COLUMBUS_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/StarWars.png").unwrap()
    ))
    .unwrap();
    static ref CUBE_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/Dark.png").unwrap()
    ))
    .unwrap();
    static ref CUBE_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/Dark.png").unwrap()
    ))
    .unwrap();
    static ref COMMON_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/Common.png").unwrap()
    ))
    .unwrap();
    static ref COMMON_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/Common.png").unwrap()
    ))
    .unwrap();
    static ref UNCOMMON_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/Uncommon.png").unwrap()
    ))
    .unwrap();
    static ref UNCOMMON_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/Uncommon.png").unwrap()
    ))
    .unwrap();
    static ref RARE_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/Rare.png").unwrap()
    ))
    .unwrap();
    static ref RARE_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/Rare.png").unwrap()
    ))
    .unwrap();
    static ref EPIC_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/Epic.png").unwrap()
    ))
    .unwrap();
    static ref EPIC_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/Epic.png").unwrap()
    ))
    .unwrap();
    static ref LEGENDARY_BACKGROUND: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/backgrounds/Legendary.png").unwrap()
    ))
    .unwrap();
    static ref LEGENDARY_OVERLAY: Image = Image::from_encoded(Data::new_copy(
        &fs::read("./assets/locker/overlays/Legendary.png").unwrap()
    ))
    .unwrap();
}

pub struct FortniteLocker {
    item_width: f32,
    item_height: f32,
}

impl FortniteLocker {
    pub fn new(item_width: f32, item_height: f32) -> Self {
        Self {
            item_width,
            item_height,
        }
    }

    async fn fetch_image(url: &str) -> Result<skia_safe::Image, Box<dyn std::error::Error>> {
        if Path::new(url).exists() {
            let file_data = fs::read(url);
            let image_data = skia_safe::Data::new_copy(&file_data?);
            let image = Image::from_encoded(image_data);
            return image.ok_or_else(|| "Failed to decode image".into());
        }

        let bytes = reqwest::get(url).await?.bytes().await?;
        let image_data = skia_safe::Data::new_copy(&bytes);
        let image = Image::from_encoded(image_data);

        image.ok_or_else(|| "Failed to decode image".into())
    }

    pub async fn get_item_img(
        &self,
        item: &Item,
    ) -> Result<skia_safe::Image, Box<dyn std::error::Error>> {
        let output = format!(".rust_fn_locker/{}.png", item.id.to_lowercase());

        if (Path::new(&output)).exists() {
            let file_data = fs::read(output);
            let image_data = skia_safe::Data::new_copy(&file_data?);
            let image = Image::from_encoded(image_data);
            return image.ok_or_else(|| "Failed to decode image".into());
        }

        let mut surface =
            surfaces::raster_n32_premul((self.item_width as i32, self.item_height as i32)).unwrap();
        let canvas = surface.canvas();

        let (background, overlay, color) = if let Some(series) = &*item.series {
            match series {
                Series::MarvelSeries => (
                    MARVEL_BACKGROUND.clone(),
                    MARVEL_OVERLAY.clone(),
                    Color::from_argb(255, 168, 53, 56),
                ),
                Series::CreatorCollabSeries => (
                    CREATOR_COLLAB_BACKGROUND.clone(),
                    CREATOR_COLLAB_OVERLAY.clone(),
                    Color::from_argb(255, 43, 134, 135),
                ),
                Series::DCUSeries => (
                    DCU_BACKGROUND.clone(),
                    DCU_OVERLAY.clone(),
                    Color::from_argb(255, 80, 97, 122),
                ),
                Series::ShadowSeries => (
                    SHADOW_BACKGROUND.clone(),
                    SHADOW_OVERLAY.clone(),
                    Color::from_argb(255, 66, 64, 63),
                ),
                Series::SlurpSeries => (
                    SLURP_BACKGROUND.clone(),
                    SLURP_OVERLAY.clone(),
                    Color::from_argb(255, 0, 233, 176),
                ),
                Series::LavaSeries => (
                    LAVA_BACKGROUND.clone(),
                    LAVA_OVERLAY.clone(),
                    Color::from_argb(255, 185, 102, 100),
                ),
                Series::FrozenSeries => (
                    FRONZEN_BACKGROUND.clone(),
                    FRONZEN_OVERLAY.clone(),
                    Color::from_argb(255, 148, 215, 244),
                ),
                Series::PlatformSeries => (
                    GAMING_LEGENDS_BACKGROUND.clone(),
                    GAMING_LEGENDS_OVERLAY.clone(),
                    Color::from_argb(255, 117, 108, 235),
                ),
                Series::ColumbusSeries => (
                    COLUMBUS_BACKGROUND.clone(),
                    COLUMBUS_OVERLAY.clone(),
                    Color::from_argb(255, 231, 196, 19),
                ),
                Series::CUBESeries => (
                    CUBE_BACKGROUND.clone(),
                    CUBE_OVERLAY.clone(),
                    Color::from_argb(255, 179, 62, 187),
                ),
            }
        } else {
            match item.rarity {
                Rarity::Common => (
                    COMMON_BACKGROUND.clone(),
                    COMMON_OVERLAY.clone(),
                    Color::from_argb(255, 96, 170, 58),
                ),
                Rarity::Uncommon => (
                    UNCOMMON_BACKGROUND.clone(),
                    UNCOMMON_OVERLAY.clone(),
                    Color::from_argb(255, 96, 170, 58),
                ),
                Rarity::Rare => (
                    RARE_BACKGROUND.clone(),
                    RARE_OVERLAY.clone(),
                    Color::from_argb(255, 73, 172, 242),
                ),
                Rarity::Epic => (
                    EPIC_BACKGROUND.clone(),
                    EPIC_OVERLAY.clone(),
                    Color::from_argb(255, 177, 91, 226),
                ),
                Rarity::Legendary => (
                    LEGENDARY_BACKGROUND.clone(),
                    LEGENDARY_OVERLAY.clone(),
                    Color::from_argb(255, 211, 120, 65),
                ),
                _ => (
                    COMMON_BACKGROUND.clone(),
                    COMMON_OVERLAY.clone(),
                    Color::from_argb(255, 96, 170, 58),
                ),
            }
        };

        canvas.draw_image(background, (0, 0), Some(&Paint::default()));

        let item_image_url: &str = item.images.icon.as_ref().unwrap();
        let item_image = Self::fetch_image(item_image_url).await.unwrap();

        let dest_rect = Rect::from_xywh(0.0, 0.0 - 265.0 * 0.05, 265.0 * 0.9, 265.0 * 0.9);
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        canvas.draw_image_rect(item_image, None, &dest_rect, &paint);

        canvas.draw_image(overlay, (0, 0), Some(&Paint::default()));

        let mut font = Font::default();
        font.set_size(265.0 * 0.1);
        font.set_typeface(TYPEFACE.clone());

        let mut text_width = font.measure_str(&item.name.to_uppercase(), None).0;
        while text_width > self.item_width * 0.9 {
            font.set_size(font.size() - 1.0);
            text_width = font.measure_str(&item.name.to_uppercase(), None).0;
        }

        let mut paint = Paint::default();
        paint.set_color(skia_safe::Color::WHITE);
        canvas.draw_str(
            &item.name.to_uppercase(),
            ((self.item_width / 2.0) - (text_width / 2.0), 265.0 * 0.825),
            &font,
            &paint,
        );

        font.set_size(265.0 * 0.1);

        let rarity_text = format!(
            "{} {}",
            item.rarity.to_string().to_uppercase(),
            item.item_type.to_string().to_uppercase()
        );

        let mut paint = Paint::default();
        paint.set_color(color);

        let mut text_width = font.measure_str(&rarity_text, None).0;
        while text_width > self.item_width * 0.9 {
            font.set_size(font.size() - 1.0);
            text_width = font.measure_str(&rarity_text, None).0;
        }

        canvas.draw_str(
            &rarity_text,
            (
                (self.item_width / 2.0) - (text_width / 2.0),
                self.item_height * 0.91,
            ),
            &font,
            &paint,
        );

        let image_data = surface.image_snapshot();
        if let Some(png_data) = encode::image(None, &image_data, EncodedImageFormat::PNG, Some(100))
        {
            let mut file = File::create(&output).unwrap();
            file.write_all(png_data.as_bytes()).unwrap();
        }

        let file_data = fs::read(output);
        let image_data = skia_safe::Data::new_copy(&file_data?);
        let image = Image::from_encoded(image_data);

        image.ok_or_else(|| "Failed to decode image".into())
    }

    pub async fn generate_locker(&self, items: Vec<Item>) -> Vec<u8> {
        let gap = 30.0;

        let rendered_length = (items.len() as f32).sqrt().ceil() as i32;

        let c_x = self.item_width * rendered_length as f32 + gap + rendered_length as f32 * gap;
        let header_scale = c_x / 2000.0;
        let c_y = self.item_height * (items.len() as f32 / rendered_length as f32).ceil()
            + gap
            + rendered_length as f32 * gap
            + header_scale * 128.0
            + header_scale * 80.0;

        let mut surface = surfaces::raster_n32_premul((c_x as i32, c_y as i32)).unwrap();
        let canvas = surface.canvas();

        for (i, item) in items.iter().enumerate() {
            let x = (i as i32 % rendered_length) as f32 * (self.item_width + gap) + gap;
            let y = (i as i32 / rendered_length) as f32 * (self.item_height + gap) + gap;

            let item_image = self.get_item_img(item).await.unwrap();

            let dest_rect = Rect::from_xywh(
                x + (self.item_width * 0.05),
                y + (self.item_height * 0.05),
                self.item_width * 0.9,
                self.item_height * 0.9,
            );
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            canvas.draw_image_rect(item_image, None, &dest_rect, &paint);
        }

        let image_data = surface.image_snapshot();
        let png_data_option = encode::image(None, &image_data, EncodedImageFormat::PNG, Some(100));

        match png_data_option {
            Some(png_data) => {
                // Convert SkData to Vec<u8>
                let bytes: Vec<u8> = png_data.as_bytes().to_vec();
                bytes
            }
            None => {
                panic!("Failed to encode image");
            }
        }
    }
}
