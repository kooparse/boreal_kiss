use crate::renderer::{Mesh, Transform, Vector, Vertex};
use nalgebra_glm as glm;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Collider {
    Plane,
    Sphere,
    Cube,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BoundingBox {
    pub aabb_min: Vector,
    pub aabb_max: Vector,
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

        let center = Vector(
            (min_x + max_x) / 2.,
            (min_y + max_y) / 2.,
            (min_z + max_z) / 2.,
        );

        let aabb_max = Vector(max_x, max_y, max_z);
        let aabb_min = Vector(min_x, min_y, min_z);

        Self {
            center,
            aabb_min,
            aabb_max,
        }
    }
}

pub fn intersect_ray_cube(
    ray: (glm::Vec3, glm::Vec3),
    entity: &Mesh,
) -> (bool, f32) {
    let mut t_min = 0.;
    let mut t_max = 100000.;

    let entity_model = entity.transform.to_model();

    let entity_pos = glm::vec3(
        entity_model.column(3)[0],
        entity_model.column(3)[1],
        entity_model.column(3)[2],
    );

    // The vector between the origin and the object position.
    // Used to compute the intersection with the plane.
    let delta = entity_pos - ray.0;
    let scale = entity.transform.scale.to_glm();

    let (aabb_max, aabb_min) = {
        let (aabb_max, aabb_min) = (
            entity.bounding_box.aabb_max.to_glm(),
            entity.bounding_box.aabb_min.to_glm(),
        );

        (
            glm::vec3(
                aabb_max.x * scale.x,
                aabb_max.y * scale.y,
                aabb_max.z * scale.z,
            ),
            glm::vec3(
                aabb_min.x * scale.x,
                aabb_min.y * scale.y,
                aabb_min.z * scale.z,
            ),
        )
    };

    //
    // X axis.
    let axis_x = {
        glm::vec3(
            entity_model.column(0)[0],
            entity_model.column(0)[1],
            entity_model.column(0)[2],
        )
    };

    let e = glm::dot(&axis_x, &delta);
    let f = glm::dot(&ray.1, &axis_x);

    let mut t1 = (e + aabb_min.x) / f;
    let mut t2 = (e + aabb_max.x) / f;

    // Swap
    if t1 > t2 {
        let w = t1;
        t1 = t2;
        t2 = w;
    }

    if t2 < t_max {
        t_max = t2;
    }
    if t1 > t_min {
        t_min = t1;
    }

    if t_min > t_max {
        return (false, 0.);
    }

    // axis Y
    let axis_y = {
        glm::vec3(
            entity_model.column(1)[0],
            entity_model.column(1)[1],
            entity_model.column(1)[2],
        )
    };

    let e = glm::dot(&axis_y, &delta);
    let f = glm::dot(&ray.1, &axis_y);

    let mut t1 = (e + aabb_min.y) / f;
    let mut t2 = (e + aabb_max.y) / f;

    // Swap
    if t1 > t2 {
        let w = t1;
        t1 = t2;
        t2 = w;
    }

    if t2 < t_max {
        t_max = t2;
    }
    if t1 > t_min {
        t_min = t1;
    }

    if t_min > t_max {
        return (false, 0.);
    }

    // axis Z
    let axis_z = {
        glm::vec3(
            entity_model.column(2)[0],
            entity_model.column(2)[1],
            entity_model.column(2)[2],
        )
    };

    let e = glm::dot(&axis_z, &delta);
    let f = glm::dot(&ray.1, &axis_z);

    let mut t1 = (e + aabb_min.z) / f;
    let mut t2 = (e + aabb_max.z) / f;

    // Swap
    if t1 > t2 {
        let w = t1;
        t1 = t2;
        t2 = w;
    }

    if t2 < t_max {
        t_max = t2;
    }
    if t1 > t_min {
        t_min = t1;
    }

    if t_min > t_max {
        return (false, 0.);
    }

    return (true, t_min);
}

pub fn sphere_hit(ray: (glm::Vec3, glm::Vec3), entity: &Mesh) -> (bool, f32) {
    let bb = entity.bounding_box;
    let r = bb.aabb_max.0 - bb.center.0;

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
// t = ((p0 - r0) ⋅ n) / rd ⋅ n
// If t >= 0 if are on the plane.
//
//
// We know if the ray touched our infinite sized plane. We also want to
// check if it is on the delimited bounds. So we're going to do it with
// a circle around the center of the plane (not very precise with the edges
// but it will be enough.
pub fn plane_hit(ray: (glm::Vec3, glm::Vec3), entity: &Mesh) -> (bool, f32) {
    let mut is_hit = false;
    let mut t = 0f32;
    let (origin, direction) = ray;

    // This position is the center of the plane.
    let bb = entity.bounding_box;
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
            let radius = bb.aabb_max.0 - bb.center.0;
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
