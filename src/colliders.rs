use crate::renderer::{LoadedMesh, Vector, Vertex};
use nalgebra_glm as glm;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Collider {
    Plane(BoundingBox),
    Sphere(BoundingBox),
    OBB(BoundingBox),
}

impl Collider {
    pub fn get_bb(&self) -> BoundingBox {
        match &self {
            Collider::Plane(bb) => *bb,
            Collider::Sphere(bb) => *bb,
            Collider::OBB(bb) => *bb,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BoundingBox {
    pub size: Vector,
    pub center: Vector,
}

impl BoundingBox {
    pub fn from_vertex(vertex: &Vertex) -> Self {
        let (mut min_x, mut min_y, mut min_z) = (0., 0., 0.);
        let (mut max_x, mut max_y, mut max_z) = (0., 0., 0.);

        // We could do this with cmp::min and cmp::max,
        // but float doesn't impl Ord trait... Thanks Rust.
        for vector in &vertex.primitives {
            // Min
            if vector.0 <= min_x {
                min_x = vector.0;
            };
            if vector.1 <= min_y {
                min_y = vector.1;
            };
            if vector.2 <= min_z {
                min_z = vector.2;
            };
            // Max...
            if vector.0 >= max_x {
                max_x = vector.0;
            };
            if vector.1 >= max_y {
                max_y = vector.1;
            };
            if vector.2 >= max_z {
                max_z = vector.2;
            };
        }

        let size = Vector(max_x, max_y, max_z);
        let center = Vector(
            (min_x + max_x) / 2.,
            (min_y + max_y) / 2.,
            (min_z + max_z) / 2.,
        );

        Self { size, center }
    }
}

pub fn sphere_hit(
    ray: (glm::Vec3, glm::Vec3),
    entity: &LoadedMesh,
) -> (bool, f32) {
    let bb = entity.collider.unwrap().get_bb();
    let r = bb.size.0 - bb.center.0;

    let (origin, direction) = ray;
    let mesh_pos = entity.transform.position.to_glm();

    let center = mesh_pos;

    let t = glm::dot(&(center - origin), &direction);
    let p = origin + (direction * t);
    let y = glm::length(&(center - p));

    (y < r, t)
}

// This is a plane vs ray intersection.
// Equation of a plane (with infinite size) is (p − p0) ⋅ n = 0:
// Where p is a point on the plane, p0 is the center, and n the normal.
//
// We had to replace p by the equation of a ray (r0 + rd*t) = p:
// Where r0 is the origin, rd the direction (normalized vector),
// p a point on the ray and finally t the distance between r0 and the end
// of the ray.
//
// So have to p on the plane equation with the ray equation. We get
// ((r0 + rd*t) - p0) ⋅ n = 0. We want t.
//
// t = ((p0 - r0) ⋅ n) / r0 ⋅ n
// If t >= 0 if are on the plane.
//
//
// We know if the ray touched our infinite sized plane. We also want to
// check if it is on the delimited bounds. So we're going to do it with
// a circle around the center of the plane (not very precise with the edges
// but it will be enough.
pub fn plane_hit(
    ray: (glm::Vec3, glm::Vec3),
    entity: &LoadedMesh,
) -> (bool, f32) {
    let mut is_hit = false;
    let mut t = 0f32;
    let (origin, direction) = ray;

    // This position is the center of the plane.
    let bb = entity.collider.unwrap().get_bb();
    let plane_pos = entity.transform.position.to_glm();

    // Already normalized.
    let normal = glm::vec3(0., 1., 0.);

    // Compute our denominator.
    let denonminator = glm::dot(&normal, &direction);

    // If it's near to 0, it's parallel to the plane.
    // So no hit, we won't waste computation any further.
    if denonminator.abs() > 0.0001 {
        let diff = plane_pos - origin;
        t = glm::dot(&diff, &normal) / denonminator;

        // Intersect our infinite plane.
        if t > 0.0001 {
            // We check if the point on the plane is
            // in our finite bound.
            // This is the radius of the circle.
            let radius = bb.size.0 - bb.center.0;
            // p is our point in the plane.
            let p = origin + direction * t;
            // This is the vector between the intersection of our ray
            // and the center of the plane.
            let diff = p - plane_pos;
            let d2 = glm::dot(&diff, &diff);

            // If the point is inside the radius, we hit.
            if f32::sqrt(d2) <= radius {
                is_hit = true;
            }
        }
    }

    (is_hit, t)
}
