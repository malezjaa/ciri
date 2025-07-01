use libm::{acosf, sqrtf};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use three_d::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vec3> for Vector3<f32> {
    fn from(vec: Vec3) -> Self {
        Vector3::new(vec.x, vec.y, vec.z)
    }
}

impl From<Vector3<f32>> for Vec3 {
    fn from(vec: Vector3<f32>) -> Self {
        Vec3::new(vec.x, vec.y, vec.z)
    }
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::new(0f32, 0f32, 0f32);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Calculates the dot product of two vector3.
    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        self.z.mul_add(rhs.z, self.x.mul_add(rhs.x, self.y * rhs.y))
    }

    /// Calculates length of the vector.
    #[inline]
    pub fn length(self) -> f32 {
        sqrtf(self.dot(self))
    }

    /// Calculates the squared length of the vector.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Calculates the angle between two vectors.
    #[inline]
    pub fn angle(self, rhs: Self) -> f32 {
        acosf(self.dot(rhs) / sqrtf(self.length_squared() * rhs.length_squared()))
    }

    /// The cross-product of the lhs and rhs vectors. This vector is usually not normalized.
    #[inline]
    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y.mul_add(rhs.z, -(rhs.y * self.z)),
            y: self.z.mul_add(rhs.x, -(rhs.z * self.x)),
            z: self.x.mul_add(rhs.y, -(rhs.x * self.y)),
        }
    }

    /// Calculates the distance between two three-dimensional points.
    #[inline]
    #[must_use]
    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length()
    }

    /// Interpolates between the points a and b by the interpolant t.
    #[inline]
    #[must_use]
    pub fn lerp(self, rhs: Self, t: f32) -> Self {
        self * (1.0 - t) + rhs * t
    }

    /// Returns a vector made from the largest components of two vectors.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: if self.x > rhs.x { self.x } else { rhs.x },
            y: if self.y > rhs.y { self.y } else { rhs.y },
            z: if self.z > rhs.z { self.z } else { rhs.z },
        }
    }

    /// Returns a vector made from the smallest components of two vectors.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: if self.x < rhs.x { self.x } else { rhs.x },
            y: if self.y < rhs.y { self.y } else { rhs.y },
            z: if self.z < rhs.z { self.z } else { rhs.z },
        }
    }
}

macro_rules! impl_vec3_ops {
    ($vec_type:ty) => {
        impl Add for $vec_type {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self {
                Self { x: self.x.add(rhs.x), y: self.y.add(rhs.y), z: self.z.add(rhs.z) }
            }
        }
        impl Add<&Self> for $vec_type {
            type Output = Self;

            #[inline]
            fn add(self, rhs: &Self) -> Self {
                self.add(*rhs)
            }
        }
        impl Add<&$vec_type> for &$vec_type {
            type Output = $vec_type;

            #[inline]
            fn add(self, rhs: &$vec_type) -> $vec_type {
                (*self).add(*rhs)
            }
        }
        impl Add<$vec_type> for &$vec_type {
            type Output = $vec_type;

            #[inline]
            fn add(self, rhs: $vec_type) -> $vec_type {
                (*self).add(rhs)
            }
        }
        impl AddAssign for $vec_type {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.x.add_assign(rhs.x);
                self.y.add_assign(rhs.y);
                self.z.add_assign(rhs.z);
            }
        }
        impl AddAssign<&Self> for $vec_type {
            #[inline]
            fn add_assign(&mut self, rhs: &Self) {
                self.add_assign(*rhs);
            }
        }

        impl Sub for $vec_type {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                Self { x: self.x.sub(rhs.x), y: self.y.sub(rhs.y), z: self.z.sub(rhs.z) }
            }
        }
        impl Sub<&Self> for $vec_type {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: &Self) -> Self {
                self.sub(*rhs)
            }
        }
        impl Sub<&$vec_type> for &$vec_type {
            type Output = $vec_type;

            #[inline]
            fn sub(self, rhs: &$vec_type) -> $vec_type {
                (*self).sub(*rhs)
            }
        }
        impl Sub<$vec_type> for &$vec_type {
            type Output = $vec_type;

            #[inline]
            fn sub(self, rhs: $vec_type) -> $vec_type {
                (*self).sub(rhs)
            }
        }
        impl SubAssign for $vec_type {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.x.sub_assign(rhs.x);
                self.y.sub_assign(rhs.y);
                self.z.sub_assign(rhs.z);
            }
        }
        impl SubAssign<&Self> for $vec_type {
            #[inline]
            fn sub_assign(&mut self, rhs: &Self) {
                self.sub_assign(*rhs);
            }
        }

        impl Mul for $vec_type {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self {
                Self { x: self.x.mul(rhs.x), y: self.y.mul(rhs.y), z: self.z.mul(rhs.z) }
            }
        }
        impl Mul<&Self> for $vec_type {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: &Self) -> Self {
                self.mul(*rhs)
            }
        }
        impl Mul<&$vec_type> for &$vec_type {
            type Output = $vec_type;

            #[inline]
            fn mul(self, rhs: &$vec_type) -> $vec_type {
                (*self).mul(*rhs)
            }
        }
        impl Mul<$vec_type> for &$vec_type {
            type Output = $vec_type;

            #[inline]
            fn mul(self, rhs: $vec_type) -> $vec_type {
                (*self).mul(rhs)
            }
        }
        impl MulAssign for $vec_type {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.x.mul_assign(rhs.x);
                self.y.mul_assign(rhs.y);
                self.z.mul_assign(rhs.z);
            }
        }
        impl MulAssign<&Self> for $vec_type {
            #[inline]
            fn mul_assign(&mut self, rhs: &Self) {
                self.mul_assign(*rhs);
            }
        }

        impl Mul<f32> for $vec_type {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: f32) -> Self {
                Self { x: self.x.mul(rhs), y: self.y.mul(rhs), z: self.z.mul(rhs) }
            }
        }
        impl Mul<&f32> for $vec_type {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: &f32) -> Self {
                self.mul(*rhs)
            }
        }
        impl Mul<&f32> for &$vec_type {
            type Output = $vec_type;

            #[inline]
            fn mul(self, rhs: &f32) -> $vec_type {
                (*self).mul(*rhs)
            }
        }
        impl Mul<f32> for &$vec_type {
            type Output = $vec_type;

            #[inline]
            fn mul(self, rhs: f32) -> $vec_type {
                (*self).mul(rhs)
            }
        }
        impl MulAssign<f32> for $vec_type {
            #[inline]
            fn mul_assign(&mut self, rhs: f32) {
                self.x.mul_assign(rhs);
                self.y.mul_assign(rhs);
                self.z.mul_assign(rhs);
            }
        }
        impl MulAssign<&f32> for $vec_type {
            #[inline]
            fn mul_assign(&mut self, rhs: &f32) {
                self.mul_assign(*rhs);
            }
        }

        // Scalar * Vector
        impl Mul<$vec_type> for f32 {
            type Output = $vec_type;

            #[inline]
            fn mul(self, rhs: $vec_type) -> $vec_type {
                <$vec_type>::new(self.mul(rhs.x), self.mul(rhs.y), self.mul(rhs.z))
            }
        }
        impl Mul<&$vec_type> for f32 {
            type Output = $vec_type;

            #[inline]
            fn mul(self, rhs: &$vec_type) -> $vec_type {
                self.mul(*rhs)
            }
        }
        impl Mul<&$vec_type> for &f32 {
            type Output = $vec_type;

            #[inline]
            fn mul(self, rhs: &$vec_type) -> $vec_type {
                (*self).mul(*rhs)
            }
        }
        impl Mul<$vec_type> for &f32 {
            type Output = $vec_type;

            #[inline]
            fn mul(self, rhs: $vec_type) -> $vec_type {
                (*self).mul(rhs)
            }
        }

        impl Div for $vec_type {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self {
                Self { x: self.x.div(rhs.x), y: self.y.div(rhs.y), z: self.z.div(rhs.z) }
            }
        }
        impl Div<&Self> for $vec_type {
            type Output = Self;

            #[inline]
            fn div(self, rhs: &Self) -> Self {
                self.div(*rhs)
            }
        }
        impl Div<&$vec_type> for &$vec_type {
            type Output = $vec_type;

            #[inline]
            fn div(self, rhs: &$vec_type) -> $vec_type {
                (*self).div(*rhs)
            }
        }
        impl Div<$vec_type> for &$vec_type {
            type Output = $vec_type;

            #[inline]
            fn div(self, rhs: $vec_type) -> $vec_type {
                (*self).div(rhs)
            }
        }
        impl DivAssign for $vec_type {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                self.x.div_assign(rhs.x);
                self.y.div_assign(rhs.y);
                self.z.div_assign(rhs.z);
            }
        }
        impl DivAssign<&Self> for $vec_type {
            #[inline]
            fn div_assign(&mut self, rhs: &Self) {
                self.div_assign(*rhs);
            }
        }

        impl Div<f32> for $vec_type {
            type Output = Self;

            #[inline]
            fn div(self, rhs: f32) -> Self {
                Self { x: self.x.div(rhs), y: self.y.div(rhs), z: self.z.div(rhs) }
            }
        }
        impl Div<&f32> for $vec_type {
            type Output = Self;

            #[inline]
            fn div(self, rhs: &f32) -> Self {
                self.div(*rhs)
            }
        }
        impl Div<&f32> for &$vec_type {
            type Output = $vec_type;

            #[inline]
            fn div(self, rhs: &f32) -> $vec_type {
                (*self).div(*rhs)
            }
        }
        impl Div<f32> for &$vec_type {
            type Output = $vec_type;

            #[inline]
            fn div(self, rhs: f32) -> $vec_type {
                (*self).div(rhs)
            }
        }
        impl DivAssign<f32> for $vec_type {
            #[inline]
            fn div_assign(&mut self, rhs: f32) {
                self.x.div_assign(rhs);
                self.y.div_assign(rhs);
                self.z.div_assign(rhs);
            }
        }
        impl DivAssign<&f32> for $vec_type {
            #[inline]
            fn div_assign(&mut self, rhs: &f32) {
                self.div_assign(*rhs);
            }
        }

        impl Div<$vec_type> for f32 {
            type Output = $vec_type;

            #[inline]
            fn div(self, rhs: $vec_type) -> $vec_type {
                <$vec_type>::new(self.div(rhs.x), self.div(rhs.y), self.div(rhs.z))
            }
        }
        impl Div<&$vec_type> for f32 {
            type Output = $vec_type;

            #[inline]
            fn div(self, rhs: &$vec_type) -> $vec_type {
                self.div(*rhs)
            }
        }
        impl Div<&$vec_type> for &f32 {
            type Output = $vec_type;

            #[inline]
            fn div(self, rhs: &$vec_type) -> $vec_type {
                (*self).div(*rhs)
            }
        }
        impl Div<$vec_type> for &f32 {
            type Output = $vec_type;

            #[inline]
            fn div(self, rhs: $vec_type) -> $vec_type {
                (*self).div(rhs)
            }
        }

        impl Neg for $vec_type {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                Self { x: self.x.neg(), y: self.y.neg(), z: self.z.neg() }
            }
        }
        impl Neg for &$vec_type {
            type Output = $vec_type;

            #[inline]
            fn neg(self) -> $vec_type {
                (*self).neg()
            }
        }
    };
}

impl_vec3_ops!(Vec3);
