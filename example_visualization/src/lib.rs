use palette::{IntoColor, Srgb};
use shark::{
    point::{Point, primitives::Line},
    shader::{
        primitives::{checkerboard, color, off},
        FragOne, FragThree, Fragment, IntoShader, Shader, ShaderExt,
    },
};
use shark_visualizer_interface::VisualizationExports;

#[no_mangle]
pub extern "C" fn vis_exports() -> VisualizationExports<'static, FragThree> {
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
    let points = vec![Point::new(0.0, 1.0, 0.0)].into_iter();

    VisualizationExports::new(shader, points)
}
