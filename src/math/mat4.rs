use std::ops::Mul;
use math::vec3::Vec3;
use math::vec4::Vec4;


macro_rules! ACCUMULATE
{
    ($temp:expr, $pos:expr, $neg:expr) => 
    (
        if $temp >= 0.0
        {
            $pos += $temp;
        } else {
            $neg += $temp;
        }
    )
}

pub struct Mat4
{
    pub data : [[f32; 4]; 4]
}

impl Mat4
{
    pub fn new() -> Mat4
    {
        Mat4 {
            data: [ [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0] ]
        }
    }

    pub fn new_from_data(data: [[f32; 4]; 4]) -> Mat4
    {
        Mat4 {
            data: data
        }
    }

    pub fn identity(&mut self)
    {
        self.data = [ [1.0, 0.0, 0.0, 0.0],
                      [0.0, 1.0, 0.0, 0.0],
                      [0.0, 0.0, 1.0, 0.0],
                      [0.0, 0.0, 0.0, 1.0] ]
    }

    pub fn to_string(&self) -> String
    {
        format!(
            "[{:5.2}, {:5.2}, {:5.2}, {:5.2}]\n\
             [{:5.2}, {:5.2}, {:5.2}, {:5.2}]\n\
             [{:5.2}, {:5.2}, {:5.2}, {:5.2}]\n\
             [{:5.2}, {:5.2}, {:5.2}, {:5.2}]\n",
            self.data[0][0], self.data[0][1],
            self.data[0][2], self.data[0][3],

            self.data[1][0], self.data[1][1],
            self.data[1][2], self.data[1][3],

            self.data[2][0], self.data[2][1],
            self.data[2][2], self.data[2][3],

            self.data[3][0], self.data[3][1],
            self.data[3][2], self.data[3][3]
        )
    }

    pub fn inverse(&self) -> Mat4
    {

        let src: [[f32; 4]; 4] = self.data;
        let mut dst = [[0.0; 4]; 4];

        if src[0][3] == 0.0 && src[1][3] == 0.0 &&
           src[2][3] == 0.0 && src[3][3] == 0.0
        {
            let mut pos: f32 = 0.0;
            let mut neg: f32 = 0.0;
            let flt_epsilon: f32 = 1.0e-6;

            let mut temp = src[0][0] * src[1][1] * src[2][2];
            ACCUMULATE!(temp, pos, neg);
            temp = src[0][1] * src[1][2] * src[2][0];
            ACCUMULATE!(temp, pos, neg);
            temp = src[0][2] * src[1][0] * src[2][1];
            ACCUMULATE!(temp, pos, neg);
            temp = src[0][2] * src[1][1] * src[2][0];
            ACCUMULATE!(temp, pos, neg);
            temp = src[0][1] * src[1][0] * src[2][2];
            ACCUMULATE!(temp, pos, neg);
            temp = src[0][0] * src[1][2] * src[2][1];
            ACCUMULATE!(temp, pos, neg);

            let mut det_1 = pos + neg;

            /* Is the submatrix a singular? */
            if (det_1 == 0.0) || ((det_1 / (pos - neg)) < flt_epsilon)
            {
                return self.clone();
            } else {
                det_1 = 1.0 / det_1;
                dst[0][0] =  (src[1][1] * src[2][2] -
                              src[1][2] * src[2][1]) * det_1;
                dst[1][0] =- (src[1][0] * src[2][2] -
                              src[1][2] * src[2][0]) * det_1;
                dst[2][0] =  (src[1][0] * src[2][1] -
                              src[1][1] * src[2][0]) * det_1;
                dst[0][1] =- (src[0][1] * src[2][2] -
                              src[0][2] * src[2][1]) * det_1;
                dst[1][1] =  (src[0][0] * src[2][2] -
                              src[0][2] * src[2][0]) * det_1;
                dst[2][1] =- (src[0][0] * src[2][1] -
                              src[0][1] * src[2][0]) * det_1;
                dst[0][2] =  (src[0][1] * src[1][2] -
                              src[0][2] * src[1][1]) * det_1;
                dst[1][2] =- (src[0][0] * src[1][2] -
                              src[0][2] * src[1][0]) * det_1;
                dst[2][2] =  (src[0][0] * src[1][1] -
                              src[0][1] * src[1][0]) * det_1;
                          
                /* Calculate -C * inverse(A) */
                dst[3][0] =- (src[3][0] * dst[0][0] +
                              src[3][1] * dst[1][0] +
                              src[3][2] * dst[2][0]);
                dst[3][1] =- (src[3][0] * dst[0][1] +
                              src[3][1] * dst[1][1] +
                              src[3][2] * dst[2][1]);
                dst[3][2] =- (src[3][0] * dst[0][2] +
                              src[3][1] * dst[1][2] +
                              src[3][2] * dst[2][2]);
                /* Fill in last column */
                dst[0][3] = 0.0;
                dst[1][3] = 0.0;
                dst[2][3] = 0.0;
                dst[3][3] = 1.0;
            }

        } else {
            /* else non-affine */
            let mut max = 0.0;
            let mut p = [0; 4];
            let k = 0;

            dst = src.clone();

            for k in 0 ..4
            {
                max = 0.0;
                p[k] = 0;

                for i in k ..4
                {
                    let mut sum = 0.0;
                    for j in k ..4
                    {
                        sum += dst[i][j].abs();
                        if sum > 0.0
                        {
                            let tmp = dst[i][k].abs() / sum;
                            if tmp > max
                            {
                                max = tmp;
                                p[k] = i;
                            }
                        }
                    }
                }
            }

            if max == 0.0
            {
                println!("ERROR: Matrix is singular.");
                return self.clone();
            }

            if p[k] != k
            {
                for j in 0 ..4
                {
                    let tmp = dst[k][j];
                    dst[k][j] = dst[p[k]][j];
                    dst[p[k]][j] = tmp;
                }
            }

            let inv_pivot = 1.0 / dst[k][k];
            for j in 0 ..4
            {
                if j != k
                {
                    dst[k][j] = - dst[k][j] * inv_pivot;
                    for i in 0 ..4
                    {
                        if i != k
                        {
                            dst[i][j] += dst[i][k] * dst[k][j];
                        }
                    }
                }
            }

            for k in 2 ..0
            {
                if p[k] != k
                {
                    for i in 0 ..4
                    {
                        let tmp = dst[i][k];
                        dst[i][k] = dst[i][p[k]];
                        dst[i][p[k]] = tmp;
                    }
                }
            }
        }
        return Mat4::new_from_data(dst);
    }
}

impl Mul for Mat4
{
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4
    {
        let mut mat = Mat4::new();
        for i in 0..4
        {
            for j in 0..4
            {
                mat.data[i][j] =  self.data[i][0] * rhs.data[0][j] +
                                  self.data[i][1] * rhs.data[1][j] +
                                  self.data[i][2] * rhs.data[2][j] +
                                  self.data[i][3] * rhs.data[3][j];
            }
        }
        mat
    }
}

impl Mul<Vec4> for Mat4
{
    type Output = Vec3;
    fn mul(self, rhs: Vec4) -> Vec3
    {
        let m_data : &[[f32; 4]; 4] = &self.data;

        Vec3 {
            x: ( rhs.x * m_data[0][0] + rhs.y * m_data[0][1] + rhs.z * m_data[0][2] +  rhs.w * m_data[0][3]),
            y: ( rhs.x * m_data[1][0] + rhs.y * m_data[1][1] + rhs.z * m_data[1][2] +  rhs.w * m_data[1][3]),
            z: ( rhs.x * m_data[2][0] + rhs.y * m_data[2][1] + rhs.z * m_data[2][2] +  rhs.w * m_data[2][3]),
        }
    }
}

impl Clone for Mat4
{
    fn clone(&self) -> Mat4
    {
        Mat4 {
            data: self.data.clone()
        }
    }
}
