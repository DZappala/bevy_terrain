use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    image::ImageLoaderError,
    prelude::{Image, Result, Vec},
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use bytemuck::cast_slice;
use std::io::Cursor;
use tiff::{
    ColorType,
    decoder::{Decoder, DecodingResult},
};

#[derive(Default)]
pub struct TiffLoader;
impl AssetLoader for TiffLoader {
    type Asset = Image;
    type Settings = ();
    type Error = ImageLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        ctx: &mut LoadContext<'_>,
    ) -> Result<Image, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let mut decoder = Decoder::new(Cursor::new(bytes)).unwrap();

        let (width, height) = decoder.dimensions().unwrap();

        let decoding_result = decoder.read_image().unwrap();
        let (data, dtype_str) = match &decoding_result {
            DecodingResult::U8(data) => (cast_slice(data).to_vec(), "U8"),
            DecodingResult::U16(data) => (cast_slice(data).to_vec(), "U16"),
            DecodingResult::U32(data) => (cast_slice(data).to_vec(), "U32"),
            DecodingResult::U64(data) => (cast_slice(data).to_vec(), "U64"),
            DecodingResult::F32(data) => (cast_slice(data).to_vec(), "F32"),
            DecodingResult::F64(data) => (cast_slice(data).to_vec(), "F64"),
            DecodingResult::I8(data) => (cast_slice(data).to_vec(), "I8"),
            DecodingResult::I16(data) => (cast_slice(data).to_vec(), "I16"),
            DecodingResult::I32(data) => (cast_slice(data).to_vec(), "I32"),
            DecodingResult::I64(data) => (cast_slice(data).to_vec(), "I64"),
        };

        let path_ref = ctx.asset_path();
        let color_type = decoder.colortype().unwrap_or_else(|err| {
            panic!(
                "Header of .tif does not define a colortype or dtype\nPath: {path_ref:?}\nDetails: {err:?}"
            )
        });

        let tex_fmt = match color_type {
            ColorType::Gray(_) | ColorType::GrayA(_) => match decoding_result {
                DecodingResult::U8(_) => TextureFormat::R8Unorm,
                DecodingResult::U16(_) => TextureFormat::R16Unorm,
                DecodingResult::U32(_) => TextureFormat::R32Uint,
                DecodingResult::F32(_) => TextureFormat::R32Float,
                _ => todo!(
                    "Unimplemented colortype-datatype combination. Valid data types for \"Gray\" are UInt8 (a.k.a. byte), UInt16, and UInt32\nPath: {path_ref:?}\nColortype: {color_type:?}\n{dtype_str}"
                ),
            },
            ColorType::RGB(_) | ColorType::RGBA(_) => match decoding_result {
                DecodingResult::U8(_) => TextureFormat::Rgba8Unorm,
                _ => todo!(
                    "Unimplemented colortype-datatype combination. Valid data types for \"RGB and RGBA\" is UInt8 (a.k.a. byte).\nPath: {path_ref:?}\nColortype: {color_type:?}\nDatatype: {dtype_str:?}"
                ),
            },
            _ => todo!(
                ".tif colortype or dtype not yet implemented.\nPath: {path_ref:?}\nColortype:{color_type:?}\nDatatype: {dtype_str:?}",
            ),
        };

        // let img_result: Result<Image, Box<dyn Any + Send>> = catch_unwind(|| -> Image {
        //     Image::new(
        //         Extent3d {
        //             width,
        //             height,
        //             depth_or_array_layers: 1,
        //         },
        //         TextureDimension::D2,
        //         data.clone(),
        //         tex_fmt,
        //         RenderAssetUsages::MAIN_WORLD,
        //     )
        // });

        // match img_result {
        //     Ok(x) => Ok(x),
        //     Err(_) => panic!(
        //         "Failed to convert Image:\n\tpath: {path_ref:?}\n\tcolortype {color_type:?}\n\tdtype {dtype_str:?}"
        //     ),
        // }

        let mut image = Image::new_uninit(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            tex_fmt,
            RenderAssetUsages::MAIN_WORLD,
        );

        // Avoid Image::new size assert
        image.data = Some(data);

        Ok(image)
    }

    fn extensions(&self) -> &[&str] {
        &["tif", "tiff"]
    }
}
