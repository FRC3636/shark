use palette::{IntoColor, Srgb};
use shark::{
    point::Point,
    shader::{
        create_shader_export,
        primitives::{checkerboard, color, off},
        FragOne, FragThree, Fragment, IntoShader, Shader, ShaderExt,
    },
    VisualizationExports,
};

#[no_mangle]
pub extern "C" fn shader_export() -> VisualizationExports<FragThree> {
    let flip_flop = (|frag: FragOne| {
        if frag.time() % 2.0 < 1.0 {
            Srgb::new(1.0, 0.0, 0.0)
        } else {
            off().shade(frag).into_color()
        }
    })
    .into_shader();

    let shader = checkerboard(
        flip_flop.extrude().extrude(),
        color(Srgb::new(0.0, 0.0, 1.0)),
        2.0,
    );
    // let shader = off();
    let points: &'static [Point] = Box::leak(Box::new([]));

    VisualizationExports::new(create_shader_export(shader), points.into())
}
