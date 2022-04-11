use bevy_ecs::prelude::*;

use crate::ColliderWireframe;

#[cfg(feature = "dim3")]
pub fn extract_wireframes(mut commands: Commands, query: Query<Entity, With<ColliderWireframe>>) {
    for entity in query.iter() {
        commands.get_or_spawn(entity).insert(ColliderWireframe);
    }
}

#[cfg(feature = "dim2")]
pub fn extract_wireframes2d(mut commands: Commands, query: Query<Entity, With<ColliderWireframe>>) {
    for entity in query.iter() {
        commands.get_or_spawn(entity).insert(ColliderWireframe);
    }
}
