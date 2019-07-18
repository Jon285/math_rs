use crate::matrix::Mat2;
use crate::vectors::Vec3;

use std::default::Default;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat3 {
    mat: [[f32; 3]; 3],
}

#[allow(dead_code)]
impl Mat3 {
    #[inline]
    pub fn new(
        s0e0: f32,
        s0e1: f32,
        s0e2: f32,
        s1e0: f32,
        s1e1: f32,
        s1e2: f32,
        s2e0: f32,
        s2e1: f32,
        s2e2: f32,
    ) -> Self {
        Mat3 {
            mat: [[s0e0, s0e1, s0e2], [s1e0, s1e1, s1e2], [s2e0, s2e1, s2e2]],
        }
    }

    ///Returns a null matrix
    pub fn zero() -> Self {
        Mat3 {
            mat: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        }
    }

    //================================== TRANSFORMATION MATRICES =========================================

    ///Returns a rotation Matrix around the x-axis
    #[inline]
    pub fn rotation_x(ang: f32) -> Self {
        Mat3 {
            mat: [
                [1.0, 0.0, 0.0],
                [0.0, ang.cos(), ang.sin()],
                [0.0, -ang.sin(), ang.cos()],
            ],
        }
    }

    ///Returns a rotation Matrix around the y-axis
    #[inline]
    pub fn rotation_y(ang: f32) -> Self {
        Mat3 {
            mat: [
                [ang.cos(), 0.0, ang.sin()],
                [0.0, 1.0, 0.0],
                [-ang.sin(), 0.0, ang.cos()],
            ],
        }
    }

    ///Returns a rotation Matrix around the z-axis
    #[inline]
    pub fn rotation_z(ang: f32) -> Self {
        Mat3 {
            mat: [
                [ang.cos(), ang.sin(), 0.0],
                [-ang.sin(), ang.cos(), 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    ///Returns a projection Matrix in the x- and y-axis
    #[inline]
    pub fn projection_xy() -> Self {
        Mat3 {
            mat: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]],
        }
    }

    ///Returns a projection Matrix in the x- and z-axis
    #[inline]
    pub fn projection_xz() -> Self {
        Mat3 {
            mat: [[1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    ///Returns a projection Matrix in the y- and z-axis
    #[inline]
    pub fn projection_yz() -> Self {
        Mat3 {
            mat: [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    ///Creates a projection Matrix in the plane perpendicular to the Vector `n`
    #[inline]
    pub fn projection(n: Vec3) -> Self {
        let n = n.normalized();

        Mat3 {
            mat: [
                [1.0 - n.x.powi(2), -n.x * n.y, -n.x * n.z],
                [-n.x * n.y, 1.0 - n.y.powi(2), -n.y * n.z],
                [-n.x * n.z, -n.y * n.z, 1.0 - n.z.powi(2)],
            ],
        }
    }

    ///Creates a rotation Matrix around de axis of `n` by `ang` radians
    #[inline]
    pub fn rotation(ang: f32, n: Vec3) -> Self {
        let n = n.normalized();

        let factor = 1.0 - ang.cos();
        let sin = ang.sin();
        let cos = ang.cos();

        Mat3 {
            mat: [
                [
                    n.x.powi(2) * factor + cos,
                    n.x * n.y * factor + n.z * sin,
                    n.x * n.z * factor - n.z * sin,
                ],
                [
                    n.x * n.y * factor - n.z * sin,
                    n.y.powi(2) * factor + cos,
                    n.y * n.z * factor + n.x * sin,
                ],
                [
                    n.x * n.z * factor + n.y * sin,
                    n.y * n.z * factor - n.x * sin,
                    n.z.powi(2) * factor + cos,
                ],
            ],
        }
    }

    ///Returns a matrix to uniformly scale a 3D vector in all directions by a factor `k`
    #[inline]
    pub fn scale(k: f32) -> Self {
        Mat3 {
            mat: [[k, 0.0, 0.0], [0.0, k, 0.0], [0.0, 0.0, k]],
        }
    }

    ///Creates a scale Matrix towards the arbitrary direction of `n` by a factor of `k`
    #[inline]
    pub fn scale_arb(k: f32, n: Vec3) -> Self {
        let n = n.normalized();

        //pre calculating some of the members
        let scale = 1.0 + (k - 1.0);
        let nx_ny = (k - 1.0) * n.x * n.y;
        let nx_nz = (k - 1.0) * n.x * n.z;
        let ny_nz = (k - 1.0) * n.y * n.z;

        Mat3 {
            mat: [
                [scale * n.x.powi(2), nx_ny, nx_nz],
                [nx_ny, scale * n.y.powi(2), ny_nz],
                [nx_nz, ny_nz, scale * n.z.powi(2)],
            ],
        }
    }

    #[inline]
    pub fn reflection(n: Vec3) -> Self {
        let n = n.normalized();

        Mat3 {
            mat: [
                [1.0 - 2.0 * n.x.powi(2), -2.0 * n.x * n.y, -2.0 * n.x * n.z],
                [-2.0 * n.x * n.y, 1.0 - 2.0 * n.y.powi(2), -2.0 * n.x * n.z],
                [-2.0 * n.x * n.z, -2.0 * n.y * n.z, 1.0 - 2.0 * n.z.powi(2)],
            ],
        }
    }

    #[inline]
    pub fn shearing_xy(s: f32, t: f32) -> Self {
        Mat3 {
            mat: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [s, t, 1.0]],
        }
    }

    #[inline]
    pub fn shearing_xz(s: f32, t: f32) -> Self {
        Mat3 {
            mat: [[1.0, 0.0, 0.0], [s, 1.0, t], [0.0, 0.0, 1.0]],
        }
    }

    #[inline]
    pub fn shearing_yz(s: f32, t: f32) -> Self {
        Mat3 {
            mat: [[1.0, s, t], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    //=============================================================================================================

    #[inline]
    pub fn from_vec(f: Vec3, s: Vec3, t: Vec3) -> Self {
        Mat3 {
            mat: [[f.x, f.y, f.z], [s.x, s.y, s.z], [t.x, t.y, t.z]],
        }
    }

    #[inline]
    pub fn transpose(&mut self) {
        let temp = *self;
        *self = Mat3 {
            mat: [
                [temp.mat[0][0], temp.mat[1][0], temp.mat[2][0]],
                [temp.mat[1][0], temp.mat[1][1], temp.mat[2][1]],
                [temp.mat[0][2], temp.mat[1][2], temp.mat[2][2]],
            ],
        }
    }

    #[inline]
    pub fn transpost(&self) -> Self {
        Mat3 {
            mat: [
                [self.mat[0][0], self.mat[1][0], self.mat[2][0]],
                [self.mat[1][0], self.mat[1][1], self.mat[2][1]],
                [self.mat[0][2], self.mat[1][2], self.mat[2][2]],
            ],
        }
    }

    pub fn minor(&self, i: usize, j: usize) -> f32 {
        if i > 2 || j > 2 {
            panic!("out of bonds matrix access");
        }

        let mut matrix_2 = Mat2::default();
        let mut col = 0;
        let mut row = 0;

        for a in 0..3 {
            for b in 0..3 {
                //are we in the excluded row or column?
                if a != i && b != j {
                    matrix_2[col][row] = self[a][b];
                    row += 1;

                    //column is filled, change to the next and reset the row
                    if row == 2 {
                        row = 0;
                        col += 1;
                    }
                }
            }
        }
        matrix_2.determinant()
    }

    pub fn cofactor(&self, i: usize, j: usize) -> f32 {
        let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };

        sign * self.minor(i, j)
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        let mut ret: f32 = 0.0;

        //hardcoded column value since the result is the same in any other
        for i in 0..3 {
            ret += self[0][i] * self.cofactor(0, i);
        }
        ret
    }

    pub fn inverse(&self) -> Option<Mat3> {
        let determinant = self.determinant();

        if determinant == 0.0 {
            return None;
        }

        let div = 1.0 / determinant;
        let mut adj = Mat3::default();

        //create the adjoint matrix
        for i in 0..3 {
            for j in 0..3 {
                adj[j][i] = self.cofactor(i, j);
            }
        }
        Some(adj * div)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const f32 {
        &self.mat[0][0] as *const f32
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        &mut self[0][0] as *mut f32
    }
}

impl_mat_ops!(Mat3, mat, 3, [f32; 3]);
impl_mat_ops!(Mat3, Vec3, 3);

impl Default for Mat3 {
    fn default() -> Self {
        Mat3 {
            mat: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }
}
