use bevy::{ecs::component::Component, reflect::Reflect};

//pub(crate) fn damage_from_attacks(
//    mut damage_events: EventWriter<DamageEvent>,
//    mut projectile_events: EventReader<AttackEvent>,
//    damagers: Query<&Stat<Damage>, Or<(With<Weapon>, With<Actor>)>>,
//) {
//    for AttackEvent {
//        attacker,
//        weapon,
//        defender,
//        location: _,
//        direction: _,
//    } in projectile_events.read()
//    {
//        if let Ok(damage) = damagers.get(*weapon) {
//            damage_events.send(DamageEvent {
//                target: *defender,
//                source: *attacker,
//                amount: damage.current_value(),
//            });
//        }
//    }
//}
//
//fn impart_knockback(
//    mut knockback_events: EventReader<KnockbackEvent>,
//    mut target_query: Query<&mut ExternalImpulse>,
//) {
//    for KnockbackEvent {
//        entity,
//        direction,
//        force,
//    } in knockback_events.read()
//    {
//        let impulse_vector = direction.normalize() * *force * 3000.;
//        if let Ok(mut impulse) = target_query.get_mut(*entity) {
//            impulse.apply_impulse(impulse_vector);
//        }
//    }
//}
