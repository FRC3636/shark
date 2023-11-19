use palette::{IntoColor, LinSrgb};
use shark::point::Point;
use shark::shader::{Fragment, Shader};
#[cfg(target_arch = "wasm32")]
use shark::shader::FragThree;

#[repr(C)]
#[derive(Clone)]
pub struct ShaderExport<'a, F: Fragment> {
    shader: *const (),
    f: &'a extern "C" fn(*const (), F) -> LinSrgb<f64>,
}
// f will always be Send even when F isn't, and the only way to create shader is when it points to a static shader implementing Send
unsafe impl<'a, F: Fragment + Send> Send for ShaderExport<'a, F> {}

pub fn create_shader_export<'a, S: Shader<F> + 'a + Send, F: Fragment>(
    shader: S,
) -> ShaderExport<'a, F> {
    let shader_ptr = (Box::leak(Box::new(shader)) as *const S).cast();

    extern "C" fn shader_export_fn<S: Shader<F>, F: Fragment>(
        shader: *const (),
        frag: F,
    ) -> LinSrgb<f64> {
        let shader = unsafe { &*(shader.cast::<S>()) };
        shader.shade(frag).into_color()
    }

    ShaderExport {
        shader: shader_ptr,
        f: &(shader_export_fn::<S, F> as extern "C" fn(*const (), F) -> LinSrgb<f64>),
    }
}

impl<F: Fragment> Shader<F> for ShaderExport<'static, F> {
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: F) -> Self::Output {
        (self.f)(self.shader, frag)
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct PointsExport<'a> {
    _marker: std::marker::PhantomData<&'a Point>,
    points: *const Point,
    len: usize,
}
impl PointsExport<'_> {
    pub fn as_slice(&self) -> &[Point] {
        unsafe { std::slice::from_raw_parts(self.points, self.len) }
    }
}
unsafe impl Send for PointsExport<'_> {}

impl<'a> From<&'a [Point]> for PointsExport<'a> {
    fn from(points: &'a [Point]) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            points: points.as_ptr(),
            len: points.len(),
        }
    }
}

impl From<Vec<Point>> for PointsExport<'_> {
    fn from(value: Vec<Point>) -> Self {
        let arr = value.into_boxed_slice();
        Self {
            _marker: std::marker::PhantomData,
            points: arr.as_ptr(),
            len: arr.len(),
        }
    }
}

// This probably shouldn't be used outside of exporting to the visualizer.
impl FromIterator<Point> for PointsExport<'static> {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let points = iter.into_iter().collect::<Vec<_>>();
        let len = points.len();
        let points = Box::leak(Box::new(points)).as_ptr();
        Self {
            _marker: std::marker::PhantomData,
            points,
            len,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct VisualizationExports<'a, F: Fragment> {
    pub shader: ShaderExport<'a, F>,
    pub points: PointsExport<'a>,
}
impl<'a, F: Fragment> VisualizationExports<'a, F> {
    pub fn new<S: Shader<F> + Send + 'a>(shader: S, points: impl Iterator<Item = Point>) -> Self {
        Self {
            shader: create_shader_export(shader),
            points: points.collect::<Vec<_>>().into(),
        }
    }
}
