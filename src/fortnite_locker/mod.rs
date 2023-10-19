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
}

pub struct FortniteLocker;

impl FortniteLocker {
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

    pub async fn get_item_img(item: &Item) -> Result<skia_safe::Image, Box<dyn std::error::Error>> {
        let output = format!(".rust_fn_locker/{}.png", item.id.to_lowercase());

        if (Path::new(&output)).exists() {
            let file_data = fs::read(output);
            let image_data = skia_safe::Data::new_copy(&file_data?);
            let image = Image::from_encoded(image_data);
            return image.ok_or_else(|| "Failed to decode image".into());
        }

        let mut surface = surfaces::raster_n32_premul((250, 265)).unwrap();
        let canvas = surface.canvas();

        let (background, overlay, color) = if let Some(series) = &*item.series {
            match series {
                Series::MarvelSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Marvel.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Marvel.png").await?,
                    Color::from_argb(255, 168, 53, 56),
                ),
                Series::CreatorCollabSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Icon.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Icon.png").await?,
                    Color::from_argb(255, 43, 134, 135),
                ),
                Series::DCUSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/DC.png").await?,
                    Self::fetch_image("./assets/locker/overlays/DC.png").await?,
                    Color::from_argb(255, 80, 97, 122),
                ),
                Series::ShadowSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Shadow.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Shadow.png").await?,
                    Color::from_argb(255, 66, 64, 63),
                ),
                Series::SlurpSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Slurp.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Slurp.png").await?,
                    Color::from_argb(255, 0, 233, 176),
                ),
                Series::LavaSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Lava.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Lava.png").await?,
                    Color::from_argb(255, 185, 102, 100),
                ),
                Series::FrozenSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Frozen.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Frozen.png").await?,
                    Color::from_argb(255, 148, 215, 244),
                ),
                Series::PlatformSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/GamingLegends.png").await?,
                    Self::fetch_image("./assets/locker/overlays/GamingLegends.png").await?,
                    Color::from_argb(255, 117, 108, 235),
                ),
                Series::ColumbusSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/StarWars.png").await?,
                    Self::fetch_image("./assets/locker/overlays/StarWars.png").await?,
                    Color::from_argb(255, 231, 196, 19),
                ),
                Series::CUBESeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Dark.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Dark.png").await?,
                    Color::from_argb(255, 179, 62, 187),
                ),
            }
        } else {
            match item.rarity {
                Rarity::Common => (
                    Self::fetch_image("./assets/locker/backgrounds/Common.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Common.png").await?,
                    Color::from_argb(255, 96, 170, 58),
                ),
                Rarity::Uncommon => (
                    Self::fetch_image("./assets/locker/backgrounds/Uncommon.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Uncommon.png").await?,
                    Color::from_argb(255, 96, 170, 58),
                ),
                Rarity::Rare => (
                    Self::fetch_image("./assets/locker/backgrounds/Rare.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Rare.png").await?,
                    Color::from_argb(255, 73, 172, 242),
                ),
                Rarity::Epic => (
                    Self::fetch_image("./assets/locker/backgrounds/Epic.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Epic.png").await?,
                    Color::from_argb(255, 177, 91, 226),
                ),
                Rarity::Legendary => (
                    Self::fetch_image("./assets/locker/backgrounds/Legendary.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Legendary.png").await?,
                    Color::from_argb(255, 211, 120, 65),
                ),
                _ => (
                    Self::fetch_image("./assets/locker/backgrounds/Common.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Common.png").await?,
                    Color::from_argb(255, 96, 170, 58),
                ),
            }
        };

        canvas.draw_image(&background, (0, 0), Some(&Paint::default()));

        let item_image_url: &str = item.images.icon.as_ref().unwrap();
        let item_image = Self::fetch_image(item_image_url).await.unwrap();

        let dest_rect = Rect::from_xywh(0.0, 0.0 - 265.0 * 0.05, 265.0 * 0.9, 265.0 * 0.9);
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        canvas.draw_image_rect(&item_image, None, &dest_rect, &paint);

        canvas.draw_image(&overlay, (0, 0), Some(&Paint::default()));

        let mut font = Font::default();
        font.set_size(265.0 * 0.1);
        font.set_typeface(TYPEFACE.clone());

        let mut text_width = font.measure_str(&item.name.to_uppercase(), None).0;
        while text_width > 250.0 * 0.9 {
            font.set_size(font.size() - 1.0);
            text_width = font.measure_str(&item.name.to_uppercase(), None).0;
        }

        let mut paint = Paint::default();
        paint.set_color(skia_safe::Color::WHITE);
        canvas.draw_str(
            &item.name.to_uppercase(),
            ((250.0 / 2.0) - (text_width / 2.0), 265.0 * 0.825),
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
        while text_width > 250.0 * 0.9 {
            font.set_size(font.size() - 1.0);
            text_width = font.measure_str(&rarity_text, None).0;
        }

        canvas.draw_str(
            &rarity_text,
            ((250.0 / 2.0) - (text_width / 2.0), 265.0 * 0.91),
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
}
