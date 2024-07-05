use bevy::{prelude::*, window::PrimaryWindow};

use crate::pxtext::{PickRect, EventType, PickableText, PxText, PxTextEvent};

pub(crate) fn handle_input_system(
    q_text: Query<(Entity, &PxText, &Children)>,
    q_pickable: Query<(&PickableText, &PickRect)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut pick_evw: EventWriter<PxTextEvent>,
) {
    let (camera, camera_transform) = q_camera.iter().next().unwrap();
    if let Some(cursor_pos) = q_windows
        .single()
        .cursor_position()
        .and_then(|cursor|
            camera.viewport_to_world_2d(camera_transform, cursor)
        ) {
        for (entity, text, children) in q_text.iter() {
            for child in children.iter() {
                if let Ok((
                    pickable,
                    rects
                )) = q_pickable.get(*child) {
                    let (value, range) = pickable.get_string(text);

                    for rect in &rects.0 {
                        if rect.contains(cursor_pos.as_ivec2()) {
                            let pick_type = if mouse_buttons.just_pressed(
                                MouseButton::Left
                            ) {
                                EventType::LeftClick
                            } else if mouse_buttons.just_pressed(
                                MouseButton::Right
                            ) {
                                EventType::RightClick
                            } else {
                                EventType::Hover
                            };

                            pick_evw.send(PxTextEvent {
                                entity,
                                range: range.clone(),
                                value: value.clone(),
                                rect: *rect,
                                pick_type,
                            });
                        }
                    }
                }
            }
        }
    }
}
