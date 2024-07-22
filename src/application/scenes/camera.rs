use glam::{Mat4, Vec3, Vec4};

#[derive(Debug)]
pub struct CameraGPU {
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub view_matrix_inverse: Mat4,
    pub projection_matrix_inverse: Mat4,
    pub position: Vec4,
    pub plane_width: f32,
    pub plane_height: f32,
    pub plane_near: f32,
}

pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

pub struct Camera {
    // camera Attributes
    pub eye: Vec3,
    pub at: Vec3,
    pub world_up: Vec3,

    pub up: Vec3,
    pub right: Vec3,

    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,

    pub movement_acceleration: f32,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,

    pub yaw: f32,
    pub pitch: f32,

    pub is_accelerating: bool,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 1600. / 920.;
        Camera::new(
            &Vec3::new(0., 0., -5.),
            aspect_ratio,
            45.,
            0.1,
            200.,
            &Vec3::new(0., 1., 0.),
        )
    }
}

impl Camera {
    pub fn new(
        position: &Vec3,
        aspect_ratio: f32,
        fov: f32,
        near: f32,
        far: f32,
        world_up: &Vec3,
    ) -> Self {
        let mut camera = Camera {
            eye: *position,
            at: Vec3::ZERO,
            world_up: *world_up,
            up: Vec3::ZERO,
            right: Vec3::ZERO,
            fov,
            aspect_ratio,
            near,
            far,
            movement_acceleration: 5.,
            movement_speed: 20.,
            mouse_sensitivity: 0.1,
            yaw: -90.,
            pitch: 0.,
            is_accelerating: false,
        };

        camera.update_vectors();

        camera
    }

    fn update_vectors(&mut self) {
        // calculate the new at vector
        let mut front = Vec3::ZERO;
        front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        front.y = self.pitch.to_radians().sin();
        front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.at = front.normalize();
        self.right = Vec3::cross(self.at, self.world_up).normalize();
        self.up = Vec3::cross(self.right, self.at).normalize();
    }

    fn get_view(&self) -> Mat4 {
        Mat4::look_at_lh(self.eye, self.eye + self.at, self.up)
    }

    fn get_perspective(&self) -> Mat4 {
        Mat4::perspective_lh(
            self.fov.to_radians(),
            self.aspect_ratio,
            self.near,
            self.far,
        )
    }

    fn get_plane_height(&self) -> f32 {
        2. * self.near * (0.5 * self.fov.to_radians()).tan()
    }

    fn get_plane_width(&self, plane_height: f32) -> f32 {
        plane_height * self.aspect_ratio
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f64) {
        let mut velocity = self.movement_speed * (delta_time as f32);
        if self.is_accelerating {
            velocity *= self.movement_acceleration;
        }

        match direction {
            CameraMovement::FORWARD => self.eye -= self.at * velocity,
            CameraMovement::BACKWARD => self.eye += self.at * velocity,
            CameraMovement::LEFT => self.eye -= self.right * velocity,
            CameraMovement::RIGHT => self.eye += self.right * velocity,
            CameraMovement::UP => self.eye += self.world_up * velocity,
            CameraMovement::DOWN => self.eye -= self.world_up * velocity,
        };
    }

    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32, constrain_pitch: bool) {
        let x_offset = x_offset * self.mouse_sensitivity;
        let y_offset = y_offset * self.mouse_sensitivity;
        self.yaw += x_offset;
        self.pitch += y_offset;
        // make sure that when pitch is out of bounds, screen doesn't get flipped
        if constrain_pitch {
            self.pitch = self.pitch.clamp(-89.0, 89.0);
        }
        // update Front, Right and Up Vectors using the updated Euler angles
        self.update_vectors();
    }

    pub fn get_gpu_data(&self) -> CameraGPU {
        let view_mat = self.get_view();
        let proj_mat = self.get_perspective();
        let plane_height = self.get_plane_height();
        let plane_width = self.get_plane_width(plane_height);

        CameraGPU {
            view_matrix: view_mat,
            projection_matrix: proj_mat,
            view_matrix_inverse: Mat4::inverse(&view_mat),
            projection_matrix_inverse: Mat4::inverse(&proj_mat),
            position: Vec4::new(self.eye.x, self.eye.y, self.eye.z, 1.),
            plane_width,
            plane_height,
            plane_near: self.near,
        }
    }
}
