use glam::{Mat4, Quat, Vec3};

/// A 3D transformation combining translation, rotation, and scale.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Transform {
    /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
    pub translation: Vec3,
    /// Rotation of the entity.
    pub rotation: Quat,
    /// Scale of the entity.
    pub scale: Vec3,
}

impl Transform {
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self { translation, rotation, scale }
    }

    /// Converts this transform into a 4x4 transformation matrix.
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    /// Creates an identity transform (no translation, rotation, or scaling).
    pub fn identity() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// Creates a transform with only translation, no rotation, or scaling.
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// Creates a transform with only rotation, no translation, or scaling.
    pub fn from_rotation(rotation: Quat) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation,
            scale: Vec3::ONE,
        }
    }

    /// Creates a transform with only scaling, no translation or rotation.
    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale,
        }
    }

    /// Creates a transform from a 4x4 transformation matrix.
    ///
    /// Decomposes the matrix into translation, rotation, and scale components.
    pub fn from_matrix(matrix: Mat4) -> Self {
        let (scale, rotation, translation) = matrix.to_scale_rotation_translation();
        Self { translation, rotation, scale }
    }

    /// Moves this transform by the given offset.
    pub fn translate(mut self, delta: Vec3) -> Self {
        self.translation += delta;
        self
    }

    /// Rotates this transform by the given rotation.
    pub fn rotate(mut self, rotation: Quat) -> Self {
        self.rotation = rotation * self.rotation;
        self
    }

    /// Rotates this transform around a specific point in space.
    pub fn rotate_around_point(mut self, point: Vec3, rotation: Quat) -> Self {
        // Translate to origin, rotate, translate back
        self.translation -= point;
        self.translation = rotation * self.translation;
        self.translation += point;
        self.rotation = rotation * self.rotation;
        self
    }

    /// Scales this transform by the given factors for each axis.
    pub fn scale(mut self, factor: Vec3) -> Self {
        self.scale *= factor;
        self
    }

    /// Scales this transform uniformly by the same factor on all axes.
    pub fn uniform_scale(mut self, factor: f32) -> Self {
        self.scale *= factor;
        self
    }

    /// Returns a new transform moved by the given offset.
    pub fn translated(mut self, delta: Vec3) -> Self {
        self.translate(delta);
        self
    }

    /// Returns a new transform rotated by the given rotation.
    pub fn rotated(mut self, rotation: Quat) -> Self {
        self.rotate(rotation);
        self
    }

    /// Returns a new transform scaled by the given factors.
    pub fn scaled(mut self, factor: Vec3) -> Self {
        self.scale(factor);
        self
    }

    /// Gets the forward direction vector (-Z axis after rotation).
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }

    /// Gets the backward direction vector (+Z axis after rotation).
    pub fn back(&self) -> Vec3 {
        self.rotation * Vec3::Z
    }

    /// Gets the up direction vector (+Y axis after rotation).
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    /// Gets the down direction vector (-Y axis after rotation).
    pub fn down(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Y
    }

    /// Gets the right direction vector (+X axis after rotation).
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    /// Gets the left-direction vector (-X axis after rotation).
    pub fn left(&self) -> Vec3 {
        self.rotation * Vec3::NEG_X
    }

    /// Rotates this transform to look at a target position.
    ///
    /// The forward direction will point toward the target.
    pub fn look_at(&mut self, target: Vec3, up: Vec3) -> &mut Self {
        let forward = (target - self.translation).normalize();
        let right = forward.cross(up).normalize();
        let up = right.cross(forward);

        self.rotation = Quat::from_mat3(&glam::Mat3::from_cols(right, up, -forward));
        self
    }

    /// Rotates this transform to look in a specific direction.
    pub fn look_to(&mut self, direction: Vec3, up: Vec3) -> &mut Self {
        let forward = direction.normalize();
        let right = forward.cross(up).normalize();
        let up = right.cross(forward);

        self.rotation = Quat::from_mat3(&glam::Mat3::from_cols(right, up, -forward));
        self
    }

    /// Combines this transform with another transform.
    pub fn mul_transform(&self, other: &Transform) -> Transform {
        // Apply this transform to the other transform
        Transform {
            translation: self.translation + self.rotation * (self.scale * other.translation),
            rotation: self.rotation * other.rotation,
            scale: self.scale * other.scale,
        }
    }

    /// Transforms a point from local space to world space.
    ///
    /// Applies translation, rotation, and scale to the point.
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.translation + self.rotation * (self.scale * point)
    }

    /// Transforms a vector from local space to world space.
    ///
    /// Applies rotation and scale to the vector, but not translation.
    pub fn transform_vector(&self, vector: Vec3) -> Vec3 {
        self.rotation * (self.scale * vector)
    }

    /// Returns the inverse of this transform.
    ///
    /// The inverse transform undoes the effects of this transform.
    pub fn inverse(&self) -> Transform {
        let inv_rotation = self.rotation.inverse();
        let inv_scale = Vec3::ONE / self.scale;
        let inv_translation = inv_rotation * (-inv_scale * self.translation);

        Transform {
            translation: inv_translation,
            rotation: inv_rotation,
            scale: inv_scale,
        }
    }

    /// Linearly interpolates between this transform and another.
    ///
    /// Uses slerp for rotation to ensure smooth rotation interpolation.
    pub fn lerp(&self, other: &Transform, t: f32) -> Transform {
        Transform {
            translation: self.translation.lerp(other.translation, t),
            rotation: self.rotation.slerp(other.rotation, t),
            scale: self.scale.lerp(other.scale, t),
        }
    }

    /// Rotates this transform around the Y axis by the specified angle in radians.
    pub fn rotate_y(&mut self, angle: f32) {
        self.rotation *= Quat::from_axis_angle(Vec3::Y, angle);
    }

    /// Rotates this transform around the X axis by the specified angle in radians.
    pub fn rotate_x(&mut self, angle: f32) {
        self.rotation *= Quat::from_axis_angle(Vec3::X, angle);
    }

    /// Rotates this transform around the Z axis by the specified angle in radians.
    pub fn rotate_z(&mut self, angle: f32) {
        self.rotation *= Quat::from_axis_angle(Vec3::Z, angle);
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

impl std::ops::Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Self::Output {
        self.mul_transform(&rhs)
    }
}

impl std::ops::MulAssign<Transform> for Transform {
    fn mul_assign(&mut self, rhs: Transform) {
        *self = *self * rhs;
    }
}