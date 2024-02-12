use bevy::prelude::*;

use crate::component;

pub fn setup(
    mut commands: Commands
    ) {
    
}

pub fn up_down(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut component::Floor, &mut Text)>,
    ) {
    for (mut floor, mut text) in query.iter_mut() {
        if keys.just_pressed(KeyCode::K) {
            floor.current += 1;
        }
        if keys.just_pressed(KeyCode::J) {
            floor.current -= 1;
        }
        text.sections[0].value = floor.current.to_string();
    }
}
