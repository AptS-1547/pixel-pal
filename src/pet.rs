use bevy::prelude::*;

#[derive(Component)]
pub struct Pet {
    pub hunger: f32,
    pub happiness: f32,
    pub energy: f32,
}

impl Pet {
    pub fn new() -> Self {
        Self {
            hunger: 80.0,
            happiness: 80.0,
            energy: 100.0,
        }
    }
}

pub fn update_pet_state(time: Res<Time>, mut pet_query: Query<&mut Pet>) {
    for mut pet in &mut pet_query {
        pet.hunger = (pet.hunger - 1.5 * time.delta_secs()).clamp(0.0, 100.0);
        pet.happiness = (pet.happiness - 1.0 * time.delta_secs()).clamp(0.0, 100.0);
        pet.energy = (pet.energy - 0.8 * time.delta_secs()).clamp(0.0, 100.0);
    }
}
