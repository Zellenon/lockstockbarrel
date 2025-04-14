use bevy::{math::{ops::atan2, Affine2}, prelude::*};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};
use std::fmt::Debug;

pub trait Transform2d: Component + Debug + PartialEq + Clone + Copy + Reflect {
    fn get_angle_f32(&self) -> f32;

    fn from_rotation2d(rotation: f32) -> Self;

    fn with_translation2d(self, translation: Vec2) -> Self;

    #[must_use]
    fn with_rotation2d(self, rotation: f32) -> Self;

    /// Returns this [`Transform2d`] with a new scale.
    #[must_use]
    fn with_scale2d(self, scale: Vec2) -> Self;

    /// Returns this [`Transform2d`] with a new Z translation.
    #[must_use]
    fn with_layer(self, z_translation: f32) -> Self;

    fn pointed_to(self, direction: Vec2, target_direction: Vec2) -> Self;

    fn pointed_at(self, direction: Vec2, target_position: Vec2) -> Self;

    fn point_to(&mut self, direction: Vec2, target_direction: Vec2);

    fn point_at(&mut self, direction: Vec2, target_position: Vec2);

    /// Get the unit vector in the local `X` direction.
    fn local_x_2d(&self) -> Vec2;

    /// Equivalent to [`-local_x()`][Self::local_x()]
    fn left(&self) -> Vec2;

    /// Equivalent to [`local_x()`][Self::local_x()]
    fn right(&self) -> Vec2;

    /// Get the unit vector in the local `Y` direction.
    fn local_y_2d(&self) -> Vec2;

    /// Equivalent to [`local_y()`][Self::local_y]
    fn up(&self) -> Vec2;

    /// Equivalent to [`-local_y()`][Self::local_y]
    fn down(&self) -> Vec2;

    /// Returns the rotation matrix from this transforms rotation.
    fn rotation_matrix(&self) -> Mat2;

    /// Computes the affine transformation matrix of this transform.
    fn compute_matrix(&self) -> Mat3;

    /// Computes the affine transform of this transform.
    fn compute_affine(&self) -> Affine2;

    /// Translates this [`Transform2d`] around a `point` in space.
    ///
    /// If this [`Transform2d`] has a parent, the `point` is relative to the [`Transform2d`] or [`Transform`] of the parent.
    fn translate_around2d(&mut self, point: Vec2, angle: f32);

    /// Rotates this [`Transform2d`] around a `point` in space.
    ///
    /// If this [`Transform2d`] has a parent, the `point` is relative to the [`Transform2d`] or [`Transform`] of the parent.
    fn rotate_around2d(&mut self, point: Vec2, angle: f32);
}

impl Transform2d for Transform {
    fn get_angle_f32(&self) -> f32 {
        self.rotation.to_2d()
    }

    #[inline]
    fn from_rotation2d(rotation: f32) -> Self {
        Transform {
            rotation: Quat::from_2d(rotation),
            ..Self::IDENTITY
        }
    }

    #[must_use]
    #[inline]
    fn with_translation2d(mut self, translation: Vec2) -> Self {
        self.translation = translation.extend(self.translation.z);
        self
    }

    #[must_use]
    #[inline]
    fn with_rotation2d(mut self, rotation: f32) -> Self {
        self.rotation = Quat::from_2d(rotation);
        self
    }

    /// Returns this [`Transform2d`] with a new scale.
    #[must_use]
    #[inline]
    fn with_scale2d(mut self, scale: Vec2) -> Self {
        self.scale = scale.extend(1.);
        self
    }

    /// Returns this [`Transform2d`] with a new Z translation.
    #[must_use]
    #[inline]
    fn with_layer(mut self, z_translation: f32) -> Self {
        self.translation.z = z_translation;
        self
    }

    #[inline]
    fn pointed_to(mut self, direction: Vec2, target_direction: Vec2) -> Self {
        self.point_to(direction, target_direction);
        self
    }

    #[inline]
    fn pointed_at(mut self, direction: Vec2, target_position: Vec2) -> Self {
        self.point_at(direction, target_position);
        self
    }

    #[inline]
    fn point_to(&mut self, direction: Vec2, target_direction: Vec2) {
        self.rotation = Quat::from_2d(Vec2::angle_between(direction, target_direction));
    }

    #[inline]
    fn point_at(&mut self, direction: Vec2, target_position: Vec2) {
        self.point_to(direction, target_position - self.translation.xy());
    }

    /// Get the unit vector in the local `X` direction.
    #[inline]
    fn local_x_2d(&self) -> Vec2 {
        let (sin, cos) = self.rotation.to_2d().sin_cos();
        (cos, sin).into()
    }

    #[inline]
    /// Equivalent to [`-local_x()`][Self::local_x()]
    fn left(&self) -> Vec2 {
        -self.local_x_2d()
    }

    #[inline]
    /// Equivalent to [`local_x()`][Self::local_x()]
    fn right(&self) -> Vec2 {
        self.local_x_2d()
    }

    /// Get the unit vector in the local `Y` direction.
    #[inline]
    fn local_y_2d(&self) -> Vec2 {
        let (sin, cos) = self.rotation.to_2d().sin_cos();
        (-sin, cos).into()
    }

    /// Equivalent to [`local_y()`][Self::local_y]
    #[inline]
    fn up(&self) -> Vec2 {
        self.local_y_2d()
    }

    /// Equivalent to [`-local_y()`][Self::local_y]
    #[inline]
    fn down(&self) -> Vec2 {
        -self.local_y_2d()
    }

    /// Returns the rotation matrix from this transforms rotation.
    #[inline]
    fn rotation_matrix(&self) -> Mat2 {
        Mat2::from_angle(self.rotation.to_2d())
    }

    /// Computes the affine transformation matrix of this transform.
    #[inline]
    fn compute_matrix(&self) -> Mat3 {
        Mat3::from_scale_angle_translation(
            self.scale.xy(),
            self.rotation.to_2d(),
            self.translation.xy(),
        )
    }

    /// Computes the affine transform of this transform.
    #[inline]
    fn compute_affine(&self) -> Affine2 {
        Affine2::from_scale_angle_translation(
            self.scale.xy(),
            self.rotation.to_2d(),
            self.translation.xy(),
        )
    }

    /// Translates this [`Transform2d`] around a `point` in space.
    ///
    /// If this [`Transform2d`] has a parent, the `point` is relative to the [`Transform2d`] or [`Transform`] of the parent.
    #[inline]
    fn translate_around2d(&mut self, point: Vec2, angle: f32) {
        *self = self
            .with_translation2d(point + Mat2::from_angle(angle) * (self.translation.xy() - point));
    }

    /// Rotates this [`Transform2d`] around a `point` in space.
    ///
    /// If this [`Transform2d`] has a parent, the `point` is relative to the [`Transform2d`] or [`Transform`] of the parent.
    #[inline]
    fn rotate_around2d(&mut self, point: Vec2, angle: f32) {
        self.translate_around2d(point, angle);
        self.rotation = Quat::from_2d(self.rotation.to_2d() + angle);
    }
}

pub trait To2D<T> {
    fn to_2d(&self) -> T;
    fn from_2d(t: T) -> Self;
}

impl To2D<f32> for Quat {
    fn to_2d(&self) -> f32 {
        self.to_euler(EulerRot::ZYX).0
    }

    fn from_2d(t: f32) -> Self {
        Quat::from_rotation_z(t)
    }
}

pub trait IntoScale {
    fn into_scale(self) -> Vec2;
}

impl IntoScale for Vec2 {
    fn into_scale(self) -> Vec2 {
        self
    }
}

impl IntoScale for f32 {
    fn into_scale(self) -> Vec2 {
        Vec2::splat(self)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::TAU;

    use super::*;

    #[test]
    fn local_vectors() {
        let mut transform = Transform::from_rotation2d(TAU / 2.44);
        // assert_eq!(transform.local_y(), transform.rotation_matrix() * Vec2::Y);
        // assert_eq!(transform.local_x(), transform.rotation_matrix() * Vec2::X);
        // transform.rotation = TAU / -0.56;
        // assert_eq!(transform.local_y(), transform.rotation_matrix() * Vec2::Y);
        // assert_eq!(transform.local_x(), transform.rotation_matrix() * Vec2::X);
    }
}
