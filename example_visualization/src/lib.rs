use shark::{
    primitives::{color, off},
    shader::{create_shader_export, FragThree, ShaderExport, ShaderExt},
};

#[no_mangle]
pub extern "C" fn shader_export() -> ShaderExport<'static, FragThree> {
    let shader = color::<FragThree>(palette::Srgb::new(1.0, 0.0, 1.0)).checkerboard(off(), 2.0);

    create_shader_export(shader)
}
