use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub fn load_tiled_objects(parent: &mut ChildBuilder, map: &tiled::Map) {
    for object_group in map.object_groups.iter() {
        parent
            .spawn()
            .insert(Name::new(object_group.name.clone()))
            .insert(object_group.clone())
            .insert(Transform::identity())
            .insert(GlobalTransform::identity())
            .with_children(|parent| {
                for object in object_group.objects.iter() {
                    #[cfg(not(feature = "rapier"))]
                    object_entity(&mut parent.spawn(), map, object);

                    #[cfg(feature = "rapier")]
                    super::rapier::spawn_rapier_object(
                        object_entity(&mut parent.spawn(), map, object),
                        map,
                        object,
                    );
                }
            });
    }
}

fn object_entity<'a, 'b, 'c, 'd>(
    commands: &'a mut EntityCommands<'c, 'd>,
    map: &tiled::Map,
    object: &'b tiled::Object,
) -> &'a mut EntityCommands<'c, 'd> {
    commands
        .insert(Name::new(object.name.clone()))
        .insert(object.clone())
        .insert(Transform::from_xyz(
            object.y as f32,
            map.height as f32 + object.x as f32,
            20.0,
        ))
        .insert(GlobalTransform::from_xyz(
            object.y as f32,
            map.height as f32 + object.x as f32,
            20.0,
        ))
}
