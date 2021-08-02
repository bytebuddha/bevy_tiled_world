use bevy::prelude::*;

use crate::*;

pub fn handle_map_change(
    mut commands: Commands,
    mut events: EventReader<ChangeMap>,
    mut map_roots: Query<&mut ActiveMap, With<Vec<Handle<Map>>>>,
    mut map_entities: Query<(&MapIndex, &Children)>
) {
    for event in events.iter() {
        let mut active = map_roots.single_mut().unwrap();
        active.0 = event.0;
        for (index, children) in map_entities.iter_mut() {
            for child in children.iter() {
                    if index.0 == event.0 {
                        commands.entity(*child)
                            .insert(Visible { is_visible: true, is_transparent: true });
                    } else {
                        commands.entity(*child)
                            .insert(Visible { is_visible: false, is_transparent: true });
                    }
            }
        }
    }
}
