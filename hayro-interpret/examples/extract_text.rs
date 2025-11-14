use hayro_interpret::font::Glyph;
use hayro_interpret::{
    BlendMode, ClipPath, Context, Device, GlyphDrawMode, Image, InterpreterSettings, Paint,
    PathDrawMode, SoftMask, interpret_page,
};
use hayro_syntax::Pdf;
use hayro_syntax::object::dict::keys::{BASE_ENCODING, FONT_NAME, TYPE};
use hayro_syntax::object::{Name, Object};

use log::info;

use std::fmt::Write;

use kurbo::{Affine, BezPath, Point, Rect};
use std::path::PathBuf;
use std::sync::Arc;

fn main() {
    colog::default_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let args: Vec<String> = std::env::args().collect();
    let relative_path = args
        .get(1)
        .expect("Please provide a relative path to the PDF file as the first argument");
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    let data = std::fs::read(path).unwrap();

    let pdf = Pdf::new(Arc::new(data)).unwrap();

    let settings = InterpreterSettings::default();
    // Pass dummy values for bbox and initial transform, since we don't care about those.
    let mut context = Context::new(
        Affine::IDENTITY,
        Rect::new(0.0, 0.0, 1.0, 1.0),
        pdf.xref(),
        settings,
    );

    // Run everything!
    let page = &pdf.pages()[0];

    let mut extractor = TextExtractor::default();

    interpret_page(page, &mut context, &mut extractor);

    println!("{}", extractor.text);
}

#[derive(Default)]
struct TextExtractor {
    text: String,
}

/// Implement `Device` for `TextExtractor`. We extract Unicode text from glyphs.
impl Device<'_> for TextExtractor {
    fn set_soft_mask(&mut self, _: Option<SoftMask<'_>>) {}

    fn draw_path(&mut self, _: &BezPath, _: Affine, _: &Paint<'_>, _: &PathDrawMode) {}

    fn push_clip_path(&mut self, _: &ClipPath) {}

    fn push_transparency_group(&mut self, _: f32, _: Option<SoftMask<'_>>, _: BlendMode) {}

    fn draw_glyph(
        &mut self,
        glyph: &Glyph<'_>,
        _: Affine,
        _: Affine,
        _: &Paint<'_>,
        _: &GlyphDrawMode,
    ) {
        if let Some(unicode_char) = glyph.as_unicode() {
            write!(self.text, "{}", unicode_char).unwrap();
        } else {
            // Fallback for glyphs without Unicode mapping
            self.text.push('�'); // Replacement character
        }
    }

    fn pop_clip_path(&mut self) {}

    fn pop_transparency_group(&mut self) {}

    fn draw_image(&mut self, _: Image<'_, '_>, _: Affine) {}

    fn set_blend_mode(&mut self, _: BlendMode) {}
}
