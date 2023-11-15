// use palette::Oklab;
// use shark::{primitives::{color, off}, shader::{create_vtable_shader, VtableShader, FragThree, ShaderExt}};

// pub extern "C" fn shader_export() -> VtableShader<FragThree, Oklab> {
//     let mut shader = color::<FragThree>(palette::Srgb::new(1.0, 0.0, 1.0)).checkerboard(off(), 2);

//     create_vtable_shader(Box::leak(Box::new(shader)))
// }