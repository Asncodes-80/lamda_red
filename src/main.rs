mod export;
mod parsing;

#[derive(Clone, Copy)]
pub struct SurfaceSettings {
    width: f64,
    height: f64,
    shape_count: i32,
}

#[derive(Clone, Copy)]
pub struct ShapeSettings {
    width: i32,
    height: i32,
}

fn main() {
    let mut default_page_settings: SurfaceSettings = SurfaceSettings {
        width: 400.0,
        height: 600.0,
        shape_count: 6,
    };

    let default_shape_settings: ShapeSettings = ShapeSettings {
        width: 120,
        height: 60,
    };

    let lines =
        parsing::read_input("sample.zz", default_page_settings, default_shape_settings).unwrap();

    for _ in 0..(lines as i32) - default_page_settings.shape_count {
        default_page_settings.height += default_shape_settings.height as f64 + 70.0;
    }
    export::convert_to_png("output.xml", default_page_settings);
}
