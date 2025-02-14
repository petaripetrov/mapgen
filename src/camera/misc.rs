use bevy::{math::Vec3, prelude::{Bundle, Component, KeyCode}};

// Bundle to spawn custom camera easily
#[derive(Bundle, Default)]
pub struct CameraBundle {
    // pub camera: Camera3dBundle,
    pub state: CameraState,
    pub settings: CameraSettings
}

// The internal state of the controller
#[derive(Component)]
pub struct CameraState {
    pub center: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraAction {
    Pan,
    Orbit,
    Zoom,
}

/// The configuration of the pan-orbit controller
#[derive(Component)]
pub struct CameraSettings {
    /// World units per pixel of mouse motion
    pub pan_sensitivity: f32,
    /// Radians per pixel of mouse motion
    pub orbit_sensitivity: f32,
    /// Exponent per pixel of mouse motion
    pub zoom_sensitivity: f32,
    /// Key to hold for panning
    pub pan_key: Option<KeyCode>,
    /// Key to hold for orbiting
    pub orbit_key: Option<KeyCode>,
    /// Key to hold for zooming
    pub zoom_key: Option<KeyCode>,
    /// What action is bound to the scroll wheel?
    pub scroll_action: Option<CameraAction>,
    /// For devices with a notched scroll wheel, like desktop mice
    pub scroll_line_sensitivity: f32,
    /// For devices with smooth scrolling, like touchpads
    pub scroll_pixel_sensitivity: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        CameraState {
            center: Vec3::ZERO,
            radius: 1.0,
            upside_down: false,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            pan_sensitivity: 0.001, // 1000 pixels per world unit
            orbit_sensitivity: 0.1f32.to_radians(), // 0.1 degree per pixel
            zoom_sensitivity: 0.01,
            pan_key: Some(KeyCode::ControlLeft),
            orbit_key: Some(KeyCode::AltLeft),
            zoom_key: Some(KeyCode::ShiftLeft),
            scroll_action: Some(CameraAction::Zoom),
            scroll_line_sensitivity: 16.0, // 1 "line" == 16 "pixels of motion"
            scroll_pixel_sensitivity: 1.0,
        }
    }
}