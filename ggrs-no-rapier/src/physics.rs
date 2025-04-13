use std::{any::TypeId, marker::PhantomData};

use bevy::{ecs::component::ComponentId, math::bounding::{Aabb2d, IntersectsVolume}};

use crate::prelude::*;

#[derive(Default)]
pub struct PhysicsPlugin{
    // collision_types: Vec<(TypeId, TypeId)>
}

// impl PhysicsPlugin {
//     fn add_collision_type<T: Component + 'static, U: Component + 'static>(mut self: Self) -> Self {
//         self.collision_types.push((TypeId::of::<T>(), TypeId::of::<U>()));
//         return self;
//     }
// }

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(GgrsSchedule,
            (
                apply_gravity,
                apply_velocity,
                detect_collisions::<Fighter, Fighter>,
                detect_collisions::<Hitbox, Hurtbox>
                
            )
            .chain()
            // TODO there should be a better way to do this
            .after(apply_inputs)
        );
        // TODO insert more physics things
    }
}

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    velocity: Velocity,
    gravity: Gravity,
}

#[derive(Component, Default)]
pub struct Velocity(Vec2);

fn apply_velocity(
    mut objects: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>
) {
    for (mut transform, vel) in &mut objects {
        transform.translation += time.delta_secs() * vel.0.extend(0.);
    }
}

#[derive(Component, Default)]
pub struct Gravity;

fn apply_gravity(
    mut gravity_objects: Query<&mut Velocity, With<Gravity>>
) {
    for mut v in &mut gravity_objects {
        v.0.y -= 9.8;
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Collision {
    offset: Vec2,
    half_size: Vec2,
}

enum Side {
    Left,
    Right
}

#[derive(Event)]
pub struct CollisionEvent<
    T: Send + Sync + 'static,
    U: Send + Sync + 'static
>{
    entity1: Entity,
    entity2: Entity,
    side: Side, 
    _p1: PhantomData<T>,
    _p2: PhantomData<U>
}

impl <T: Send + Sync + 'static, U: Send + Sync + 'static> CollisionEvent<T, U> {
    fn new(entity1: Entity, entity2: Entity, side: Side) -> CollisionEvent<T,U> {
        CollisionEvent{
            entity1,
            entity2,
            side,
            _p1: PhantomData,
            _p2: PhantomData
        }
    }
}

fn detect_collisions<
T: Send + Sync + Component + 'static,
U: Send + Sync + Component + 'static
>(
    boxes: Query<(Entity, &Transform, &Collision), With<T>>,
    other_boxes: Query<(Entity, &Transform, &Collision), With<U>>,
    mut events: EventWriter<CollisionEvent<T, U>>
) {
    for (e1,t1,c1) in boxes.iter() {
        for (e2,t2,c2) in other_boxes.iter() {
            if e1 != e2 {
                let aabb1 = get_collision_aabb(&t1, c1);
                let aabb2 = get_collision_aabb(&t2, c2);

                if aabb1.intersects(&aabb2) {
                    let side = if t1.translation.x < t2.translation.x {
                        Side::Left
                    } else {
                        Side::Right
                    };
                    events.send(CollisionEvent::new(
                            e1,
                            e2,
                            side
                    ));
                }
            }
        }
    }
}

fn get_collision_aabb(t: &Transform, c: &Collision) -> Aabb2d {
    Aabb2d::new(t.translation.truncate() + c.offset, c.half_size)
}

#[derive(Component)]
struct Hitbox;

#[derive(Component)]
struct Hurtbox;
