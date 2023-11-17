use palette::{Srgb, IntoColor};
use shark::{
    primitives::{checkerboard, color, off},
    shader::{
        create_shader_export, FragOne, FragThree, Fragment, IntoShader, ShaderExport, ShaderExt, Shader,
    },
};

#[no_mangle]
pub extern "C" fn shader_export() -> ShaderExport<'static, FragThree> {
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
    create_shader_export(shader)
}
