use rapier2d::prelude::*;

pub struct PhysicsEngine {

}

impl PhysicsEngine {
    pub fn new() {

    }

    pub fn update(dt: f32) {

    }
}


mod tests {

    use super::*;

    #[test]
    fn main() {

        let mut physics_pipeline = PhysicsPipeline::new();
        let gravity = vector![0.0, -9.81];
        let integration_parameters = IntegrationParameters::default();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = BroadPhaseBvh::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let ball_radius = 1.0;
        let ball_pos = point![0.0, 10.0];

        let ball_rigid_body = RigidBodyBuilder::dynamic()
            .translation(ball_pos.coords)
            .build();

        let ball_collider = ColliderBuilder::ball(ball_radius)
            .restitution(0.7)
            .build();

        let ball_handle = rigid_body_set.insert(ball_rigid_body);
        collider_set.insert_with_parent(ball_collider, ball_handle, &mut rigid_body_set);

        let ground = RigidBodyBuilder::fixed()
            .translation(vector![0.0, -5.0])
            .build();
        
        let ground_collider = ColliderBuilder::cuboid(50.0, 1.0).build();
        let ground_handle = rigid_body_set.insert(ground);
        collider_set.insert_with_parent(ground_collider, ground_handle, &mut rigid_body_set);

        for step in 0..100 {
            physics_pipeline.step(
                &gravity,
                &integration_parameters,
                &mut island_manager,
                &mut broad_phase,
                &mut narrow_phase,
                &mut rigid_body_set,
                &mut collider_set,
                &mut impulse_joint_set,
                &mut multibody_joint_set,
                &mut ccd_solver,
                &(),
                &()
            );

            if let Some(ball) = rigid_body_set.get(ball_handle) {
                let position = ball.translation();
                println!("Шаг {}: Позиция шара = ({:.2}, {:.2})", step, position.x, position.y);
            }
        }

        if let Some(ball) = rigid_body_set.get(ball_handle) {
            let final_position = ball.translation();
            println!("\nФинальная позиция шара: ({:.2}, {:.2})", final_position.x, final_position.y);
        }
    }
}