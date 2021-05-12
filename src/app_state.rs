use bevy::prelude::*;
// NOTE: this "state based" approach to multiple windows is a short term workaround.
// Future Bevy releases shouldn't require such a strict order of operations.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AppState {
    CentralPanelState,
    Breakout,
    Contributors,
    Snake,
}

// remove all entities that are not a camera
pub fn game_state_teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub const PX_SIZE_OF_LEFT_PANEL: f32 = 120.0;
