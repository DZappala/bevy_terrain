use std::env::set_var;

use bevy_terrain::prelude::{AttachmentFormat, AttachmentLabel};
use bevy_terrain_preprocess::prelude::{
    Cli, PreprocessContext, PreprocessDataType, PreprocessNoData, preprocess,
};
use gdal::raster::GdalDataType;

fn main() {
    unsafe {
        if true {
            set_var("RAYON_NUM_THREADS", "0");
            set_var("GDAL_NUM_THREADS", "ALL_CPUS");
        } else {
            set_var("RAYON_NUM_THREADS", "1");
            set_var("GDAL_NUM_THREADS", "1");
        }
    }

    let args = Cli {
        src_path: vec!["assets/source_data/gebco.tif".into()],
        terrain_path: "assets/terrains/earth".into(),
        temp_path: None,
        overwrite: false,
        no_data: PreprocessNoData::Source,
        data_type: PreprocessDataType::DataType(GdalDataType::UInt16),
        fill_radius: 16.0,
        create_mask: true,
        lod_count: None,
        attachment_label: AttachmentLabel::Height,
        texture_size: 512,
        border_size: 2,
        mip_level_count: 1,
        format: AttachmentFormat::R16U,
    };

    let _args = Cli {
        src_path: vec!["assets/source_data/GRAYNEW6.tif".into()],
        terrain_path: "assets/terrains/earth".into(),
        temp_path: None,
        overwrite: true,
        no_data: PreprocessNoData::NoData(0.0),
        data_type: PreprocessDataType::DataType(GdalDataType::Float32),
        fill_radius: 0.0,
        create_mask: false,
        lod_count: None,
        attachment_label: AttachmentLabel::Height,
        texture_size: 512,
        border_size: 4,
        mip_level_count: 4,
        format: AttachmentFormat::R32F,
    };

    let (src_dataset, mut context) = PreprocessContext::from_cli(args).unwrap();

    preprocess(src_dataset, &mut context);

    let args = Cli {
        src_path: vec!["assets/source_data/NE2NEW4.tif".into()],
        terrain_path: "assets/terrains/earth".into(),
        temp_path: None,
        overwrite: true,
        no_data: PreprocessNoData::NoData(0.0),
        data_type: PreprocessDataType::DataType(GdalDataType::UInt8),
        fill_radius: 0.0,
        create_mask: false,
        lod_count: Some(4),
        attachment_label: AttachmentLabel::Custom("albedo".to_string()),
        texture_size: 512,
        border_size: 2,
        // mip_level_count: 1,
        // format: AttachmentFormat::RgbaU8,
        mip_level_count: 4,
        format: AttachmentFormat::Rgb8U,
    };

    let (src_dataset, mut context) = PreprocessContext::from_cli(args).unwrap();

    preprocess(src_dataset, &mut context);
}
