use bevy::prelude::*;

pub fn load_tiled_objects(parent: &mut ChildBuilder, map: &tiled::Map) {
    for object_group in map.object_groups.iter() {
        parent.spawn()
            .insert(Name::new(object_group.name.clone()))
            .insert(object_group.clone())
            .insert(Transform::identity())
            .insert(GlobalTransform::identity())
            .with_children(|parent| {
                for object in object_group.objects.iter() {
                    parent.spawn()
                        .insert(Name::new(object.name.clone()))
                        .insert(object.clone())
                        .insert(Transform::from_xyz(object.x as f32, object.y as f32, 0.0))
                        .insert(GlobalTransform::from_xyz(object.x as f32, object.y as f32, 0.0));
                }
            });
    }
}
