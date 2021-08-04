use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn draw_colliders(
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    query: Query<(&ColliderShape, &ColliderPosition)>,
) {
    for (shape, position) in query.iter() {
        match shape.0.as_typed_shape() {
            TypedShape::Cuboid(cuboid) => {
                let points = cuboid.to_polyline();
                lines.line_colored(
                    Vec3::new(
                        position.translation.x + points[0][0],
                        position.translation.y - points[1][1],
                        10.0,
                    ),
                    Vec3::new(
                        position.translation.x + points[1][0],
                        position.translation.y - points[1][1],
                        10.0,
                    ),
                    0.0,
                    Color::RED,
                );
                lines.line_colored(
                    Vec3::new(
                        position.translation.x + points[1][0],
                        position.translation.y - points[1][1],
                        10.0,
                    ),
                    Vec3::new(
                        position.translation.x + points[2][0],
                        position.translation.y - points[2][1],
                        10.0,
                    ),
                    0.0,
                    Color::RED,
                );
                lines.line_colored(
                    Vec3::new(
                        position.translation.x + points[2][0],
                        position.translation.y - points[2][1],
                        10.0,
                    ),
                    Vec3::new(
                        position.translation.x + points[3][0],
                        position.translation.y - points[3][1],
                        10.0,
                    ),
                    0.0,
                    Color::RED,
                );
                lines.line_colored(
                    Vec3::new(
                        position.translation.x + points[3][0],
                        position.translation.y - points[3][1],
                        10.0,
                    ),
                    Vec3::new(
                        position.translation.x + points[0][0],
                        position.translation.y - points[0][1],
                        10.0,
                    ),
                    0.0,
                    Color::RED,
                );
            }
            TypedShape::Polyline(polyline) => {
                let segments: Vec<Segment> = polyline.segments().collect();
                for segment in segments.iter() {
                    lines.line_colored(
                        Vec3::new(
                            position.translation.x + segment.a[0],
                            position.translation.y - segment.a[1],
                            10.0,
                        ),
                        Vec3::new(
                            position.translation.x + segment.b[0],
                            position.translation.y - segment.b[1],
                            10.0,
                        ),
                        0.0,
                        Color::RED,
                    );
                }
                lines.line_colored(
                    Vec3::new(
                        position.translation.x + segments[segments.len() - 1].a[0],
                        position.translation.y - segments[segments.len() - 1].a[1],
                        10.0,
                    ),
                    Vec3::new(
                        position.translation.x + segments[0].a[0],
                        position.translation.y - segments[0].a[1],
                        10.0,
                    ),
                    0.0,
                    Color::RED,
                );
            },
            TypedShape::Ball(_) =>  {
                lines.line_colored(
                    Vec3::new(
                        position.translation.x - 6.0,
                        position.translation.y - 6.0,
                        10.0,
                    ),
                    Vec3::new(
                        position.translation.x + 6.0,
                        position.translation.y + 6.0,
                        10.0,
                    ),
                    0.0,
                    Color::RED,
                );

                lines.line_colored(
                    Vec3::new(
                        position.translation.x + 6.0,
                        position.translation.y - 6.0,
                        10.0,
                    ),
                    Vec3::new(
                        position.translation.x - 6.0,
                        position.translation.y + 6.0,
                        10.0,
                    ),
                    0.0,
                    Color::RED,
                );
            }
            _ => {}
        }
    }
}
