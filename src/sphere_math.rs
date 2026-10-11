use bevy::math::{Dir3, Quat, Vec3};

pub fn project(normal: &Dir3, vec: &Vec3) -> Vec3 {
    vec - normal.as_vec3() * normal.dot(*vec)
}

pub fn move_in_direction(point: &Vec3, direction: &Vec3, distance: f32) -> Vec3 {
    let radius = point.length();

    let angle = distance / radius;
    let normal = Dir3::new(*point).unwrap();
    let tangent = project(&normal, direction);

    let axis = normal.cross(tangent.normalize());
    let rotation = Quat::from_axis_angle(axis, angle);
    rotation * point
}
