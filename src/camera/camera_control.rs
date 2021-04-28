use crate::camera::*;
use crate::core::Error;
use crate::math::*;

///
/// 3D controls for a camera. Use this to add additional control functionality to a [camera](crate::Camera).
///
pub struct CameraControl {
    camera: Camera,
}

impl CameraControl {
    pub fn new(camera: Camera) -> Self {
        Self { camera }
    }

    ///
    /// Translate the camera by the given change.
    ///
    pub fn translate(&mut self, change: &Vec3) -> Result<(), Error> {
        let position = *self.position();
        let target = *self.target();
        let up = *self.up();
        self.set_view(position + change, target + change, up)?;
        Ok(())
    }

    ///
    /// Rotate the camera around the given point while keeping the same distance to the point.
    /// The input `x` specifies the amount of rotation in the left direction and `y` specifies the amount of rotation in the up direction.
    /// If you want the camera up direction to stay fixed, use the [rotate_around_with_fixed_up](crate::CameraControl::rotate_around_with_fixed_up) function instead.
    ///
    pub fn rotate_around(&mut self, point: &Vec3, x: f32, y: f32) -> Result<(), Error> {
        let dir = (point - self.position()).normalize();
        let right = dir.cross(*self.up());
        let up = right.cross(dir);
        let new_pos = self.position() - right * x + up * y;
        let new_dir = (point - new_pos).normalize();
        let dist = point.distance(*self.position());
        let target = *self.target();
        self.set_view(point - dist * new_dir, target, up)?;
        Ok(())
    }

    ///
    /// Rotate the camera around the given point while keeping the same distance to the point and the same up direction.
    /// The input `x` specifies the amount of rotation in the left direction and `y` specifies the amount of rotation in the up direction.
    ///
    pub fn rotate_around_with_fixed_up(
        &mut self,
        point: &Vec3,
        x: f32,
        y: f32,
    ) -> Result<(), Error> {
        let dir = (point - self.position()).normalize();
        let right = dir.cross(*self.up());
        let mut up = right.cross(dir);
        let new_pos = self.position() - right * x + up * y;
        let new_dir = (point - new_pos).normalize();
        up = *self.up();
        if new_dir.dot(up).abs() < 0.999 {
            let dist = point.distance(*self.position());
            let target = *self.target();
            self.set_view(point - dist * new_dir, target, up)?;
        }
        Ok(())
    }

    ///
    /// Moves the camera in the plane orthogonal to the current view direction, which means the view and up directions will stay the same.
    /// The input `x` specifies the amount of translation in the left direction and `y` specifies the amount of translation in the up direction.
    ///
    pub fn pan(&mut self, x: f32, y: f32) -> Result<(), Error> {
        let right = self.right_direction();
        let up = right.cross(self.view_direction());
        let delta = -right * x + up * y;
        self.translate(&delta)?;
        Ok(())
    }

    ///
    /// Moves the camera towards the given point by the amount delta times the distance to the point while keeping the given minimum and maximum distance to the point.
    ///
    pub fn zoom_towards(
        &mut self,
        point: &Vec3,
        delta: f32,
        minimum: f32,
        maximum: f32,
    ) -> Result<(), Error> {
        let distance = point.distance(*self.position());
        match self.projection_type() {
            ProjectionType::Orthographic {
                width,
                height,
                depth,
            } => {
                let h = (height - delta * distance).max(minimum).min(maximum);
                let w = h * width / height;
                let d = *depth;
                self.set_orthographic_projection(w, h, d)?;
            }
            ProjectionType::Perspective { .. } => {
                let target = *self.target();
                let up = *self.up();
                let direction = self.view_direction();
                let mut zoom = (delta + 1.0) * distance;
                zoom = zoom.max(minimum).min(maximum);
                self.set_view(point - direction * zoom, target, up)?;
            }
        }
        Ok(())
    }
}

impl std::ops::Deref for CameraControl {
    type Target = Camera;

    fn deref(&self) -> &Self::Target {
        &self.camera
    }
}

impl std::ops::DerefMut for CameraControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.camera
    }
}
