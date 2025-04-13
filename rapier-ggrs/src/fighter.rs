use crate::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Component)]
pub struct Fighter {
    pub airborne: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Component)]
pub struct Floor{}

// simply set airborne to false if we are ever touching the ground
pub fn land_on_ground(
    mut fighters: Query<(Entity, &mut Fighter)>,
    floor: Query<Entity, With<Floor>>,
    rapier_context: Res<RapierContext>,
) {
    for (fighter_ent, mut fighter) in fighters.iter_mut() {
        for floor_ent in floor.iter() {
            if let Some(_) = rapier_context.contact_pair(fighter_ent, floor_ent) {
                fighter.airborne = false;
            }
        }
    }
}
