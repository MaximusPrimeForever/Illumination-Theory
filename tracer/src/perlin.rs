use crate::vec3::Point3;


pub struct Perlin {
    random_floats: Vec<f64>,
    perm_x: Vec<i64>,
    perm_y: Vec<i64>,
    perm_z: Vec<i64>
}

impl Perlin {
    pub fn noise(&self, point: &Point3) -> f64 {
        let i = (4.0 * point.x()) as usize & 255;
        let j = (4.0 * point.y()) as usize & 255;
        let k = (4.0 * point.z()) as usize & 255;

        let index = (self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize;
        return self.random_floats[index]
    }

    fn generate_perlin_permutation(point_count: usize) -> Vec<i64> {
        let mut perm = Vec::<i64>::with_capacity(point_count);

        for i in 0..point_count {
            perm.push(i as i64);
        }

        Perlin::permute(&mut perm);
        perm
    }

    fn permute(array: &mut Vec<i64>) {
        for i in (array.len() - 1)..0 {
            let target = rand::random::<usize>() % i;
            array.swap(i, target);
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let point_count = 256;
        let mut rf = Vec::<f64>::with_capacity(point_count);
        for _ in 0..point_count {
            rf.push(rand::random::<f64>());
        }

        Perlin { 
            random_floats: rf,
            perm_x: Perlin::generate_perlin_permutation(point_count),
            perm_y: Perlin::generate_perlin_permutation(point_count),
            perm_z: Perlin::generate_perlin_permutation(point_count)
        }
    }
}