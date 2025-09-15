use crate::config::Codec;
use image_webp::WebPEncoder;
use std::{
    fs,
    io::{BufReader, BufWriter},
    path::Path,
};

pub struct ImageConverter {}
impl ImageConverter {
    pub fn convert(src_path: &Path, dst_path: &Path, codec: &Codec) {
        let buf_reader = BufReader::new(fs::File::open(src_path).unwrap());
        let buf_writer = BufWriter::new(
            fs::File::create(Path::new(&format!("{}", dst_path.display()))).unwrap(),
        );
        let decoder = png::Decoder::new(buf_reader);
        let mut reader = decoder.read_info().unwrap();
        let mut img_buf = vec![0; reader.output_buffer_size().unwrap()];
        let img_info = reader.next_frame(&mut img_buf).unwrap();
        let icc_profile = reader.info().icc_profile.clone();
        let exif_metadata = reader.info().exif_metadata.clone();
        let itxt_chunks = reader.info().utf8_text.clone();

        match codec {
            Codec::WebP => {
                let mut encoder = WebPEncoder::new(buf_writer);
                let color_type = match img_info.color_type {
                    png::ColorType::Rgb => image_webp::ColorType::Rgb8,
                    png::ColorType::Rgba => image_webp::ColorType::Rgba8,
                    _ => panic!(),
                };
                if let Some(icc_profile) = icc_profile {
                    encoder.set_icc_profile(icc_profile.to_vec());
                }
                if let Some(exif_metadata) = exif_metadata {
                    encoder.set_exif_metadata(exif_metadata.to_vec());
                }
                for itxt_chunk in itxt_chunks {
                    encoder.set_xmp_metadata(itxt_chunk.get_text().unwrap().as_bytes().to_vec());
                    println!("XMP: {}", itxt_chunk.get_text().unwrap());
                }
                encoder
                    .encode(&img_buf, img_info.width, img_info.height, color_type)
                    .unwrap();
            }
            _ => {
                unimplemented!(
                    "これが表示されているなら、バグの可能性が高いです。GitHubにて報告してもらえると助かります。"
                )
            }
        }
    }
}
