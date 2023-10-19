extern crate psutil;
extern crate skia_safe;

use crate::fortnite_api_io::models::items::Item;
use crate::fortnite_api_io::models::items::Rarity;
use crate::fortnite_api_io::models::items::Series;
use reqwest;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use skia_safe::{encode, surfaces, EncodedImageFormat, Image, Paint, Rect};

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

        let mut surface = surfaces::raster_n32_premul((500, 530)).unwrap();
        let canvas = surface.canvas();

        let (background, overlay) = if let Some(series) = &*item.series {
            match series {
                Series::MarvelSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Marvel.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Marvel.png").await?,
                ),
                Series::CreatorCollabSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Icon.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Icon.png").await?,
                ),
                Series::DCUSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/DC.png").await?,
                    Self::fetch_image("./assets/locker/overlays/DC.png").await?,
                ),
                Series::ShadowSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Dark.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Dark.png").await?,
                ),
                Series::SlurpSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Slurp.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Slurp.png").await?,
                ),
                Series::LavaSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Lava.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Lava.png").await?,
                ),
                Series::FrozenSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Frozen.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Frozen.png").await?,
                ),
                Series::PlatformSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/GamingLegends.png").await?,
                    Self::fetch_image("./assets/locker/overlays/GamingLegends.png").await?,
                ),
                Series::ColumbusSeries => (
                    Self::fetch_image("./assets/locker/backgrounds/StarWars.png").await?,
                    Self::fetch_image("./assets/locker/overlays/StarWars.png").await?,
                ),
                Series::CUBESeries => (
                    Self::fetch_image("./assets/locker/backgrounds/Shadow.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Shadow.png").await?,
                ),
            }
        } else {
            match item.rarity {
                Rarity::Common => (
                    Self::fetch_image("./assets/locker/backgrounds/Common.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Common.png").await?,
                ),
                Rarity::Uncommon => (
                    Self::fetch_image("./assets/locker/backgrounds/Uncommon.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Uncommon.png").await?,
                ),
                Rarity::Rare => (
                    Self::fetch_image("./assets/locker/backgrounds/Rare.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Rare.png").await?,
                ),
                Rarity::Epic => (
                    Self::fetch_image("./assets/locker/backgrounds/Epic.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Epic.png").await?,
                ),
                Rarity::Legendary => (
                    Self::fetch_image("./assets/locker/backgrounds/Legendary.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Legendary.png").await?,
                ),
                _ => (
                    Self::fetch_image("./assets/locker/backgrounds/Common.png").await?,
                    Self::fetch_image("./assets/locker/overlays/Common.png").await?,
                ),
            }
        };

        canvas.draw_image(&background, (0, 0), Some(&Paint::default()));

        let item_image_url: &str = item.images.icon.as_ref().unwrap();
        let item_image = Self::fetch_image(item_image_url).await.unwrap();

        let dest_rect = Rect::from_xywh(0.0, 0.0 - 530.0 * 0.05, 530.0 * 0.9, 530.0 * 0.9);
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        canvas.draw_image_rect(&item_image, None, &dest_rect, &paint);

        canvas.draw_image(&overlay, (0, 0), Some(&Paint::default()));

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
