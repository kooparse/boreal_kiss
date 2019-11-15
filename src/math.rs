use std::cmp::PartialEq;
use std::ops::Mul;

pub trait VecFunctions: PartialEq {
    /// The length of vector |a| is the square root of the sum of
    /// the squares of its coordinates.
    /// Length is also called "magnitude".
    fn length(&self) -> f32;
    /// Dot product or "inner product".
    fn dot(&self, other: &Self) -> f32;
    /// Cross product.
    fn cross(&self, other: &Self) -> Self;
    // Unit vector.
    fn normalize(&self) -> Self;
}

pub trait MatrixFunctions {
    fn identity() -> Self;
    fn determinant(&self) -> f32;
    fn translate(&mut self, vector: &Vec3);
    fn scale(&mut self, vector: &Vec3);
    fn rotate_x(&mut self, angle: f32);
    fn rotate_y(&mut self, angle: f32);
    fn rotate_z(&mut self, angle: f32);
}

#[derive(Debug)]
pub struct Mat4 {
    pub data: [f32; 16],
}
impl Mat4 {
    fn new(data: [f32; 16]) -> Self {
        Self { data }
    }
}

impl MatrixFunctions for Mat4 {
    fn identity() -> Self {
        Self {
            #[rustfmt::skip]
            data: [
            // r1c1_0  r1c2_1  r1c3_2  r1c4_3
                1., 0., 0., 0.,
            // r2c1_4  r2c2_5  r2c3_6  r2c4_7
                0., 1., 0., 0.,
            // r3c1_8  r3c2_9  r3c3_10  r3c4_11
                0., 0., 1., 0.,
            // r4c1_12  r4c2_13  r4c3_14  r4c4_15
                0., 0., 0., 1.
            ],
        }
    }

    fn determinant(&self) -> f32 {
        let m = self.data;
        m[12] * m[9] * m[6] * m[3]
            - m[8] * m[13] * m[6] * m[3]
            - m[12] * m[5] * m[10] * m[3]
            + m[4] * m[13] * m[10] * m[3]
            + m[8] * m[5] * m[14] * m[3]
            - m[4] * m[9] * m[14] * m[3]
            - m[12] * m[9] * m[2] * m[7]
            + m[8] * m[13] * m[2] * m[7]
            + m[12] * m[1] * m[10] * m[7]
            - m[0] * m[13] * m[10] * m[7]
            - m[8] * m[1] * m[14] * m[7]
            + m[0] * m[9] * m[14] * m[7]
            + m[12] * m[5] * m[2] * m[11]
            - m[4] * m[13] * m[2] * m[11]
            - m[12] * m[1] * m[6] * m[11]
            + m[0] * m[13] * m[6] * m[11]
            + m[4] * m[1] * m[14] * m[11]
            - m[0] * m[5] * m[14] * m[11]
            - m[8] * m[5] * m[2] * m[15]
            + m[4] * m[9] * m[2] * m[15]
            + m[8] * m[1] * m[6] * m[15]
            - m[0] * m[9] * m[6] * m[15]
            - m[4] * m[1] * m[10] * m[15]
            + m[0] * m[5] * m[10] * m[15]
    }

    fn scale(&mut self, scale: &Vec3) {
        self.data[0] *= scale.x;
        self.data[5] *= scale.y;
        self.data[10] *= scale.z;
    }

    fn translate(&mut self, trans: &Vec3) {
        self.data[3] *= trans.x;
        self.data[7] *= trans.y;
        self.data[11] *= trans.z;
    }

    fn rotate_x(&mut self, angle: f32) {
        unimplemented!()
    }
    fn rotate_y(&mut self, trans: &Vec3) {
        unimplemented!()
    }
    fn rotate_z(&mut self, trans: &Vec3) {
        unimplemented!()
    }
}

impl PartialEq for Mat4 {
    fn eq(&self, other: &Self) -> bool {
        self.data[0] == other.data[0]
            && self.data[1] == other.data[1]
            && self.data[2] == other.data[2]
            && self.data[3] == other.data[3]
            && self.data[4] == other.data[4]
            && self.data[5] == other.data[5]
            && self.data[6] == other.data[6]
            && self.data[7] == other.data[7]
            && self.data[8] == other.data[8]
            && self.data[9] == other.data[9]
            && self.data[10] == other.data[10]
            && self.data[11] == other.data[11]
            && self.data[12] == other.data[12]
            && self.data[13] == other.data[13]
            && self.data[14] == other.data[14]
            && self.data[15] == other.data[15]
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Self;

    // TODO: Use simd to compute thise.
    // Not effective.
    fn mul(self, rhs: Mat4) -> Self::Output {
        let data = self.data;
        let rhs = rhs.data;

        // Row 1
        let r1c1 = (data[0] * rhs[0])
            + (data[1] * rhs[4])
            + (data[2] * rhs[8])
            + (data[3] * rhs[12]);

        let r1c2 = (data[0] * rhs[1])
            + (data[1] * rhs[5])
            + (data[2] * rhs[9])
            + (data[3] * rhs[13]);

        let r1c3 = (data[0] * rhs[2])
            + (data[1] * rhs[6])
            + (data[2] * rhs[10])
            + (data[3] * rhs[14]);

        let r1c4 = (data[0] * rhs[3])
            + (data[1] * rhs[7])
            + (data[2] * rhs[11])
            + (data[3] * rhs[15]);

        // Row 2
        let r2c1 = (data[4] * rhs[0])
            + (data[5] * rhs[4])
            + (data[6] * rhs[8])
            + (data[7] * rhs[12]);

        let r2c2 = (data[4] * rhs[1])
            + (data[5] * rhs[5])
            + (data[6] * rhs[9])
            + (data[7] * rhs[13]);

        let r2c3 = (data[4] * rhs[2])
            + (data[5] * rhs[6])
            + (data[6] * rhs[10])
            + (data[7] * rhs[14]);

        let r2c4 = (data[4] * rhs[3])
            + (data[5] * rhs[7])
            + (data[6] * rhs[11])
            + (data[7] * rhs[15]);

        // Row 3
        let r3c1 = (data[8] * rhs[0])
            + (data[9] * rhs[4])
            + (data[10] * rhs[8])
            + (data[11] * rhs[12]);

        let r3c2 = (data[8] * rhs[1])
            + (data[9] * rhs[5])
            + (data[10] * rhs[9])
            + (data[11] * rhs[13]);

        let r3c3 = (data[8] * rhs[2])
            + (data[9] * rhs[6])
            + (data[10] * rhs[10])
            + (data[11] * rhs[14]);

        let r3c4 = (data[8] * rhs[3])
            + (data[9] * rhs[7])
            + (data[10] * rhs[11])
            + (data[11] * rhs[15]);

        // Row 4
        let r4c1 = (data[12] * rhs[0])
            + (data[13] * rhs[4])
            + (data[14] * rhs[8])
            + (data[15] * rhs[12]);

        let r4c2 = (data[12] * rhs[1])
            + (data[13] * rhs[5])
            + (data[14] * rhs[9])
            + (data[15] * rhs[13]);

        let r4c3 = (data[12] * rhs[2])
            + (data[13] * rhs[6])
            + (data[14] * rhs[10])
            + (data[15] * rhs[14]);

        let r4c4 = (data[12] * rhs[3])
            + (data[13] * rhs[7])
            + (data[14] * rhs[11])
            + (data[15] * rhs[15]);

        Self::new([
            r1c1, r1c2, r1c3, r1c4, r2c1, r2c2, r2c3, r2c4, r3c1, r3c2, r3c3,
            r3c4, r4c1, r4c2, r4c3, r4c4,
        ])
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::identity()
    }
}

#[derive(Default, Debug)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl VecFunctions for Vec2 {
    fn length(&self) -> f32 {
        f32::sqrt((self.x * self.x) + (self.y * self.y))
    }

    fn dot(&self, other: &Vec2) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    fn cross(&self, _: &Vec2) -> Self {
        unimplemented!()
    }

    fn normalize(&self) -> Self {
        let length = self.length();
        Self::new(self.x / length, self.y / length)
    }
}

#[derive(Default, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl VecFunctions for Vec3 {
    fn length(&self) -> f32 {
        f32::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z))
    }

    fn dot(&self, other: &Vec3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    fn cross(&self, other: &Vec3) -> Self {
        // cx = ay*bz − az*by
        let cx = (self.y * other.z) - (self.z * other.y);
        // cy = az*bx − ax*bz
        let cy = (self.z * other.x) - (self.x * other.z);
        // cz = ax*by − ay*bz
        let cz = (self.x * other.y) - (self.y * other.z);

        Self::new(cx, cy, cz)
    }

    fn normalize(&self) -> Self {
        let length = self.length();
        Self::new(self.x / length, self.y / length, self.z / length)
    }
}

#[derive(Default, Debug)]
pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl PartialEq for Vec4 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.z == other.z
            && self.w == other.w
    }
}

impl VecFunctions for Vec4 {
    fn length(&self) -> f32 {
        f32::sqrt(
            (self.x * self.x)
                + (self.y * self.y)
                + (self.z * self.z)
                + (self.w * self.w),
        )
    }

    fn dot(&self, other: &Vec4) -> f32 {
        (self.x * other.x)
            + (self.y * other.y)
            + (self.z * other.z)
            + (self.w + other.w)
    }

    fn cross(&self, _other: &Vec4) -> Self {
        unimplemented!()
    }

    fn normalize(&self) -> Self {
        let length = self.length();
        Self::new(
            self.x / length,
            self.y / length,
            self.z / length,
            self.w / length,
        )
    }
}

//
//
// Quick interfaces.
//
pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

pub fn dot<T: VecFunctions>(a: &T, b: &T) -> f32 {
    a.dot(b)
}

pub fn cross<T: VecFunctions>(a: &T, b: &T) -> T {
    a.cross(b)
}

pub fn scale(a: &Mat4, s: &Vec3) -> Mat4 {
    let mut mat4 = Mat4::new(a.data);
    mat4.scale(s);
    mat4
}

pub fn translate(a: &Mat4, s: &Vec3) -> Mat4 {
    let mut mat4 = Mat4::new(a.data);
    mat4.translate(s);
    mat4
}

pub fn rotate_x(a: &Mat4, s: &Vec3) -> Mat4 {
    let mut mat4 = Mat4::new(a.data);
    mat4.rotate_x(s);
    mat4
}

pub fn rotate_y(a: &Mat4, s: &Vec3) -> Mat4 {
    let mut mat4 = Mat4::new(a.data);
    mat4.rotate_y(s);
    mat4
}

pub fn rotate_z(a: &Mat4, s: &Vec3) -> Mat4 {
    let mut mat4 = Mat4::new(a.data);
    mat4.rotate_y(s);
    mat4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vec2() {
        let vec = vec2(2., 2.);
        assert_eq!(vec, Vec2::new(2., 2.))
    }

    #[test]
    fn new_vec3() {
        let vec = vec3(2., 2., 2.);
        assert_eq!(vec, Vec3::new(2., 2., 2.))
    }

    #[test]
    fn length_vec2() {
        let vec = vec2(5., 5.);
        assert_eq!(vec.length(), 7.071068)
    }

    #[test]
    fn length_vec3() {
        let vec = vec3(5., 5., 5.);
        assert_eq!(vec.length(), 8.6602545)
    }

    #[test]
    fn normalize_all_vec() {
        let vec_a = vec2(1., 2.);
        let vec_b = vec3(1., 2., 1.);
        assert_eq!(vec_a.normalize(), vec2(0.4472136, 0.8944272));
        assert_eq!(vec_b.normalize(), vec3(0.40824828, 0.81649655, 0.40824828));
    }

    #[test]
    fn mul_mat4() {
        let a = Mat4::new([
            2., 3., 4., 5., 2., 3., 4., 5., 2., 3., 4., 5., 100., 3., 4., 5.,
        ]);
        let b = Mat4::new([
            10., 11., 12., 13., 14., 15., 16., 17., 18., 19., 20., 21., 22.,
            23., 24., 25.,
        ]);

        let result = [
            244., 258., 272., 286., 244., 258., 272., 286., 244., 258., 272.,
            286., 1224., 1336., 1448., 1560.,
        ];
        assert_eq!(a * b, Mat4::new(result));
    }
}
