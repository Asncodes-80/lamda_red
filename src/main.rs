mod export;
mod parsing;

#[derive(Clone, Copy)]
pub struct SurfaceSettings {
    width: i32,
    height: i32,
    shape_count: i32,
}

#[derive(Clone, Copy)]
pub struct ShapeSettings {
    width: i32,
    height: i32,
}

fn main() {
    let mut default_page_settings: SurfaceSettings = SurfaceSettings {
        width: 800,
        height: 600,
        shape_count: 6,
    };

    let default_shape_settings: ShapeSettings = ShapeSettings {
        width: 120,
        height: 60,
    };

    let line =
        parsing::read_input("sample.zz", default_page_settings, default_shape_settings).unwrap();
    println!("{}", line);
    // if line > 6 && line < 24 {
    //     let t = (line as i32) - default_page_settings.shape_count;

    //     for i in 0..t {
    //         default_page_settings.height += default_shape_settings.height;
    //         println!("{}: {}", i, default_page_settings.height);
    //     }
    // }
    export::convert_to_png("output.xml", default_page_settings);
}
