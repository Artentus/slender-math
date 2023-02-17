//! Lightweight math library for game development

#![feature(portable_simd)]
#![deny(missing_docs)]

use std::fmt::Debug;
use std::fmt::Display;

#[rustfmt::skip]
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut,
    Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use std::simd::Which::*;
use std::simd::*;

macro_rules! simd_swizzle_1 {
    ($v:expr, $x:literal) => {
        simd_swizzle!($v, [$x, $x, $x, $x])
    };
}

macro_rules! simd_swizzle_0101 {
    ($a:expr, $b:expr) => {
        simd_swizzle!($a, $b, [First(0), First(1), Second(0), Second(1)])
    };
}

macro_rules! simd_swizzle_2323 {
    ($a:expr, $b:expr) => {
        simd_swizzle!($a, $b, [First(2), First(3), Second(2), Second(3)])
    };
}

macro_rules! def_field {
    ($name:ident, $name_mut:ident, $i:literal, $t:ty) => {
        #[doc = concat!("The ", stringify!($name), " component of the vector")]
        #[inline]
        pub const fn $name(&self) -> $t {
            self.0.as_array()[$i]
        }

        #[doc = concat!("The ", stringify!($name), " component of the vector")]
        #[inline]
        pub fn $name_mut(&mut self) -> &mut $t {
            self.0.index_mut($i)
        }
    };
}

/// A vector with 2 f32 components
#[derive(Clone, Copy, PartialEq)]
#[repr(C, align(8))]
pub struct Vector2f(f32x2);
impl Vector2f {
    /// The vector (0, 0)
    pub const ZERO: Self = Self::new(0.0, 0.0);
    /// The vector (1, 1)
    pub const ONE: Self = Self::new(1.0, 1.0);
    /// The vector (1, 0)
    pub const UNIT_X: Self = Self::new(1.0, 0.0);
    /// The vector (0, 1)
    pub const UNIT_Y: Self = Self::new(0.0, 1.0);

    def_field!(x, x_mut, 0, f32);
    def_field!(y, y_mut, 1, f32);

    #[cfg(feature = "color_fields")]
    def_field!(r, r_mut, 0, f32);
    #[cfg(feature = "color_fields")]
    def_field!(g, g_mut, 1, f32);

    /// Creates a new vector from the given components
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self(f32x2::from_array([x, y]))
    }

    /// Creates a new vector by setting all components to the given scalar
    #[inline]
    pub const fn from_scalar(scalar: f32) -> Self {
        Self(f32x2::from_array([scalar; 2]))
    }

    /// Creates a new vector from the given array
    #[inline]
    pub const fn from_array(array: [f32; 2]) -> Self {
        Self(f32x2::from_array(array))
    }

    /// Converts the vector into an array
    #[inline]
    pub const fn to_array(&self) -> [f32; 2] {
        self.0.to_array()
    }

    /// Returns an array reference to the vector
    #[inline]
    pub const fn as_array(&self) -> &[f32; 2] {
        self.0.as_array()
    }

    /// Returns a mutable array reference to the vector
    #[inline]
    pub fn as_mut_array(&mut self) -> &mut [f32; 2] {
        self.0.as_mut_array()
    }

    #[inline]
    const fn from_simd_truncate(simd_vec: f32x2) -> Self {
        Self(simd_vec)
    }

    /// Calculates the cross product between this vector and rhs by setting the Z components to 0
    /// and returns the magnitude of the resulting vector
    #[inline]
    pub fn cross(self, rhs: Self) -> f32 {
        let prod = self * rhs.yx();
        prod.0[0] - prod.0[1]
    }
}
impl Debug for Vector2f {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2f({}, {})", self.x(), self.y())
    }
}
impl Display for Vector2f {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

/// A vector with 3 f32 components
#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct Vector3f(f32x4);
impl Vector3f {
    /// The vector (0, 0, 0)
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    /// The vector (1, 1, 1)
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0);
    /// The vector (1, 0, 0)
    pub const UNIT_X: Self = Self::new(1.0, 0.0, 0.0);
    /// The vector (0, 1, 0)
    pub const UNIT_Y: Self = Self::new(0.0, 1.0, 0.0);
    /// The vector (0, 0, 1)
    pub const UNIT_Z: Self = Self::new(0.0, 0.0, 1.0);

    def_field!(x, x_mut, 0, f32);
    def_field!(y, y_mut, 1, f32);
    def_field!(z, z_mut, 2, f32);

    #[cfg(feature = "color_fields")]
    def_field!(r, r_mut, 0, f32);
    #[cfg(feature = "color_fields")]
    def_field!(g, g_mut, 1, f32);
    #[cfg(feature = "color_fields")]
    def_field!(b, b_mut, 2, f32);

    /// Creates a new vector from the given components
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(f32x4::from_array([x, y, z, 0.0]))
    }

    /// Creates a new vector by setting all components to the given scalar
    #[inline]
    pub const fn from_scalar(scalar: f32) -> Self {
        Self(f32x4::from_array([scalar, scalar, scalar, 0.0]))
    }

    /// Creates a new vector from the given array
    #[inline]
    pub const fn from_array(array: [f32; 3]) -> Self {
        Self(f32x4::from_array([array[0], array[1], array[2], 0.0]))
    }

    /// Creates a new vector from the given 2-component vector
    #[inline]
    pub const fn from_v2f(v: v2f, z: f32) -> Self {
        Self(f32x4::from_array([v.x(), v.y(), z, 0.0]))
    }

    /// Converts the vector into an array
    #[inline]
    pub const fn to_array(&self) -> [f32; 3] {
        let array: [f32; 4] = self.0.to_array();
        [array[0], array[1], array[2]]
    }

    /// Returns an array reference to the vector
    #[inline]
    pub const fn as_array(&self) -> &[f32; 3] {
        let a: &[f32; 4] = self.0.as_array();
        unsafe { std::mem::transmute(a) }
    }

    /// Returns a mutable array reference to the vector
    #[inline]
    pub fn as_mut_array(&mut self) -> &mut [f32; 3] {
        let a: &mut [f32; 4] = self.0.as_mut_array();
        unsafe { std::mem::transmute(a) }
    }

    #[inline]
    fn from_simd_truncate(simd_vec: f32x4) -> Self {
        let zero = f32x4::splat(0.0);
        let mask = mask32x4::from_array([true, true, true, false]);
        Self(mask.select(simd_vec, zero))
    }

    /// Calculates the cross product between this vector and rhs
    pub fn cross(self, rhs: Self) -> Self {
        // Algorithm from: https://geometrian.com/programming/tutorials/cross-product/index.php

        let tmp0 = simd_swizzle!(self.0, [1, 2, 0, 3]);
        let tmp1 = simd_swizzle!(rhs.0, [2, 0, 1, 3]);
        let tmp2 = tmp0 * rhs.0;
        let tmp3 = tmp0 * tmp1;
        let tmp4 = simd_swizzle!(tmp2, [1, 2, 0, 3]);
        Self(tmp3 - tmp4)
    }
}
impl Debug for Vector3f {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector3f({}, {}, {})", self.x(), self.y(), self.z())
    }
}
impl Display for Vector3f {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}
impl PartialEq for Vector3f {
    fn eq(&self, other: &Self) -> bool {
        (self.0.as_array()[0] == other.0.as_array()[0])
            && (self.0.as_array()[1] == other.0.as_array()[1])
            && (self.0.as_array()[2] == other.0.as_array()[2])
    }
}

/// A vector with 4 f32 components
#[derive(Clone, Copy, PartialEq)]
#[repr(C, align(16))]
pub struct Vector4f(f32x4);
impl Vector4f {
    /// The vector (0, 0, 0, 0)
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    /// The vector (1, 1, 1, 1)
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    /// The vector (1, 0, 0, 0)
    pub const UNIT_X: Self = Self::new(1.0, 0.0, 0.0, 0.0);
    /// The vector (0, 1, 0, 0)
    pub const UNIT_Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);
    /// The vector (0, 0, 1, 0)
    pub const UNIT_Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);
    /// The vector (0, 0, 0, 1)
    pub const UNIT_W: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    def_field!(x, x_mut, 0, f32);
    def_field!(y, y_mut, 1, f32);
    def_field!(z, z_mut, 2, f32);
    def_field!(w, w_mut, 3, f32);

    #[cfg(feature = "color_fields")]
    def_field!(r, r_mut, 0, f32);
    #[cfg(feature = "color_fields")]
    def_field!(g, g_mut, 1, f32);
    #[cfg(feature = "color_fields")]
    def_field!(b, b_mut, 2, f32);
    #[cfg(feature = "color_fields")]
    def_field!(a, a_mut, 3, f32);

    /// Creates a new vector from the given components
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(f32x4::from_array([x, y, z, w]))
    }

    /// Creates a new vector by setting all components to the given scalar
    #[inline]
    pub const fn from_scalar(scalar: f32) -> Self {
        Self(f32x4::from_array([scalar; 4]))
    }

    /// Creates a new vector from the given array
    #[inline]
    pub const fn from_array(array: [f32; 4]) -> Self {
        Self(f32x4::from_array(array))
    }

    /// Creates a new vector from the given 2-component vector
    #[inline]
    pub const fn from_v2f(v: v2f, z: f32, w: f32) -> Self {
        Self(f32x4::from_array([v.x(), v.y(), z, w]))
    }

    /// Creates a new vector from the given 3-component vector
    #[inline]
    pub const fn from_v3f(v: v3f, w: f32) -> Self {
        Self(f32x4::from_array([v.x(), v.y(), v.z(), w]))
    }

    /// Converts the vector into an array
    #[inline]
    pub const fn to_array(&self) -> [f32; 4] {
        self.0.to_array()
    }

    /// Returns an array reference to the vector
    #[inline]
    pub const fn as_array(&self) -> &[f32; 4] {
        self.0.as_array()
    }

    /// Returns a mutable array reference to the vector
    #[inline]
    pub fn as_mut_array(&mut self) -> &mut [f32; 4] {
        self.0.as_mut_array()
    }

    #[inline]
    const fn from_simd_truncate(simd_vec: f32x4) -> Self {
        Self(simd_vec)
    }
}
impl Debug for Vector4f {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Vector4f({}, {}, {}, {})",
            self.x(),
            self.y(),
            self.z(),
            self.w()
        )
    }
}
impl Display for Vector4f {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.x(),
            self.y(),
            self.z(),
            self.w()
        )
    }
}

macro_rules! impl_common_f {
    ($t:ty, $ts:ty) => {
        impl $t {
            /// Returns a vector with each component set to the absolute value of the corresponding component in this vector
            #[inline]
            pub fn abs(self) -> Self {
                Self(self.0.abs())
            }

            /// Returns a vector with each component set to the reciprocal of the corresponding component in this vector
            #[inline]
            pub fn recip(self) -> Self {
                Self::from_simd_truncate(self.0.recip())
            }

            /// Returns a vector with each component set to the floor of the corresponding component in this vector
            #[inline]
            pub fn floor(self) -> Self {
                Self(self.0.floor())
            }

            /// Returns a vector with each component set to the ceiling of the corresponding component in this vector
            #[inline]
            pub fn ceil(self) -> Self {
                Self(self.0.ceil())
            }

            /// Returns a vector with each component set to the fractional part of the corresponding component in this vector
            #[inline]
            pub fn fract(self) -> Self {
                Self(self.0.fract())
            }

            /// Calculates the dot product between this vector and rhs
            #[inline]
            pub fn dot(self, rhs: Self) -> f32 {
                let prod = self.0 * rhs.0;
                prod.reduce_sum()
            }

            /// The length of this vector squared
            #[inline]
            pub fn len2(self) -> f32 {
                Self::dot(self, self)
            }

            /// The length of this vector
            #[inline]
            pub fn len(self) -> f32 {
                self.len2().sqrt()
            }

            /// Normalizes the vector
            #[inline]
            pub fn normalized(self) -> Self {
                let len = self.len();
                if len == 0.0 {
                    self
                } else {
                    self / self.len()
                }
            }

            /// Linearily interpolates between this vector and rhs
            #[inline]
            pub fn lerp(self, rhs: Self, t: f32) -> Self {
                self + ((rhs - self) * t)
            }

            /// Calculates the distance between this vector and rhs squared
            #[inline]
            pub fn dist2(self, b: Self) -> f32 {
                (b - self).len2()
            }

            /// Calculates the distance between this vector and rhs
            #[inline]
            pub fn dist(self, b: Self) -> f32 {
                (b - self).len()
            }

            /// Returns a vector with each component set to the minimum of the corresponding components between this vector and rhs
            #[inline]
            pub fn min(self, rhs: Self) -> Self {
                Self(<$ts>::simd_min(self.0, rhs.0))
            }

            /// Returns a vector with each component set to the maximum of the corresponding components between this vector and rhs
            #[inline]
            pub fn max(self, rhs: Self) -> Self {
                Self(<$ts>::simd_max(self.0, rhs.0))
            }

            /// Calculates (self * a) + b in one operation
            #[inline]
            pub fn mul_add(self, a: Self, b: Self) -> Self {
                Self(<$ts>::mul_add(self.0, a.0, b.0))
            }
        }
    };
}

impl_common_f!(Vector2f, f32x2);
impl_common_f!(Vector3f, f32x4);
impl_common_f!(Vector4f, f32x4);

/// A vector with 2 i32 components
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C, align(8))]
pub struct Vector2i(i32x2);
impl Vector2i {
    /// The vector (0, 0)
    pub const ZERO: Self = Self::new(0, 0);

    def_field!(x, x_mut, 0, i32);
    def_field!(y, y_mut, 1, i32);

    /// Creates a new vector from the given components
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self(i32x2::from_array([x, y]))
    }

    /// Creates a new vector by setting all components to the given scalar
    #[inline]
    pub const fn from_scalar(scalar: i32) -> Self {
        Self(i32x2::from_array([scalar; 2]))
    }

    /// Creates a new vector from the given array
    #[inline]
    pub const fn from_array(array: [i32; 2]) -> Self {
        Self(i32x2::from_array(array))
    }

    /// Converts the vector into an array
    #[inline]
    pub const fn to_array(&self) -> [i32; 2] {
        self.0.to_array()
    }

    /// Casts this vector into a floating point vector
    #[inline]
    pub fn to_float(&self) -> Vector2f {
        Vector2f(self.0.cast())
    }

    /// Returns an array reference to the vector
    #[inline]
    pub const fn as_array(&self) -> &[i32; 2] {
        self.0.as_array()
    }

    /// Returns a mutable array reference to the vector
    #[inline]
    pub fn as_mut_array(&mut self) -> &mut [i32; 2] {
        self.0.as_mut_array()
    }

    #[inline]
    const fn from_simd_truncate(simd_vec: i32x2) -> Self {
        Self(simd_vec)
    }
}
impl Debug for Vector2i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2i({}, {})", self.x(), self.y())
    }
}
impl Display for Vector2i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

/// A vector with 3 i32 components
#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct Vector3i(i32x4);
impl Vector3i {
    /// The vector (0, 0, 0)
    pub const ZERO: Self = Self::new(0, 0, 0);

    def_field!(x, x_mut, 0, i32);
    def_field!(y, y_mut, 1, i32);
    def_field!(z, z_mut, 2, i32);

    /// Creates a new vector from the given components
    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self(i32x4::from_array([x, y, z, 0]))
    }

    /// Creates a new vector by setting all components to the given scalar
    #[inline]
    pub const fn from_scalar(scalar: i32) -> Self {
        Self(i32x4::from_array([scalar, scalar, scalar, 0]))
    }

    /// Creates a new vector from the given array
    #[inline]
    pub const fn from_array(array: [i32; 3]) -> Self {
        Self(i32x4::from_array([array[0], array[1], array[2], 0]))
    }

    /// Creates a new vector from the given 2-component vector
    #[inline]
    pub const fn from_v2i(v: v2i, z: i32) -> Self {
        Self(i32x4::from_array([v.x(), v.y(), z, 0]))
    }

    /// Converts the vector into an array
    #[inline]
    pub const fn to_array(&self) -> [i32; 3] {
        let array: [i32; 4] = self.0.to_array();
        [array[0], array[1], array[2]]
    }

    /// Casts this vector into a floating point vector
    #[inline]
    pub fn to_float(&self) -> Vector3f {
        Vector3f(self.0.cast())
    }

    /// Returns an array reference to the vector
    #[inline]
    pub const fn as_array(&self) -> &[i32; 3] {
        let a: &[i32; 4] = self.0.as_array();
        unsafe { std::mem::transmute(a) }
    }

    /// Returns a mutable array reference to the vector
    #[inline]
    pub fn as_mut_array(&mut self) -> &mut [i32; 3] {
        let a: &mut [i32; 4] = self.0.as_mut_array();
        unsafe { std::mem::transmute(a) }
    }

    #[inline]
    fn from_simd_truncate(simd_vec: i32x4) -> Self {
        let zero = i32x4::splat(0);
        let mask = mask32x4::from_array([true, true, true, false]);
        Self(mask.select(simd_vec, zero))
    }
}
impl Debug for Vector3i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector3i({}, {}, {})", self.x(), self.y(), self.z())
    }
}
impl Display for Vector3i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}
impl PartialEq for Vector3i {
    fn eq(&self, other: &Self) -> bool {
        (self.0.as_array()[0] == other.0.as_array()[0])
            && (self.0.as_array()[1] == other.0.as_array()[1])
            && (self.0.as_array()[2] == other.0.as_array()[2])
    }
}
impl Eq for Vector3i {}
impl std::hash::Hash for Vector3i {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0[0].hash(state);
        self.0[1].hash(state);
        self.0[2].hash(state);
    }
}

/// A vector with 4 i32 components
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C, align(16))]
pub struct Vector4i(i32x4);
impl Vector4i {
    /// The vector (0, 0, 0, 0)
    pub const ZERO: Self = Self::new(0, 0, 0, 0);

    def_field!(x, x_mut, 0, i32);
    def_field!(y, y_mut, 1, i32);
    def_field!(z, z_mut, 2, i32);
    def_field!(w, w_mut, 3, i32);

    /// Creates a new vector from the given components
    #[inline]
    pub const fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self(i32x4::from_array([x, y, z, w]))
    }

    /// Creates a new vector by setting all components to the given scalar
    #[inline]
    pub const fn from_scalar(scalar: i32) -> Self {
        Self(i32x4::from_array([scalar; 4]))
    }

    /// Creates a new vector from the given array
    #[inline]
    pub const fn from_array(array: [i32; 4]) -> Self {
        Self(i32x4::from_array(array))
    }

    /// Creates a new vector from the given 2-component vector
    #[inline]
    pub const fn from_v2i(v: v2i, z: i32, w: i32) -> Self {
        Self(i32x4::from_array([v.x(), v.y(), z, w]))
    }

    /// Creates a new vector from the given 3-component vector
    #[inline]
    pub const fn from_v3i(v: v3i, w: i32) -> Self {
        Self(i32x4::from_array([v.x(), v.y(), v.z(), w]))
    }

    /// Converts the vector into an array
    #[inline]
    pub const fn to_array(&self) -> [i32; 4] {
        self.0.to_array()
    }

    /// Casts this vector into a floating point vector
    #[inline]
    pub fn to_float(&self) -> Vector4f {
        Vector4f(self.0.cast())
    }

    /// Returns an array reference to the vector
    #[inline]
    pub const fn as_array(&self) -> &[i32; 4] {
        self.0.as_array()
    }

    /// Returns a mutable array reference to the vector
    #[inline]
    pub fn as_mut_array(&mut self) -> &mut [i32; 4] {
        self.0.as_mut_array()
    }

    #[inline]
    const fn from_simd_truncate(simd_vec: i32x4) -> Self {
        Self(simd_vec)
    }
}
impl Debug for Vector4i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Vector4i({}, {}, {}, {})",
            self.x(),
            self.y(),
            self.z(),
            self.w()
        )
    }
}
impl Display for Vector4i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.x(),
            self.y(),
            self.z(),
            self.w()
        )
    }
}

macro_rules! impl_common_i {
    ($t:ty, $ts:ty) => {
        impl $t {
            /// Returns a vector with each component set to the absolute value of the corresponding component in this vector
            #[inline]
            pub fn abs(self) -> Self {
                Self(self.0.abs())
            }

            /// Returns a vector with each component set to the minimum of the corresponding components between this vector and rhs
            #[inline]
            pub fn min(self, rhs: Self) -> Self {
                Self(<$ts>::simd_min(self.0, rhs.0))
            }

            /// Returns a vector with each component set to the maximum of the corresponding components between this vector and rhs
            #[inline]
            pub fn max(self, rhs: Self) -> Self {
                Self(<$ts>::simd_max(self.0, rhs.0))
            }
        }
    };
}

impl_common_i!(Vector2i, i32x2);
impl_common_i!(Vector3i, i32x4);
impl_common_i!(Vector4i, i32x4);

macro_rules! impl_operators {
    ($t:ty, $ts:ty, $ti:ty) => {
        impl Add for $t {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }
        impl AddAssign for $t {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
        impl Sub for $t {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }
        impl SubAssign for $t {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
        impl Neg for $t {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }
        impl Mul for $t {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self(self.0 * rhs.0)
            }
        }
        impl MulAssign for $t {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }
        impl Div for $t {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self::from_simd_truncate(self.0 / rhs.0)
            }
        }
        impl DivAssign for $t {
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }
        impl Rem for $t {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self::Output {
                Self::from_simd_truncate(self.0 % rhs.0)
            }
        }
        impl RemAssign for $t {
            fn rem_assign(&mut self, rhs: Self) {
                *self = *self % rhs;
            }
        }
        impl Add<$ti> for $t {
            type Output = Self;

            fn add(self, rhs: $ti) -> Self::Output {
                Self::from_simd_truncate(self.0 + <$ts>::splat(rhs))
            }
        }
        impl AddAssign<$ti> for $t {
            fn add_assign(&mut self, rhs: $ti) {
                *self = *self + rhs;
            }
        }
        impl Sub<$ti> for $t {
            type Output = Self;

            fn sub(self, rhs: $ti) -> Self::Output {
                Self::from_simd_truncate(self.0 - <$ts>::splat(rhs))
            }
        }
        impl SubAssign<$ti> for $t {
            fn sub_assign(&mut self, rhs: $ti) {
                *self = *self - rhs;
            }
        }
        impl Mul<$ti> for $t {
            type Output = Self;

            fn mul(self, rhs: $ti) -> Self::Output {
                Self::from_simd_truncate(self.0 * <$ts>::splat(rhs))
            }
        }
        impl MulAssign<$ti> for $t {
            fn mul_assign(&mut self, rhs: $ti) {
                *self = *self * rhs;
            }
        }
        impl Div<$ti> for $t {
            type Output = Self;

            fn div(self, rhs: $ti) -> Self::Output {
                Self::from_simd_truncate(self.0 / <$ts>::splat(rhs))
            }
        }
        impl DivAssign<$ti> for $t {
            fn div_assign(&mut self, rhs: $ti) {
                *self = *self / rhs;
            }
        }
        impl Rem<$ti> for $t {
            type Output = Self;

            fn rem(self, rhs: $ti) -> Self::Output {
                Self::from_simd_truncate(self.0 % <$ts>::splat(rhs))
            }
        }
        impl RemAssign<$ti> for $t {
            fn rem_assign(&mut self, rhs: $ti) {
                *self = *self % rhs;
            }
        }
        impl Index<usize> for $t {
            type Output = $ti;

            fn index(&self, index: usize) -> &Self::Output {
                self.0.index(index)
            }
        }
        impl IndexMut<usize> for $t {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                self.0.index_mut(index)
            }
        }
    };
}

impl_operators!(Vector2f, f32x2, f32);
impl_operators!(Vector3f, f32x4, f32);
impl_operators!(Vector4f, f32x4, f32);
impl_operators!(Vector2i, i32x2, i32);
impl_operators!(Vector3i, i32x4, i32);
impl_operators!(Vector4i, i32x4, i32);

macro_rules! def_quat_field {
    ($name:ident, $name_mut:ident, $i:literal, $t:ty) => {
        #[doc = concat!("The ", stringify!($name), " component of the quaternion")]
        #[inline]
        pub const fn $name(&self) -> $t {
            self.0.as_array()[$i]
        }

        #[doc = concat!("The ", stringify!($name), " component of the quaternion")]
        #[inline]
        pub fn $name_mut(&mut self) -> &mut $t {
            self.0.index_mut($i)
        }
    };
}

/// A quaternion
#[derive(Clone, Copy, PartialEq)]
#[repr(C, align(16))]
pub struct Quaternion(f32x4);
impl Quaternion {
    /// A quaternion representing no rotation
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    def_quat_field!(x, x_mut, 0, f32);
    def_quat_field!(y, y_mut, 1, f32);
    def_quat_field!(z, z_mut, 2, f32);
    def_quat_field!(w, w_mut, 3, f32);

    /// Creates a new quaternion from the given components
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(f32x4::from_array([x, y, z, w]))
    }

    /// Creates a new quaternion from the given array
    #[inline]
    pub const fn from_array(array: [f32; 4]) -> Self {
        Self(f32x4::from_array(array))
    }

    /// Converts the quaternion into an array
    #[inline]
    pub const fn to_array(&self) -> [f32; 4] {
        self.0.to_array()
    }

    /// Returns an array reference to the quaternion
    #[inline]
    pub const fn as_array(&self) -> &[f32; 4] {
        self.0.as_array()
    }

    /// Returns a mutable array reference to the quaternion
    #[inline]
    pub fn as_mut_array(&mut self) -> &mut [f32; 4] {
        self.0.as_mut_array()
    }

    /// Creates a quaternion representing a rotation around an arbitrary axis
    ///
    /// The axis vector must be normalized
    pub fn from_axis_angle(axis: Vector3f, angle: f32) -> Self {
        let (sin, cos) = (angle * 0.5).sin_cos();
        Self::new(axis.x() * sin, axis.y() * sin, axis.z() * sin, cos)
    }

    /// Creates a quaternion representing a rotation around the X axis
    pub fn from_angle_x(angle: f32) -> Self {
        let (sin, cos) = (angle * 0.5).sin_cos();
        Self::new(sin, 0.0, 0.0, cos)
    }

    /// Creates a quaternion representing a rotation around the Y axis
    pub fn from_angle_y(angle: f32) -> Self {
        let (sin, cos) = (angle * 0.5).sin_cos();
        Self::new(0.0, sin, 0.0, cos)
    }

    /// Creates a quaternion representing a rotation around the Z axis
    pub fn from_angle_z(angle: f32) -> Self {
        let (sin, cos) = (angle * 0.5).sin_cos();
        Self::new(0.0, 0.0, sin, cos)
    }

    /// Creates a quaternion representing a rotation specified by yaw, pitch and roll angles
    pub fn from_yaw_pitch_roll(yaw: f32, pitch: f32, roll: f32) -> Self {
        let y = Self::from_angle_y(yaw);
        let x = Self::from_angle_x(pitch);
        let z = Self::from_angle_z(roll);
        y * x * z
    }

    /// Converts the quaternion into an equivalent rotation around an axis
    pub fn to_axis_angle(&self) -> (Vector3f, f32) {
        let q = if self.w() > 1.0 {
            self.normalized()
        } else {
            *self
        };

        let angle = 2.0 * q.w().acos();

        let s = (1.0 - (q.w() * q.w())).sqrt();
        if s < f32::EPSILON {
            (Vector3f::new(1.0, 0.0, 0.0), angle)
        } else {
            let x = q.x() / s;
            let y = q.y() / s;
            let z = q.z() / s;

            (Vector3f::new(x, y, z), angle)
        }
    }

    /// Normalizes the quaternion
    #[inline]
    pub fn normalized(self) -> Self {
        let len = self.xyzw().len();
        if len == 0.0 {
            self
        } else {
            self * (1.0 / len)
        }
    }

    /// Returns the conjugate of this quaternion
    #[inline]
    pub fn conjugate(self) -> Self {
        Self::new(-self.x(), -self.y(), -self.z(), self.w())
    }

    /// Returns the inverse of this quaternion
    #[inline]
    pub fn inverse(self) -> Self {
        self.conjugate() * (1.0 / self.xyzw().len2())
    }

    /// Linearily interpolates between this quaternion and rhs
    pub fn lerp(self, rhs: Self, t: f32) -> Self {
        if self.xyzw().dot(rhs.xyzw()) < 0.0 {
            self - ((rhs + self) * t)
        } else {
            self + ((rhs - self) * t)
        }
        .normalized()
    }

    /// Spherically interpolates between this quaternion and rhs
    pub fn slerp(self, rhs: Self, t: f32) -> Self {
        let temp: Self;
        let mut cosom = self.xyzw().dot(rhs.xyzw());

        if cosom < 0.0 {
            temp = -rhs;
            cosom = -cosom;
        } else {
            temp = rhs;
        }

        let scale1: f32;
        let scale2: f32;
        if (1.0 - cosom) > f32::EPSILON {
            let omega = cosom.acos();
            let sinom = 1.0 / omega.sin();
            scale1 = ((1.0 - t) * omega).sin() * sinom;
            scale2 = (t * omega).sin() * sinom;
        } else {
            scale1 = 1.0 - t;
            scale2 = t;
        }

        ((self * scale1) + (temp * scale2)).normalized()
    }
}
impl Debug for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Quaternion({}, {}, {}, {})",
            self.x(),
            self.y(),
            self.z(),
            self.w()
        )
    }
}
impl Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.x(),
            self.y(),
            self.z(),
            self.w()
        )
    }
}
impl Index<usize> for Quaternion {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for Quaternion {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl Add for Quaternion {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl AddAssign for Quaternion {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for Quaternion {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl SubAssign for Quaternion {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl Neg for Quaternion {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl Mul<f32> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * f32x4::splat(rhs))
    }
}
impl MulAssign<f32> for Quaternion {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}
impl Div<f32> for Quaternion {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / f32x4::splat(rhs))
    }
}
impl DivAssign<f32> for Quaternion {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let xyz = (rhs.xyz() * self.w())
            + (self.xyz() * rhs.w())
            + Vector3f::cross(self.xyz(), rhs.xyz());
        let w = (self.w() * rhs.w()) - Vector3f::dot(self.xyz(), rhs.xyz());
        Self::new(xyz.x(), xyz.y(), xyz.z(), w)
    }
}
impl MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl Mul<Vector3f> for Quaternion {
    type Output = Vector3f;

    fn mul(self, rhs: Vector3f) -> Self::Output {
        rhs + Vector3f::cross(
            self.xyz(),
            Vector3f::cross(self.xyz(), rhs) + (rhs * self.w()),
        ) * 2.0
    }
}

macro_rules! impl_to_array {
    ($t:ty, $ts:ty, $n:literal) => {
        impl From<[$ts; $n]> for $t {
            fn from(a: [$ts; $n]) -> Self {
                Self::from_array(a)
            }
        }

        impl Into<[$ts; $n]> for $t {
            fn into(self) -> [$ts; $n] {
                self.to_array()
            }
        }

        impl AsRef<[$ts; $n]> for $t {
            fn as_ref(&self) -> &[$ts; $n] {
                self.as_array()
            }
        }

        impl AsMut<[$ts; $n]> for $t {
            fn as_mut(&mut self) -> &mut [$ts; $n] {
                self.as_mut_array()
            }
        }

        impl std::borrow::Borrow<[$ts; $n]> for $t {
            fn borrow(&self) -> &[$ts; $n] {
                self.as_array()
            }
        }

        impl std::borrow::BorrowMut<[$ts; $n]> for $t {
            fn borrow_mut(&mut self) -> &mut [$ts; $n] {
                self.as_mut_array()
            }
        }
    };
}

impl_to_array!(Vector2f, f32, 2);
impl_to_array!(Vector3f, f32, 3);
impl_to_array!(Vector4f, f32, 4);
impl_to_array!(Vector2i, i32, 2);
impl_to_array!(Vector3i, i32, 3);
impl_to_array!(Vector4i, i32, 4);
impl_to_array!(Quaternion, f32, 4);

macro_rules! format_width {
    ($value:expr) => {{
        let s = format!("{:+}", $value);
        let w = s.chars().count();
        (s, w)
    }};
}

/// Column-major 2x3 matrix, indexed as [row, column]
#[derive(Clone, Copy, PartialEq)]
#[repr(C, align(8))]
pub struct Matrix2x3([f32x2; 3]);
impl Matrix2x3 {
    /// A matrix representing no transformation
    pub const IDENTITY: Self = Self([
        f32x2::from_array([1.0, 0.0]),
        f32x2::from_array([0.0, 1.0]),
        f32x2::from_array([0.0, 0.0]),
    ]);

    /// Creates a new matrix from individual elements
    #[rustfmt::skip]
    pub const fn new(
        e00: f32, e10: f32, // Column 0
        e01: f32, e11: f32, // Column 1
        e02: f32, e12: f32, // Column 2
    ) -> Self {
        Self([
            f32x2::from_array([e00, e10]),
            f32x2::from_array([e01, e11]),
            f32x2::from_array([e02, e12]),
        ])
    }

    /// Creates a new matrix from the given array
    #[inline]
    pub const fn from_array(array: [[f32; 2]; 3]) -> Self {
        Self([
            f32x2::from_array(array[0]),
            f32x2::from_array(array[1]),
            f32x2::from_array(array[2]),
        ])
    }

    /// Converts the matrix into an array
    #[inline]
    pub const fn to_array(&self) -> [[f32; 2]; 3] {
        [
            self.0[0].to_array(),
            self.0[1].to_array(),
            self.0[2].to_array(),
        ]
    }

    #[inline]
    const fn column(&self, index: usize) -> f32x2 {
        self.0[index]
    }

    /// Checks whether this matrix is the identity matrix, up to a certain error
    pub fn is_identity(&self, epsilon: f32) -> bool {
        const I0: f32x2 = f32x2::from_array([1.0, 0.0]);
        const I1: f32x2 = f32x2::from_array([0.0, 1.0]);
        const I2: f32x2 = f32x2::from_array([0.0, 0.0]);

        let epsilon = f32x2::splat(epsilon);

        let c0 = self.column(0);
        let c1 = self.column(1);
        let c2 = self.column(2);

        let d0 = (c0 - I0).abs();
        let d1 = (c1 - I1).abs();
        let d2 = (c2 - I2).abs();

        let lt0 = d0.simd_lt(epsilon).all();
        let lt1 = d1.simd_lt(epsilon).all();
        let lt2 = d2.simd_lt(epsilon).all();

        lt0 && lt1 && lt2
    }

    /// Creates a matrix representing a translation along the X axis
    pub fn translation_x(translation: f32) -> Self {
        let mut m = Self::IDENTITY;
        m[(0, 2)] = translation;
        m
    }

    /// Creates a matrix representing a translation along the Y axis
    pub fn translation_y(translation: f32) -> Self {
        let mut m = Self::IDENTITY;
        m[(1, 2)] = translation;
        m
    }

    /// Creates a matrix representing a translation
    pub fn translation(translation: Vector2f) -> Self {
        let mut m = Self::IDENTITY;
        m[(0, 2)] = translation.x();
        m[(1, 2)] = translation.y();
        m
    }

    /// Creates a matrix representing a scaling along the X axis
    pub fn scaling_x(scale: f32) -> Self {
        let mut m = Self::IDENTITY;
        m[(0, 0)] = scale;
        m
    }

    /// Creates a matrix representing a scaling along the Y axis
    pub fn scaling_y(scale: f32) -> Self {
        let mut m = Self::IDENTITY;
        m[(1, 1)] = scale;
        m
    }

    /// Creates a matrix representing a scaling
    pub fn scaling(scale: Vector2f) -> Self {
        let mut m = Self::IDENTITY;
        m[(0, 0)] = scale.x();
        m[(1, 1)] = scale.y();
        m
    }

    /// Creates a matrix representing a rotation
    pub fn rotation(angle: f32) -> Self {
        let mut m = Self::IDENTITY;
        let (sin, cos) = angle.sin_cos();
        m[(0, 0)] = cos;
        m[(0, 1)] = -sin;
        m[(1, 0)] = sin;
        m[(1, 1)] = cos;
        m
    }

    /// Creates a matrix representing a transformation specified by scale, rotation and translation, applied in that order
    pub fn from_scale_rotation_translation(
        scale: Vector2f,
        rotation: f32,
        translation: Vector2f,
    ) -> Self {
        let scaling = Self::scaling(scale);
        let rotation = Self::rotation(rotation);
        let translation = Self::translation(translation);
        translation * rotation * scaling
    }

    /// Calculates the determinant of this matrix
    #[inline]
    pub fn determinant(&self) -> f32 {
        let c0 = Vector2f(self.column(0));
        let c1 = Vector2f(self.column(1));
        Vector2f::cross(c0, c1)
    }

    /// Calculates the inverse of this matrix
    pub fn inverse(&self) -> Self {
        let det = self.determinant();
        let inv_det = 1.0 / det;

        let _e00 = self[(0, 0)];
        let _e10 = self[(1, 0)];
        let _e01 = self[(0, 1)];
        let _e11 = self[(1, 1)];
        let _e02 = self[(0, 2)];
        let _e12 = self[(1, 2)];

        let e00 = _e11 * inv_det;
        let e10 = -_e01 * inv_det;
        let e01 = -_e10 * inv_det;
        let e11 = _e00 * inv_det;
        let e02 = (_e01 * _e12 - _e02 * _e11) * inv_det;
        let e12 = (_e02 * _e10 - _e00 * _e12) * inv_det;

        Self::new(e00, e10, e01, e11, e02, e12)
    }

    /// Linearily interpolates between this matrix and rhs
    pub fn lerp(lhs: &Self, rhs: &Self, t: f32) -> Self {
        let lhs_c0 = lhs.column(0);
        let lhs_c1 = lhs.column(1);
        let lhs_c2 = lhs.column(2);

        let rhs_c0 = rhs.column(0);
        let rhs_c1 = rhs.column(1);
        let rhs_c2 = rhs.column(2);

        let t = f32x2::splat(t);
        let c0 = lhs_c0 + ((rhs_c0 - lhs_c0) * t);
        let c1 = lhs_c1 + ((rhs_c1 - lhs_c1) * t);
        let c2 = lhs_c2 + ((rhs_c2 - lhs_c2) * t);

        Self([c0, c1, c2])
    }

    /// Multiples the matrix with a vector while not applying translation
    pub fn mul_no_translate(&self, rhs: Vector2f) -> Vector2f {
        let r0 = self.column(0);
        let r1 = self.column(1);

        let x = simd_swizzle!(rhs.0, [0, 0]);
        let y = simd_swizzle!(rhs.0, [1, 1]);
        Vector2f((r0 * x) + (r1 * y))
    }

    /// Converts the matrix into a 4x4 matrix
    #[rustfmt::skip]
    pub fn to_matrix4x4(&self) -> Matrix4x4 {
        let e00 = self[(0, 0)];
        let e10 = self[(1, 0)];
        let e01 = self[(0, 1)];
        let e11 = self[(1, 1)];
        let e02 = self[(0, 2)];
        let e12 = self[(1, 2)];

        Matrix4x4::from_array([
            [e00, e10, 0.0, 0.0],
            [e01, e11, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [e02, e12, 0.0, 1.0],
        ])
    }

    #[rustfmt::skip]
    fn format_elements(&self) -> ([[String; 2]; 3], usize) {
        let (s00, w00) = format_width!(self[(0, 0)]);
        let (s10, w10) = format_width!(self[(1, 0)]);

        let (s01, w01) = format_width!(self[(0, 1)]);
        let (s11, w11) = format_width!(self[(1, 1)]);

        let (s02, w02) = format_width!(self[(0, 2)]);
        let (s12, w12) = format_width!(self[(1, 2)]);

        let strings = [
            [s00, s10],
            [s01, s11],
            [s02, s12],
        ];

        let widths = [
            w00, w10,
            w01, w11,
            w02, w12,
        ];

        (strings, widths.into_iter().max().unwrap())
    }
}
impl Index<(usize, usize)> for Matrix2x3 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}
impl IndexMut<(usize, usize)> for Matrix2x3 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.1][index.0]
    }
}
impl Mul<Vector2f> for Matrix2x3 {
    type Output = Vector2f;

    fn mul(self, rhs: Vector2f) -> Self::Output {
        let c0 = self.column(0);
        let c1 = self.column(1);
        let c2 = self.column(2);

        let x = simd_swizzle!(rhs.0, [0, 0]);
        let y = simd_swizzle!(rhs.0, [1, 1]);
        Vector2f((c0 * x) + (c1 * y) + c2)
    }
}
impl Mul for Matrix2x3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs_c0 = self.column(0);
        let lhs_c1 = self.column(1);
        let lhs_c2 = self.column(2);

        let c0 = { (lhs_c0 * f32x2::splat(rhs[(0, 0)])) + (lhs_c1 * f32x2::splat(rhs[(1, 0)])) };
        let c1 = { (lhs_c0 * f32x2::splat(rhs[(0, 1)])) + (lhs_c1 * f32x2::splat(rhs[(1, 1)])) };
        let c2 = {
            (lhs_c0 * f32x2::splat(rhs[(0, 2)])) + (lhs_c1 * f32x2::splat(rhs[(1, 2)])) + lhs_c2
        };

        Self([c0, c1, c2])
    }
}
impl Debug for Matrix2x3 {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (strings, width) = self.format_elements();
        let s = format!("Matrix2x3(\
            \n\t{:<width$}, {:<width$}, {:<width$},\
            \n\t{:<width$}, {:<width$}, {:<width$},\
            \n)",
            strings[0][0], strings[1][0], strings[2][0],
            strings[0][1], strings[1][1], strings[2][1],
            width = width
        );

        let s = s.replace('+', " ");
        write!(f, "{}", s)
    }
}
impl Display for Matrix2x3 {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (strings, width) = self.format_elements();
        let s = format!("\
            |{:<width$}   {:<width$}   {:<width$}|\n\
            |{:<width$}   {:<width$}   {:<width$}|\n\
            |{:<width$}   {:<width$}   {:<width$}|",
            strings[0][0], strings[1][0], strings[2][0],
            strings[0][1], strings[1][1], strings[2][1],
            0.0          , 0.0          , 1.0          ,
            width = width
        );

        let s = s.replace('+', " ");
        write!(f, "{}", s)
    }
}

/// Column-major 4x4 matrix, indexed as [row, column]
#[derive(Clone, Copy, PartialEq)]
#[repr(C, align(16))]
pub struct Matrix4x4([f32x4; 4]);
impl Matrix4x4 {
    /// A matrix representing no transformation
    pub const IDENTITY: Self = Self([
        f32x4::from_array([1.0, 0.0, 0.0, 0.0]),
        f32x4::from_array([0.0, 1.0, 0.0, 0.0]),
        f32x4::from_array([0.0, 0.0, 1.0, 0.0]),
        f32x4::from_array([0.0, 0.0, 0.0, 1.0]),
    ]);

    /// Creates a new matrix from individual elements
    #[rustfmt::skip]
    pub const fn new(
        e00: f32, e10: f32, e20: f32, e30: f32, // Column 0
        e01: f32, e11: f32, e21: f32, e31: f32, // Column 1
        e02: f32, e12: f32, e22: f32, e32: f32, // Column 2
        e03: f32, e13: f32, e23: f32, e33: f32, // Column 3
    ) -> Self {
        Self([
            f32x4::from_array([e00, e10, e20, e30]),
            f32x4::from_array([e01, e11, e21, e31]),
            f32x4::from_array([e02, e12, e22, e32]),
            f32x4::from_array([e03, e13, e23, e33]),
        ])
    }

    /// Creates a new matrix from the given array
    #[inline]
    pub const fn from_array(array: [[f32; 4]; 4]) -> Self {
        Self([
            f32x4::from_array(array[0]),
            f32x4::from_array(array[1]),
            f32x4::from_array(array[2]),
            f32x4::from_array(array[3]),
        ])
    }

    /// Converts the matrix into an array
    #[inline]
    pub const fn to_array(&self) -> [[f32; 4]; 4] {
        [
            self.0[0].to_array(),
            self.0[1].to_array(),
            self.0[2].to_array(),
            self.0[3].to_array(),
        ]
    }

    #[inline]
    const fn column(&self, index: usize) -> f32x4 {
        self.0[index]
    }

    /// Checks whether this matrix is the identity matrix, up to a certain error
    pub fn is_identity(&self, epsilon: f32) -> bool {
        const I0: f32x4 = f32x4::from_array([1.0, 0.0, 0.0, 0.0]);
        const I1: f32x4 = f32x4::from_array([0.0, 1.0, 0.0, 0.0]);
        const I2: f32x4 = f32x4::from_array([0.0, 0.0, 1.0, 0.0]);
        const I3: f32x4 = f32x4::from_array([0.0, 0.0, 0.0, 1.0]);

        let epsilon = f32x4::splat(epsilon);

        let c0 = self.column(0);
        let c1 = self.column(1);
        let c2 = self.column(2);
        let c3 = self.column(3);

        let d0 = (c0 - I0).abs();
        let d1 = (c1 - I1).abs();
        let d2 = (c2 - I2).abs();
        let d3 = (c3 - I3).abs();

        let lt0 = d0.simd_lt(epsilon).all();
        let lt1 = d1.simd_lt(epsilon).all();
        let lt2 = d2.simd_lt(epsilon).all();
        let lt3 = d3.simd_lt(epsilon).all();

        lt0 && lt1 && lt2 && lt3
    }

    /// Creates a matrix representing a translation along the X axis
    pub fn translation_x(translation: f32) -> Self {
        let mut m = Self::IDENTITY;
        m[(0, 3)] = translation;
        m
    }

    /// Creates a matrix representing a translation along the Y axis
    pub fn translation_y(translation: f32) -> Self {
        let mut m = Self::IDENTITY;
        m[(1, 3)] = translation;
        m
    }

    /// Creates a matrix representing a translation along the Z axis
    pub fn translation_z(translation: f32) -> Self {
        let mut m = Self::IDENTITY;
        m[(2, 3)] = translation;
        m
    }

    /// Creates a matrix representing a translation
    pub fn translation(translation: Vector3f) -> Self {
        let mut m = Self::IDENTITY;
        m[(0, 3)] = translation.x();
        m[(1, 3)] = translation.y();
        m[(2, 3)] = translation.z();
        m
    }

    /// Creates a matrix representing a scaling along the X axis
    pub fn scaling_x(scale: f32) -> Self {
        let mut m = Self::IDENTITY;
        m[(0, 0)] = scale;
        m
    }

    /// Creates a matrix representing a scaling along the Y axis
    pub fn scaling_y(scale: f32) -> Self {
        let mut m = Self::IDENTITY;
        m[(1, 1)] = scale;
        m
    }

    /// Creates a matrix representing a scaling along the Z axis
    pub fn scaling_z(scale: f32) -> Self {
        let mut m = Self::IDENTITY;
        m[(2, 2)] = scale;
        m
    }

    /// Creates a matrix representing a scaling
    pub fn scaling(scale: Vector3f) -> Self {
        let mut m = Self::IDENTITY;
        m[(0, 0)] = scale.x();
        m[(1, 1)] = scale.y();
        m[(2, 2)] = scale.z();
        m
    }

    /// Creates a matrix representing a rotation around the X axis
    pub fn rotation_x(angle: f32) -> Self {
        let mut m = Self::IDENTITY;
        let (sin, cos) = angle.sin_cos();
        m[(1, 1)] = cos;
        m[(2, 1)] = sin;
        m[(1, 2)] = -sin;
        m[(2, 2)] = cos;
        m
    }

    /// Creates a matrix representing a rotation around the Y axis
    pub fn rotation_y(angle: f32) -> Self {
        let mut m = Self::IDENTITY;
        let (sin, cos) = angle.sin_cos();
        m[(0, 0)] = cos;
        m[(2, 0)] = -sin;
        m[(0, 2)] = sin;
        m[(2, 2)] = cos;
        m
    }

    /// Creates a matrix representing a rotation around the Z axis
    pub fn rotation_z(angle: f32) -> Self {
        let mut m = Self::IDENTITY;
        let (sin, cos) = angle.sin_cos();
        m[(0, 0)] = cos;
        m[(0, 1)] = -sin;
        m[(1, 0)] = sin;
        m[(1, 1)] = cos;
        m
    }

    /// Creates a matrix representing a rotation
    pub fn rotation(rotation: Quaternion) -> Self {
        let sqr = rotation.xyzw() * rotation.xyzw() * 2.0;
        let xx = sqr.x();
        let yy = sqr.y();
        let zz = sqr.z();

        let perm1 = rotation.xxxz() * rotation.yzww() * 2.0;
        let xy = perm1.x();
        let xz = perm1.y();
        let xw = perm1.z();
        let zw = perm1.w();

        let perm2 = rotation.yyz() * rotation.zww() * 2.0;
        let yz = perm2.x();
        let yw = perm2.y();

        let e00 = 1.0 - yy - zz;
        let e01 = xy - zw;
        let e02 = xz + yw;

        let e10 = xy + zw;
        let e11 = 1.0 - xx - zz;
        let e12 = yz - xw;

        let e20 = xz - yw;
        let e21 = yz + xw;
        let e22 = 1.0 - xx - yy;

        Self::from_array([
            [e00, e10, e20, 0.0],
            [e01, e11, e21, 0.0],
            [e02, e12, e22, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Creates a matrix representing a rotation specified by yaw, pitch and roll angles
    #[inline]
    pub fn from_yaw_pitch_roll(yaw: f32, pitch: f32, roll: f32) -> Self {
        let rot = Quaternion::from_yaw_pitch_roll(yaw, pitch, roll);
        Self::rotation(rot)
    }

    /// Creates a matrix representing a transformation specified by scale, rotation and translation, applied in that order
    pub fn from_scale_rotation_translation(
        scale: Vector3f,
        rotation: Quaternion,
        translation: Vector3f,
    ) -> Self {
        let scaling = Self::scaling(scale);
        let rotation = Self::rotation(rotation);
        let translation = Self::translation(translation);
        translation * rotation * scaling
    }

    /// Transposes this matrix
    pub fn transposed(&self) -> Self {
        let c0 = self.column(0);
        let c1 = self.column(1);
        let c2 = self.column(2);
        let c3 = self.column(3);

        macro_rules! unpacklo {
            ($a:expr, $b:expr) => {
                simd_swizzle!($a, $b, [First(0), Second(0), First(1), Second(1)])
            };
        }

        macro_rules! unpackhi {
            ($a:expr, $b:expr) => {
                simd_swizzle!($a, $b, [First(2), Second(2), First(3), Second(3)])
            };
        }

        macro_rules! movelh {
            ($a:expr, $b:expr) => {
                simd_swizzle!($a, $b, [First(0), First(1), Second(0), Second(1)])
            };
        }

        macro_rules! movehl {
            ($a:expr, $b:expr) => {
                simd_swizzle!($a, $b, [Second(2), Second(3), First(2), First(3)])
            };
        }

        // Intel _MM_TRANSPOSE4_PS macro expanded
        let tmp0 = unpacklo!(c0, c1);
        let tmp2 = unpacklo!(c2, c3);
        let tmp1 = unpackhi!(c0, c1);
        let tmp3 = unpackhi!(c2, c3);
        let c0 = movelh!(tmp0, tmp2);
        let c1 = movehl!(tmp2, tmp0);
        let c2 = movelh!(tmp1, tmp3);
        let c3 = movehl!(tmp3, tmp1);

        Self([c0, c1, c2, c3])
    }

    /// Calculates the determinant of this matrix
    pub fn determinant(&self) -> f32 {
        let _2323 = (self[(2, 2)] * self[(3, 3)]) - (self[(3, 2)] * self[(2, 3)]);
        let _1323 = (self[(1, 2)] * self[(3, 3)]) - (self[(3, 2)] * self[(1, 3)]);
        let _1223 = (self[(1, 2)] * self[(2, 3)]) - (self[(2, 2)] * self[(1, 3)]);
        let _0323 = (self[(0, 2)] * self[(3, 3)]) - (self[(3, 2)] * self[(0, 3)]);
        let _0223 = (self[(0, 2)] * self[(2, 3)]) - (self[(2, 2)] * self[(0, 3)]);
        let _0123 = (self[(0, 2)] * self[(1, 3)]) - (self[(1, 2)] * self[(0, 3)]);

        let a = (self[(1, 1)] * _2323) - (self[(2, 1)] * _1323) + (self[(3, 1)] * _1223);
        let b = (self[(0, 1)] * _2323) - (self[(2, 1)] * _0323) + (self[(3, 1)] * _0223);
        let c = (self[(0, 1)] * _1323) - (self[(1, 1)] * _0323) + (self[(3, 1)] * _0123);
        let d = (self[(0, 1)] * _1223) - (self[(1, 1)] * _0223) + (self[(2, 1)] * _0123);

        const SIGN: f32x4 = f32x4::from_array([1.0, -1.0, 1.0, -1.0]);
        let c0 = self.column(0);
        let prod = c0 * f32x4::from_array([a, b, c, d]) * SIGN;
        prod.reduce_sum()
    }

    // Matrix inverse algorithms from:
    // https://lxjk.github.io/2017/09/03/Fast-4x4-Matrix-Inverse-with-SSE-SIMD-Explained.html

    /// Calculates the inverse as long as the input matrix is a transform (only translation, rotation, scaling)
    pub fn transform_inverse(&self) -> Self {
        let self_c0 = self.column(0);
        let self_c1 = self.column(1);
        let self_c2 = self.column(2);
        let self_c3 = self.column(3);

        // transpose 3x3, we know m03 = m13 = m23 = 0
        let t0 = simd_swizzle_0101!(self_c0, self_c1); // 00, 01, 10, 11
        let t1 = simd_swizzle_2323!(self_c0, self_c1); // 02, 03, 12, 13
        let c0 = simd_swizzle!(t0, self_c2, [First(0), First(2), Second(0), Second(3)]); // 00, 10, 20, 23(=0)
        let c1 = simd_swizzle!(t0, self_c2, [First(1), First(3), Second(1), Second(3)]); // 01, 11, 21, 23(=0)
        let c2 = simd_swizzle!(t1, self_c2, [First(0), First(2), Second(2), Second(3)]); // 02, 12, 22, 23(=0)

        // (SizeSqr(mVec[0]), SizeSqr(mVec[1]), SizeSqr(mVec[2]), 0)
        let size_sqr = (c0 * c0) + (c1 * c1) + (c2 * c2);

        // optional test to avoid divide by 0
        let one = f32x4::splat(1.0);
        let eps = f32x4::splat(f32::EPSILON);
        // for each component, if(sizeSqr < SMALL_NUMBER) sizeSqr = 1;
        let mask = f32x4::simd_lt(size_sqr, eps);
        let size_sqr = mask.select(one, one / size_sqr);

        let c0 = c0 * size_sqr;
        let c1 = c1 * size_sqr;
        let c2 = c2 * size_sqr;

        // last line
        let r3 = {
            (c0 * simd_swizzle_1!(self_c3, 0))
                + (c1 * simd_swizzle_1!(self_c3, 1))
                + (c2 * simd_swizzle_1!(self_c3, 2))
        };
        const LAST: f32x4 = f32x4::from_array([0.0, 0.0, 0.0, 1.0]);
        let c3 = LAST - r3;

        Self([c0, c1, c2, c3])
    }

    // 2x2 Matrix multiply A*B
    #[inline]
    fn mul_mat2(lhs: f32x4, rhs: f32x4) -> f32x4 {
        let a = lhs * simd_swizzle!(rhs, [0, 3, 0, 3]);
        let b = simd_swizzle!(lhs, [1, 0, 3, 2]) * simd_swizzle!(rhs, [2, 1, 2, 1]);
        a + b
    }

    // 2x2 Matrix adjugate multiply (A#)*B
    #[inline]
    fn adj_mul_mat2(lhs: f32x4, rhs: f32x4) -> f32x4 {
        let a = simd_swizzle!(lhs, [3, 3, 0, 0]) * rhs;
        let b = simd_swizzle!(lhs, [1, 1, 2, 2]) * simd_swizzle!(rhs, [2, 3, 0, 1]);
        a - b
    }

    // 2x2 Matrix multiply adjugate A*(B#)
    #[inline]
    fn mul_adj_mat2(lhs: f32x4, rhs: f32x4) -> f32x4 {
        let a = lhs * simd_swizzle!(rhs, [3, 0, 3, 0]);
        let b = simd_swizzle!(lhs, [1, 0, 3, 2]) * simd_swizzle!(rhs, [2, 1, 2, 1]);
        a - b
    }

    /// Calculates the inverse of this matrix
    pub fn inverse(&self) -> Self {
        let self_c0 = self.column(0);
        let self_c1 = self.column(1);
        let self_c2 = self.column(2);
        let self_c3 = self.column(3);

        // use block matrix method
        // A is a matrix, then i(A) or iA means inverse of A, A# (or A_ in code) means adjugate of A, |A| (or detA in code) is determinant, tr(A) is trace

        // sub matrices
        let a = simd_swizzle_0101!(self_c0, self_c1);
        let b = simd_swizzle_2323!(self_c0, self_c1);
        let c = simd_swizzle_0101!(self_c2, self_c3);
        let d = simd_swizzle_2323!(self_c2, self_c3);

        // determinant as (|A| |B| |C| |D|)
        let det_sub = ({
            simd_swizzle!(self_c0, self_c2, [First(0), First(2), Second(0), Second(2)])
                * simd_swizzle!(self_c1, self_c3, [First(1), First(3), Second(1), Second(3)])
        }) - ({
            simd_swizzle!(self_c0, self_c2, [First(1), First(3), Second(1), Second(3)])
                * simd_swizzle!(self_c1, self_c3, [First(0), First(2), Second(0), Second(2)])
        });

        let det_a = simd_swizzle_1!(det_sub, 0);
        let det_b = simd_swizzle_1!(det_sub, 1);
        let det_c = simd_swizzle_1!(det_sub, 2);
        let det_d = simd_swizzle_1!(det_sub, 3);

        // let iM = 1/|M| * | X  Y |
        //                  | Z  W |

        // D#C
        let d_c = Self::adj_mul_mat2(d, c);
        // A#B
        let a_b = Self::adj_mul_mat2(a, b);

        // X# = |D|A - B(D#C)
        let x = (det_d * a) - Self::mul_mat2(b, d_c);
        // W# = |A|D - C(A#B)
        let w = (det_a * d) - Self::mul_mat2(c, a_b);

        // |M| = |A|*|D| + ... (continue later)
        let det_m = det_a * det_d;
        // Y# = |B|C - D(A#B)#
        let y = (det_b * c) - Self::mul_adj_mat2(d, a_b);
        // Z# = |C|B - A(D#C)#
        let z = (det_c * b) - Self::mul_adj_mat2(a, d_c);
        // |M| = |A|*|D| + |B|*|C| ... (continue later)
        let det_m = det_m + (det_b * det_c);

        // tr((A#B)(D#C))
        let tr = a_b * simd_swizzle!(d_c, [0, 2, 1, 3]); // (00, 01, 10, 11) as 2x2 matrix

        // |M| = |A|*|D| + |B|*|C| - tr((A#B)(D#C)
        let det_m = det_m - f32x4::splat(tr.reduce_sum());

        const ADJ_SIGN_MASK: f32x4 = f32x4::from_array([1.0, -1.0, -1.0, 1.0]);
        // (1/|M|, -1/|M|, -1/|M|, 1/|M|)
        let r_det_m = ADJ_SIGN_MASK / det_m;

        let x = x * r_det_m;
        let y = y * r_det_m;
        let z = z * r_det_m;
        let w = w * r_det_m;

        // apply adjugate and store, here we combine adjugate shuffle and store shuffle
        let c0 = simd_swizzle!(x, y, [First(3), First(1), Second(3), Second(1)]);
        let c1 = simd_swizzle!(x, y, [First(2), First(0), Second(2), Second(0)]);
        let c2 = simd_swizzle!(z, w, [First(3), First(1), Second(3), Second(1)]);
        let c3 = simd_swizzle!(z, w, [First(2), First(0), Second(2), Second(0)]);

        Self([c0, c1, c2, c3])
    }

    /// Linearily interpolates between this matrix and rhs
    pub fn lerp(lhs: &Self, rhs: &Self, t: f32) -> Self {
        let lhs_c0 = lhs.column(0);
        let lhs_c1 = lhs.column(1);
        let lhs_c2 = lhs.column(2);
        let lhs_c3 = lhs.column(3);

        let rhs_c0 = rhs.column(0);
        let rhs_c1 = rhs.column(1);
        let rhs_c2 = rhs.column(2);
        let rhs_c3 = rhs.column(3);

        let t = f32x4::splat(t);
        let c0 = lhs_c0 + ((rhs_c0 - lhs_c0) * t);
        let c1 = lhs_c1 + ((rhs_c1 - lhs_c1) * t);
        let c2 = lhs_c2 + ((rhs_c2 - lhs_c2) * t);
        let c3 = lhs_c3 + ((rhs_c3 - lhs_c3) * t);

        Self([c0, c1, c2, c3])
    }

    /// Multiples the matrix with a vector while not applying translation
    pub fn mul_no_translate(&self, rhs: Vector3f) -> Vector3f {
        let c0 = self.column(0);
        let c1 = self.column(1);
        let c2 = self.column(2);

        let x = simd_swizzle_1!(rhs.0, 0);
        let y = simd_swizzle_1!(rhs.0, 1);
        let z = simd_swizzle_1!(rhs.0, 2);
        Vector3f::from_simd_truncate((c0 * x) + (c1 * y) + (c2 * z))
    }

    #[rustfmt::skip]
    fn format_elements(&self) -> ([[String; 4]; 4], usize) {
        let (s00, w00) = format_width!(self[(0, 0)]);
        let (s10, w10) = format_width!(self[(1, 0)]);
        let (s20, w20) = format_width!(self[(2, 0)]);
        let (s30, w30) = format_width!(self[(3, 0)]);

        let (s01, w01) = format_width!(self[(0, 1)]);
        let (s11, w11) = format_width!(self[(1, 1)]);
        let (s21, w21) = format_width!(self[(2, 1)]);
        let (s31, w31) = format_width!(self[(3, 1)]);

        let (s02, w02) = format_width!(self[(0, 2)]);
        let (s12, w12) = format_width!(self[(1, 2)]);
        let (s22, w22) = format_width!(self[(2, 2)]);
        let (s32, w32) = format_width!(self[(3, 2)]);

        let (s03, w03) = format_width!(self[(0, 3)]);
        let (s13, w13) = format_width!(self[(1, 3)]);
        let (s23, w23) = format_width!(self[(2, 3)]);
        let (s33, w33) = format_width!(self[(3, 3)]);

        let strings = [
            [s00, s10, s20, s30],
            [s01, s11, s21, s31],
            [s02, s12, s22, s32],
            [s03, s13, s23, s33],
        ];

        let widths = [
            w00, w10, w20, w30,
            w01, w11, w21, w31,
            w02, w12, w22, w32,
            w03, w13, w23, w33,
        ];

        (strings, widths.into_iter().max().unwrap())
    }

    /// Creates a matrix representing the transformation of looking from a position in a direction
    pub fn look_to(pos: Vector3f, dir: Vector3f, up: Vector3f) -> Self {
        let up = up.normalized();

        let f = dir.normalized();
        let s = Vector3f::cross(up, f).normalized();
        let u = Vector3f::cross(f, s);

        let tx = -Vector3f::dot(s, pos);
        let ty = -Vector3f::dot(u, pos);
        let tz = -Vector3f::dot(f, pos);

        Self::from_array([
            [s.x(), u.x(), f.x(), 0.0],
            [s.y(), u.y(), f.y(), 0.0],
            [s.z(), u.z(), f.z(), 0.0],
            [tx, ty, tz, 1.0],
        ])
    }

    /// Creates a matrix representing the transformation of looking from a position at a target
    #[inline]
    pub fn look_at(pos: Vector3f, target: Vector3f, up: Vector3f) -> Self {
        Self::look_to(pos, target - pos, up)
    }

    /// Creates a perspective projection matrix
    ///
    /// Constraints:
    /// - fov_y > 0.0
    /// - aspect_ration > 0.0
    /// - near_plane > 1.0
    /// - far_plane > near_plane
    #[rustfmt::skip]
    pub fn perspective(fov_y: f32, aspect_ratio: f32, near_plane: f32, far_plane: f32) -> Self {
        assert!(fov_y > 0.0);
        assert!(aspect_ratio > 0.0);
        assert!(near_plane > 1.0);
        assert!(far_plane > near_plane);

        let (sin, cos) = (fov_y * 0.5).sin_cos();
        let h = cos / sin;
        let w = h / aspect_ratio;
        let r = far_plane / (far_plane - near_plane);
        let z = -r * near_plane;

        Self::from_array([
            [ w , 0.0, 0.0, 0.0],
            [0.0,  h , 0.0, 0.0],
            [0.0, 0.0,  r , 1.0],
            [0.0, 0.0,  z , 0.0]
        ])
    }

    /// Creates an orthographic projection matrix
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        let e00 = 2.0 / (right - left);
        let e11 = 2.0 / (top - bottom);
        let e03 = (right + left) / (left - right);
        let e13 = (top + bottom) / (bottom - top);

        Self::from_array([
            [e00, 0.0, 0.0, 0.0],
            [0.0, e11, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [e03, e13, 0.0, 1.0],
        ])
    }

    /// Creates a centered orthographic projection matrix
    pub fn orthographic_centered(width: f32, height: f32) -> Self {
        let e00 = 2.0 / width;
        let e11 = 2.0 / height;

        Self::from_array([
            [e00, 0.0, 0.0, 0.0],
            [0.0, e11, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}
impl Index<(usize, usize)> for Matrix4x4 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}
impl IndexMut<(usize, usize)> for Matrix4x4 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.1][index.0]
    }
}
impl Mul<Vector4f> for Matrix4x4 {
    type Output = Vector4f;

    fn mul(self, rhs: Vector4f) -> Self::Output {
        let c0 = self.column(0);
        let c1 = self.column(1);
        let c2 = self.column(2);
        let c3 = self.column(3);

        let x = simd_swizzle_1!(rhs.0, 0);
        let y = simd_swizzle_1!(rhs.0, 1);
        let z = simd_swizzle_1!(rhs.0, 2);
        let w = simd_swizzle_1!(rhs.0, 3);
        Vector4f((c0 * x) + (c1 * y) + (c2 * z) + (c3 * w))
    }
}
impl Mul<Vector3f> for Matrix4x4 {
    type Output = Vector3f;

    fn mul(self, rhs: Vector3f) -> Self::Output {
        let c0 = self.column(0);
        let c1 = self.column(1);
        let c2 = self.column(2);
        let c3 = self.column(3);

        let x = simd_swizzle_1!(rhs.0, 0);
        let y = simd_swizzle_1!(rhs.0, 1);
        let z = simd_swizzle_1!(rhs.0, 2);
        Vector3f::from_simd_truncate((c0 * x) + (c1 * y) + (c2 * z) + c3)
    }
}
impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs_c0 = self.column(0);
        let lhs_c1 = self.column(1);
        let lhs_c2 = self.column(2);
        let lhs_c3 = self.column(3);

        let c0 = {
            (lhs_c0 * f32x4::splat(rhs[(0, 0)]))
                + (lhs_c1 * f32x4::splat(rhs[(1, 0)]))
                + (lhs_c2 * f32x4::splat(rhs[(2, 0)]))
                + (lhs_c3 * f32x4::splat(rhs[(3, 0)]))
        };
        let c1 = {
            (lhs_c0 * f32x4::splat(rhs[(0, 1)]))
                + (lhs_c1 * f32x4::splat(rhs[(1, 1)]))
                + (lhs_c2 * f32x4::splat(rhs[(2, 1)]))
                + (lhs_c3 * f32x4::splat(rhs[(3, 1)]))
        };
        let c2 = {
            (lhs_c0 * f32x4::splat(rhs[(0, 2)]))
                + (lhs_c1 * f32x4::splat(rhs[(1, 2)]))
                + (lhs_c2 * f32x4::splat(rhs[(2, 2)]))
                + (lhs_c3 * f32x4::splat(rhs[(3, 2)]))
        };
        let c3 = {
            (lhs_c0 * f32x4::splat(rhs[(0, 3)]))
                + (lhs_c1 * f32x4::splat(rhs[(1, 3)]))
                + (lhs_c2 * f32x4::splat(rhs[(2, 3)]))
                + (lhs_c3 * f32x4::splat(rhs[(3, 3)]))
        };

        Self([c0, c1, c2, c3])
    }
}
impl From<Matrix2x3> for Matrix4x4 {
    fn from(other: Matrix2x3) -> Self {
        other.to_matrix4x4()
    }
}
impl Debug for Matrix4x4 {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (strings, width) = self.format_elements();
        let s = format!("Matrix4x4(\
            \n\t{:<width$}, {:<width$}, {:<width$}, {:<width$},\
            \n\t{:<width$}, {:<width$}, {:<width$}, {:<width$},\
            \n\t{:<width$}, {:<width$}, {:<width$}, {:<width$},\
            \n\t{:<width$}, {:<width$}, {:<width$}, {:<width$},\
            \n)",
            strings[0][0], strings[1][0], strings[2][0], strings[3][0],
            strings[0][1], strings[1][1], strings[2][1], strings[3][1],
            strings[0][2], strings[1][2], strings[2][2], strings[3][2],
            strings[0][3], strings[1][3], strings[2][3], strings[3][3],
            width = width
        );

        let s = s.replace('+', " ");
        write!(f, "{}", s)
    }
}
impl Display for Matrix4x4 {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (strings, width) = self.format_elements();
        let s = format!("\
            |{:<width$}   {:<width$}   {:<width$}   {:<width$}|\n\
            |{:<width$}   {:<width$}   {:<width$}   {:<width$}|\n\
            |{:<width$}   {:<width$}   {:<width$}   {:<width$}|\n\
            |{:<width$}   {:<width$}   {:<width$}   {:<width$}|",
            strings[0][0], strings[1][0], strings[2][0], strings[3][0],
            strings[0][1], strings[1][1], strings[2][1], strings[3][1],
            strings[0][2], strings[1][2], strings[2][2], strings[3][2],
            strings[0][3], strings[1][3], strings[2][3], strings[3][3],
            width = width
        );

        let s = s.replace('+', " ");
        write!(f, "{}", s)
    }
}

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

macro_rules! impl_bytemuck {
    ($t:ty) => {
        #[cfg(feature = "bytemuck")]
        unsafe impl Pod for $t {}
        #[cfg(feature = "bytemuck")]
        unsafe impl Zeroable for $t {}
    };
}

impl_bytemuck!(Vector2f);
impl_bytemuck!(Vector3f);
impl_bytemuck!(Vector4f);
impl_bytemuck!(Vector2i);
impl_bytemuck!(Vector3i);
impl_bytemuck!(Vector4i);
impl_bytemuck!(Quaternion);
impl_bytemuck!(Matrix2x3);
impl_bytemuck!(Matrix4x4);

#[allow(non_camel_case_types)]
#[cfg(feature = "short_names")]
mod short_names {
    use super::*;

    /// A vector with 2 f32 components
    pub type v2f = Vector2f;
    /// A vector with 3 f32 components
    pub type v3f = Vector3f;
    /// A vector with 4 f32 components
    pub type v4f = Vector4f;

    /// A vector with 2 i32 components
    pub type v2i = Vector2i;
    /// A vector with 3 i32 components
    pub type v3i = Vector3i;
    /// A vector with 4 i32 components
    pub type v4i = Vector4i;

    /// A quaternion
    pub type quat = Quaternion;
    /// Column-major 2x3 matrix, indexed as [row, column]
    pub type mat3 = Matrix2x3;
    /// Column-major 4x4 matrix, indexed as [row, column]
    pub type mat4 = Matrix4x4;
}

#[cfg(feature = "short_names")]
pub use short_names::*;

include!(concat!(env!("OUT_DIR"), "/swizzle.rs"));
