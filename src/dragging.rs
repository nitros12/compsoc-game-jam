use bevy::{prelude::*, render::camera::Camera};

// https://stackoverflow.com/a/65401648

pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system_to_stage(CoreStage::PreUpdate, cursor_state.system())
            .add_system_to_stage(CoreStage::Update, draggable.system())
            .add_system_to_stage(CoreStage::Update, hoverable.system())
            .add_system_to_stage(CoreStage::PostUpdate, drag.system())
            .add_system_to_stage(CoreStage::PostUpdate, drop.system())
            .add_system_to_stage(CoreStage::PostUpdate, material.system())
            .add_event::<DraggedEvent>()
            .add_event::<DroppedEvent>()
            .add_event::<DroppedOntoEvent>();
    }
}

fn setup(commands: &mut Commands) {
    commands.spawn(()).with(CursorState::default()).spawn((
        Transform::default(),
        GlobalTransform::default(),
        Cursor,
    ));
}

#[derive(Default)]
struct CursorState {
    cursor_world: Vec2,
    cursor_offset: Vec2,
    cursor_moved: bool,
}

struct Cursor;

pub struct Draggable;
pub struct Dragged;
pub struct DraggedEvent(pub Entity);
struct Dropped;

pub struct Hoverable;
pub struct Hovered;

pub struct DropTarget;

pub struct DroppedEvent(pub Entity);

/// component added to both the src and dst entities when src is dropped onto
/// dst
pub struct DroppedOntoEvent {
    pub src: Entity,
    pub dst: Entity,
}

fn cursor_state(
    mut cursor_events: EventReader<CursorMoved>,
    windows: Res<Windows>,
    mut q_cursor_state: Query<&mut CursorState>,
    q_camera: Query<&Transform, With<Camera>>,
) {
    let event_cursor_screen = cursor_events.iter().next();

    for mut cursor_state in q_cursor_state.iter_mut() {
        if let Some(event_cursor_screen) = event_cursor_screen {
            let window = windows.get_primary().unwrap();
            let cam_transform = q_camera.iter().last().unwrap();
            cursor_state.cursor_world =
                cursor_to_world(window, cam_transform, event_cursor_screen.position);

            cursor_state.cursor_moved = true;
        } else {
            cursor_state.cursor_moved = false;
        }
    }
}

fn hoverable(
    commands: &mut Commands,
    q_cursor_state: Query<&CursorState>,
    q_is_dragged: Query<(), With<Dragged>>,
    q_hoverable: Query<
        (
            Entity,
            &Transform,
            &Sprite,
            Option<&Hoverable>,
            Option<&DropTarget>,
        ),
        (Or<(With<Hoverable>, With<DropTarget>)>, Without<Dragged>),
    >,
) {
    let cursor_state = q_cursor_state.iter().next().unwrap();
    let is_dragged = q_is_dragged.iter().next().is_some();

    if cursor_state.cursor_moved {
        for (entity, transform, sprite, is_hoverable, is_drop_target) in q_hoverable.iter() {
            if !is_hoverable.is_some() && is_drop_target.is_some() && !is_dragged {
                // ignore non-hoverable targets if we're not dragging
                commands.remove_one::<Hovered>(entity);
                continue;
            }

            let half_width = sprite.size.x / 2.0;
            let half_height = sprite.size.y / 2.0;

            if transform.translation.x - half_width < cursor_state.cursor_world.x
                && transform.translation.x + half_width > cursor_state.cursor_world.x
                && transform.translation.y - half_height < cursor_state.cursor_world.y
                && transform.translation.y + half_height > cursor_state.cursor_world.y
            {
                commands.insert_one(entity, Hovered);
            } else {
                commands.remove_one::<Hovered>(entity);
            }
        }
    }
}

fn material(
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_hoverable: Query<
        (&Handle<ColorMaterial>, Option<&Hovered>, Option<&Dragged>),
        Or<(With<Hoverable>, With<DropTarget>)>,
    >,
) {
    for (material, hovered, dragged) in q_hoverable.iter() {
        let (red, green, alpha) = if dragged.is_some() {
            (0.0, 1.0, 1.0)
        } else if hovered.is_some() {
            (1.0, 0.0, 1.0)
        } else {
            (1.0, 1.0, 1.0)
        };

        materials.get_mut(material).unwrap().color.set_r(red);
        materials.get_mut(material).unwrap().color.set_g(green);
        materials.get_mut(material).unwrap().color.set_a(alpha);
    }
}

fn cursor_to_world(window: &Window, cam_transform: &Transform, cursor_pos: Vec2) -> Vec2 {
    // get the size of the window
    let size = Vec2::new(window.width() as f32, window.height() as f32);

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let screen_pos = cursor_pos - size / 2.0;

    // apply the camera transform
    let out = cam_transform.compute_matrix() * screen_pos.extend(0.0).extend(1.0);
    Vec2::new(out.x, out.y)
}

fn draggable(
    commands: &mut Commands,
    i_mouse_button: Res<Input<MouseButton>>,
    mut ev_dragged: ResMut<Events<DraggedEvent>>,
    mut q_cursor_state: Query<&mut CursorState>,
    q_pressed: Query<(Entity, &Transform), (With<Hovered>, With<Draggable>)>,
    q_released: Query<Entity, With<Dragged>>,
) {
    if i_mouse_button.just_pressed(MouseButton::Left) {
        if let Some((entity, transform)) = q_pressed.iter().next() {
            let mut cursor_state = q_cursor_state.iter_mut().next().unwrap();
            cursor_state.cursor_offset =
                transform.translation.truncate() - cursor_state.cursor_world;

            ev_dragged.send(DraggedEvent(entity));
            commands.insert_one(entity, Dragged);
        }
    } else if i_mouse_button.just_released(MouseButton::Left) {
        for entity in q_released.iter() {
            commands.remove_one::<Dragged>(entity);

            commands.insert_one(entity, Dropped);
        }
    }
}

fn drag(q_cursor_state: Query<&CursorState>, mut q_dragged: Query<&mut Transform, With<Dragged>>) {
    let cursor_state = q_cursor_state.iter().next().unwrap();
    for mut transform in q_dragged.iter_mut() {
        // set position of sprite to offset from cursor
        let pos = cursor_state.cursor_world + cursor_state.cursor_offset;

        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}

fn drop(
    commands: &mut Commands,
    mut ev_dropped_onto: ResMut<Events<DroppedOntoEvent>>,
    mut ev_dropped: ResMut<Events<DroppedEvent>>,
    mut q_dropped: Query<Entity, Added<Dropped>>,
    q_cursor_state: Query<&CursorState>,
    q_droppable: Query<(Entity, &Transform, &Sprite), (With<DropTarget>, Without<Dragged>)>,
) {
    let cursor_state = q_cursor_state.iter().next().unwrap();

    for entity in q_dropped.iter_mut() {
        let mut dropped_onto = None;

        for (entity, transform, sprite) in q_droppable.iter() {
            let half_width = sprite.size.x / 2.0;
            let half_height = sprite.size.y / 2.0;

            if transform.translation.x - half_width < cursor_state.cursor_world.x
                && transform.translation.x + half_width > cursor_state.cursor_world.x
                && transform.translation.y - half_height < cursor_state.cursor_world.y
                && transform.translation.y + half_height > cursor_state.cursor_world.y
            {
                dropped_onto = Some(entity);
            }
        }

        if let Some(dropped_onto) = dropped_onto {
            ev_dropped_onto.send(DroppedOntoEvent {
                src: entity,
                dst: dropped_onto,
            });
        }
        ev_dropped.send(DroppedEvent(entity));
        commands.remove_one::<Dropped>(entity);
    }
}
