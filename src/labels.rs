use cosmic_text::{
    Align, Attrs, Buffer, Color, FontSystem, LayoutRun, Metrics, Shaping, SwashCache, Wrap,
};
use tiny_skia::{Paint, Pixmap, Rect, Transform};
use unicode_segmentation::UnicodeSegmentation;

pub struct Label {
    pub text: String,
    pub width: f32,
    pub height: f32,
    pub font_size: f32,
    buffer: Buffer,
}

pub struct Fonts {
    system: FontSystem,
    cache: SwashCache,
}

impl Fonts {
    pub fn new() -> Result<Self, &'static str> {
        let system = FontSystem::new();
        if system.db().faces().next().is_none() {
            return Err("no system fonts found. Install a sans-serif font to render labels");
        }
        Ok(Self {
            system,
            cache: SwashCache::new(),
        })
    }

    pub fn layout(
        &mut self,
        text: &str,
        font_size: f32,
        max_width: f32,
        max_lines: usize,
    ) -> Label {
        let metrics = Metrics::new(font_size, font_size * 1.3);
        let max_height = metrics.line_height * (max_lines + 1) as f32;
        let mut wrapped = Buffer::new(&mut self.system, metrics);
        wrapped.set_wrap(Wrap::WordOrGlyph);
        wrapped.set_size(Some(max_width), Some(max_height));
        wrapped.set_text(text, &Attrs::new(), Shaping::Advanced, None);
        wrapped.shape_until_scroll(&mut self.system, false);

        let mut lines = wrapped
            .layout_runs()
            .take(max_lines + 1)
            .map(run_text)
            .collect::<Vec<_>>();
        let mut shortened = String::new();
        if lines.len() > max_lines {
            lines.truncate(max_lines);
            let mut prefix = lines[max_lines - 1].trim_end();
            loop {
                shortened.clear();
                shortened.push_str(prefix);
                shortened.push_str("...");
                let measured = self.buffer(&shortened, metrics, None);
                if text_width(&measured) <= max_width || prefix.is_empty() {
                    break;
                }
                let last = prefix.grapheme_indices(true).next_back().unwrap().0;
                prefix = prefix[..last].trim_end();
            }
            lines[max_lines - 1] = &shortened;
        }
        let text = lines.join("\n");
        let unaligned = self.buffer(&text, metrics, None);
        let width = text_width(&unaligned).ceil();
        let height = metrics.line_height * lines.len() as f32;
        let buffer = self.buffer(&text, metrics, Some(width));
        Label {
            text,
            width,
            height,
            font_size,
            buffer,
        }
    }

    fn buffer(&mut self, text: &str, metrics: Metrics, width: Option<f32>) -> Buffer {
        let mut buffer = Buffer::new(&mut self.system, metrics);
        buffer.set_wrap(Wrap::None);
        buffer.set_size(width, None);
        buffer.set_text(text, &Attrs::new(), Shaping::Advanced, Some(Align::Center));
        buffer.shape_until_scroll(&mut self.system, false);
        buffer
    }

    pub fn draw(&mut self, pixmap: &mut Pixmap, label: &mut Label, x: f32, y: f32, color: Color) {
        let mut paint = Paint::default();
        let x = x.round();
        let y = y.round();
        label.buffer.draw(
            &mut self.system,
            &mut self.cache,
            color,
            |px, py, width, height, color| {
                if let Some(rect) =
                    Rect::from_xywh(x + px as f32, y + py as f32, width as f32, height as f32)
                {
                    paint.set_color_rgba8(color.r(), color.g(), color.b(), color.a());
                    pixmap.fill_rect(rect, &paint, Transform::identity(), None);
                }
            },
        );
    }
}

fn run_text<'a>(run: LayoutRun<'a>) -> &'a str {
    let start = run.glyphs.iter().map(|glyph| glyph.start).min();
    let end = run.glyphs.iter().map(|glyph| glyph.end).max();
    match start.zip(end) {
        Some((start, end)) => run.text[start..end].trim(),
        None => "",
    }
}

fn text_width(buffer: &Buffer) -> f32 {
    buffer
        .layout_runs()
        .map(|run| run.line_w)
        .fold(0.0, f32::max)
}

#[cfg(test)]
mod tests {
    use super::Fonts;

    #[test]
    fn wraps_and_caps_labels_including_unbroken_words() {
        let mut fonts = Fonts::new().unwrap();
        let short = fonts.layout("Short", 15.0, 160.0, 4);
        assert_eq!(short.text, "Short");

        for text in [
            "a long label with several words ".repeat(25),
            "X".repeat(300),
        ] {
            let label = fonts.layout(&text, 15.0, 160.0, 4);
            assert!(label.width <= 160.0);
            assert!(label.height > short.height);
            assert_eq!(label.text.lines().count(), 4);
            assert!(label.text.ends_with("..."));
        }
    }

    #[test]
    fn shapes_utf8_latin_arabic_persian_and_mixed_labels() {
        let mut fonts = Fonts::new().unwrap();
        // Each fixture names its script and expected paragraph direction.
        let cases = [
            ("English Latin", "Train speed", false),
            ("Accented Latin", "Vitesse \u{e9}lev\u{e9}e", false),
            ("Arabic", "\u{633}\u{631}\u{639}\u{629}", true),
            ("Persian", "\u{633}\u{631}\u{639}\u{62a}", true),
            (
                "Mixed Latin and Persian",
                "Train \u{67e}\u{6cc}\u{634} 42",
                false,
            ),
        ];
        for (script, text, rtl) in cases {
            let multiline = format!("{text}\n{text}");
            let label = fonts.layout(&multiline, 15.0, 160.0, 4);
            assert_eq!(label.text, multiline, "{script}: preserve UTF-8 text");
            let runs = label.buffer.layout_runs().collect::<Vec<_>>();
            assert_eq!(runs.len(), 2, "{script}: preserve line breaks");
            for run in runs {
                assert_eq!(run.rtl, rtl, "{script}: paragraph direction");
                assert!(!run.glyphs.is_empty(), "{script}: shaped glyphs");
                assert!(
                    run.glyphs.iter().all(|glyph| glyph.glyph_id != 0),
                    "{script}: font fallback must resolve every glyph"
                );
            }
        }
    }
}
