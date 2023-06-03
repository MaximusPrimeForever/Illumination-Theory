use std::ops;

struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    fn origin() -> Vec3 {
        Vec3{e: [0.0, 0.0, 0.0]}
    }

    fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3{e: [e1, e2, e3]}
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> Self::Output {
        (*self).e[index]
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.e1 + _rhs.e1,
            self.e2 + _rhs.e2,
            self.e3 + _rhs.e3
        )
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(
            -self.e1,
            -self.e2,
            -self.e3
        )
    }
}

