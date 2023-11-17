use palette::Srgb;
use shark::{
    primitives::{color, checkerboard},
    shader::{
        create_shader_export, FragOne, FragThree, Fragment, IntoShader, ShaderExport, ShaderExt,
    },
};

#[no_mangle]
pub extern "C" fn shader_export() -> ShaderExport<'static, FragThree> {
    let flip_flop = (|frag: FragOne| {
        if frag.time() % 2.0 < 1.0 {
            Srgb::new(1.0, 0.0, 0.0)
        } else {
            Srgb::new(0.0, 1.0, 0.0)
        }
    })
    .into_shader();

    let shader = checkerboard(flip_flop.extrude().extrude(), color(Srgb::new(0.0, 0.0, 1.0)), 2.0);
    create_shader_export(shader)
}
