use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

#[derive(Default)]
pub struct CameraControlPlugin;

#[derive(SystemLabel)]
pub struct CameraControlLabel;

#[derive(Component)]
pub struct CameraControl {
    pub active: bool,
    pub boundary: Option<(Vec2, Vec2)>,
}

impl Default for CameraControl {
    fn default() -> Self {
        Self {
            active: true,
            boundary: None,
        }
    }
}

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(drag.label(CameraControlLabel))
            .add_system(zoom.label(CameraControlLabel));
    }
}

fn drag(
    windows: Res<Windows>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut query: Query<(&CameraControl, &mut Transform, &OrthographicProjection)>,
    mut last_pos: Local<Option<Vec2>>,
) {
    let window = windows.get_primary().unwrap();

    let cursor_pos = match window.cursor_position() {
        Some(cursor_pos) => cursor_pos,
        None => return,
    };

    let delta = cursor_pos - last_pos.unwrap_or(cursor_pos);

    for (cam, mut transform, projection) in query.iter_mut() {
        if cam.active && mouse_buttons.pressed(MouseButton::Left) {
            let scaling = Vec2::new(
                window.width() / (projection.right - projection.left),
                window.height() / (projection.top - projection.bottom),
            ) * projection.scale;

            let mut new_transform = transform.translation.truncate() - (delta * scaling);

            if let Some((min, max)) = cam.boundary {
                let border_x = (window.width() / 2.0) * scaling.x;
                let border_y = (window.height() / 2.0) * scaling.y;
                let border = Vec2::new(border_x, border_y);

                new_transform = new_transform.clamp(min + border, max - border);
            }

            transform.translation = new_transform.extend(transform.translation.z);
        }
    }
    *last_pos = Some(cursor_pos);
}

fn zoom(
    windows: Res<Windows>,
    mut scroll_events: EventReader<MouseWheel>,
    mut query: Query<(&CameraControl, &mut Transform, &mut OrthographicProjection)>,
) {
    let window = windows.get_primary().unwrap();

    let window_size = Vec2::new(window.width(), window.height());
    let cursor_pos = match window.cursor_position() {
        Some(cursor_pos) => cursor_pos,
        None => return,
    };
    let cursor_pos = cursor_pos / window_size * 2.0 - Vec2::ONE;

    let scroll_amount = scroll_events.iter().map(|e| e.y).sum::<f32>();

    if scroll_amount == 0.0 {
        return;
    }

    for (cam, mut transform, mut projection) in query.iter_mut() {
        if cam.active {
            let old_scale = projection.scale;
            projection.scale = projection.scale * (1.0 - scroll_amount * 0.1);
        }
    }
}
