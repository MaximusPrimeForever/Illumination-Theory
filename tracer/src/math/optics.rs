
use crate::Vec3;

pub fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * (normal)
}

pub fn refract(incident: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = -incident.dot(normal).min(1.0);

    let r_out_perp = etai_over_etat * (incident + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;

    r_out_perp + r_out_parallel
}
