use bevy::prelude::*;
use tiled::{ObjectShape, Object, PropertyValue, Map};
use bevy_rapier2d::prelude::*;
use bevy::ecs::system::EntityCommands;

pub fn spawn_rapier_object(entity: &mut EntityCommands, map: &Map, object: &Object) {
    match object.obj_type.as_str() {
        "Collider" | "Collider:Solid" | "Collider:Sensor" => spawn_collider(entity, map, object),
        _ => {}
    }
}

pub fn spawn_collider(entity: &mut EntityCommands, map: &tiled::Map, object: &Object) {
    if let Some(shape) = get_collider_shape(object, map) {
        entity.insert_bundle(ColliderBundle {
            shape,
            collider_type: get_collider_type(object),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert_bundle(RigidBodyBundle {
            body_type: get_rigid_body_type(object),
            position: Vec2::new(
                object.x + map.tile_width as f32 / 2.0,
                map.height as f32 *map.tile_height as f32 - object.y + map.tile_height as f32 / 2.0
            ).into(),
            ..Default::default()
        })
        .insert(ColliderDebugRender {
            color: Color::rgba(1.0, 0.0, 0.0, 0.65)
        });
    }
}

fn get_collider_shape(object: &Object, map: &tiled::Map) -> Option<ColliderShape> {
    match &object.shape {
        ObjectShape::Rect { width, height } => {
            Some(ColliderShape::cuboid(*width / 2.0, *height / 2.0))
        },
        ObjectShape::Ellipse { width, height } => {
            if width == height {
                Some(ColliderShape::ball(*width / 2.0))
            } else {
                Some(ColliderShape::capsule(
                    Point::new(*height / 2.0, 0.0),
                    Point::new(0.0, 0.0),
                    *width
                ))
            }
        },
        ObjectShape::Polygon { points } => {
            let mut points = points
                .iter()
                .map(|(x, y)| { Point::new(
                    x + map.tile_width as f32 / 2.0,
                    map.height as f32 *map.tile_height as f32 - y + map.tile_height as f32 / 2.0
                ) })
                .collect::<Vec<Point<f32>>>();
            points.push(points[0]);
            Some(ColliderShape::polyline(points, None))
        },
        ObjectShape::Polyline { points } => {
            let points = points
                .iter()
                .map(|(x, y)| {
                    Point::new(*x, map.height as f32 - *y)
                })
                .collect::<Vec<Point<f32>>>();
            Some(ColliderShape::polyline(points, None))
        },
        ObjectShape::Point(_, _) => {
            warn!("Colliders cannot be applied to tiled 'Point' objects.");
            None
        }
    }
}

fn get_collider_type(object: &Object) -> ColliderType {
    match object.obj_type.as_str() {
        "Collider:Sensor" => ColliderType::Sensor,
        _ => ColliderType::Solid
    }
}

fn get_rigid_body_type(object: &Object) -> RigidBodyType {
    if let Some(body_ty) = object.properties.get("RigidBodyType") {
        match body_ty {
            PropertyValue::StringValue(body_ty) => {
                match body_ty.as_str() {
                    "Static" => return RigidBodyType::Static,
                    "Dynamic" => return RigidBodyType::Dynamic,
                    value => {
                        warn!("Invalid RigidBodyType: {}", value);
                    }
                }
            },
            _ => {}
        }
    }
    RigidBodyType::Static
}
