#[macro_export]
macro_rules! vector {
    ($x:expr, $y:expr) => {
        Vec2::new($x, $y)
    };
    ($x:expr, $y:expr, $z:expr) => {
        Vec3::new($x, $y, $z)
    };
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        Vec4::new($x, $y, $z, $w)
    };
    ($($other:tt)*) => {
        compile_error!("vec! macro supports only 2, 3 or 4 arguments")
    };
}
