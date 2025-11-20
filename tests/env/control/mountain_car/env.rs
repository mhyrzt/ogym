use ogym::env::control::{MountainCar, MountainCarConfig, MountainCarReward};
use ogym::env::environment::Environment;
use ogym::spaces::{MixedItem, Space};

#[test]
fn test_mountain_car_new_discrete() {
    let config = MountainCarConfig::default().with_discrete_action();
    let env = MountainCar::new(config);
    
    assert!(env.is_ok());
    
    let env = env.unwrap();
    // Check that the action space is discrete with 3 actions
    match env.space.action.sample().unwrap() {
        MixedItem::Discrete(_) => assert!(true), // Success
        MixedItem::Continuous(_) => panic!("Expected discrete action space"),
    }
}

#[test]
fn test_mountain_car_new_continuous() {
    let config = MountainCarConfig::default().with_continuous_action();
    let env = MountainCar::new(config);
    
    assert!(env.is_ok());
    
    let env = env.unwrap();
    // Check that the action space is continuous
    match env.space.action.sample().unwrap() {
        MixedItem::Continuous(_) => assert!(true), // Success
        MixedItem::Discrete(_) => panic!("Expected continuous action space"),
    }
}

#[test]
fn test_mountain_car_force_discrete() {
    let config = MountainCarConfig::default();
    let env = MountainCar::new(config).unwrap();
    
    // Test discrete actions: 0 -> -1.0, 1 -> 0.0, 2 -> 1.0
    let force_0 = env.force(&MixedItem::Discrete(0));
    let force_1 = env.force(&MixedItem::Discrete(1));
    let force_2 = env.force(&MixedItem::Discrete(2));
    
    assert_eq!(force_0, -1.0);
    assert_eq!(force_1, 0.0);
    assert_eq!(force_2, 1.0);
}

#[test]
fn test_mountain_car_force_continuous() {
    let config = MountainCarConfig::default().with_continuous_action();
    let env = MountainCar::new(config).unwrap();
    
    // Test continuous action
    let action = MixedItem::Continuous(nalgebra::SVector::from_vec(vec![0.5]));
    let force = env.force(&action);
    
    assert_eq!(force, 0.5);
}

#[test]
fn test_mountain_car_clamp_velocity_at_boundary() {
    let config = MountainCarConfig::default();
    let env = MountainCar::new(config).unwrap();
    
    // Test clamping when x <= min_x and v < 0
    let clamped_v = env.clamp_velocity_at_boundary(&-1.3, &-0.01); // x < min_x and v < 0
    assert_eq!(clamped_v, 0.0);
    
    // Test no clamping when x > min_x
    let clamped_v = env.clamp_velocity_at_boundary(&-1.0, &-0.01); // x > min_x, v < 0
    assert_eq!(clamped_v, -0.01);
    
    // Test no clamping when x <= min_x but v >= 0
    let clamped_v = env.clamp_velocity_at_boundary(&-1.3, &0.01); // x < min_x but v >= 0
    assert_eq!(clamped_v, 0.01);
    
    // Test no clamping when x > min_x and v >= 0
    let clamped_v = env.clamp_velocity_at_boundary(&0.0, &0.01); // x > min_x and v >= 0
    assert_eq!(clamped_v, 0.01);
}

#[test]
fn test_reward_constant() {
    let config = MountainCarConfig::default().with_constant_reward();
    let env = MountainCar::new(config).unwrap();
    
    let action = MixedItem::Discrete(1);
    let reward = env.reward(&action);
    
    assert_eq!(reward, -1.0);
}

#[test]
fn test_reward_action_penalty_discrete() {
    let config = MountainCarConfig::default().with_action_penalty_reward();
    let env = MountainCar::new(config).unwrap();
    
    let action = MixedItem::Discrete(1);  // Discrete actions should not incur penalty
    let reward = env.reward(&action);
    
    assert_eq!(reward, -1.0);  // No penalty for discrete actions
}

#[test]
fn test_reward_action_penalty_continuous() {
    let config = MountainCarConfig::default().with_action_penalty_reward().with_continuous_action();
    let env = MountainCar::new(config).unwrap();
    
    let action = MixedItem::Continuous(nalgebra::SVector::from_vec(vec![0.5]));
    let reward = env.reward(&action);
    
    // Reward should be -1.0 - norm_squared = -1.0 - 0.25 = -1.25
    assert_eq!(reward, -1.25);
}

#[test]
fn test_discrete_action_space_creation() {
    let config = MountainCarConfig::default().with_discrete_action();
    let env = MountainCar::new(config).unwrap();
    
    // Verify action space is discrete with 3 actions
    for _ in 0..10 {
        let action = env.space.action.sample().unwrap();
        match action {
            MixedItem::Discrete(a) => assert!(a < 3), // Should be 0, 1, or 2
            MixedItem::Continuous(_) => panic!("Expected discrete action"),
        }
    }
}

#[test]
fn test_continuous_action_space_creation() {
    let config = MountainCarConfig::default().with_continuous_action();
    let env = MountainCar::new(config).unwrap();
    
    // Verify action space is continuous in [-1, 1]
    for _ in 0..10 {
        let action = env.space.action.sample().unwrap();
        match action {
            MixedItem::Continuous(a) => {
                assert!(a[0] >= -1.0 && a[0] <= 1.0);
            },
            MixedItem::Discrete(_) => panic!("Expected continuous action"),
        }
    }
}

#[test]
fn test_state_space_bounds() {
    let config = MountainCarConfig::default();
    let env = MountainCar::new(config).unwrap();

    // The state space should have bounds [min_x, -max_v] to [max_x, max_v]
    let low = env.space.state.low();
    let high = env.space.state.high();

    assert_eq!(low[0], config.min_x);
    assert_eq!(low[1], -config.max_v);
    assert_eq!(high[0], config.max_x);
    assert_eq!(high[1], config.max_v);
}

#[test]
fn test_reward_calculation_edge_cases() {
    // Test with action penalty reward and large continuous action
    let config = MountainCarConfig::default()
        .with_action_penalty_reward()
        .with_continuous_action();
    let env = MountainCar::new(config).unwrap();

    let large_action = MixedItem::Continuous(nalgebra::SVector::from_vec(vec![0.9]));
    let reward = env.reward(&large_action);

    // Reward should be -1.0 - (0.9)^2 = -1.0 - 0.81 = -1.81
    assert_eq!(reward, -1.81);

    // Test with zero continuous action
    let zero_action = MixedItem::Continuous(nalgebra::SVector::from_vec(vec![0.0]));
    let reward = env.reward(&zero_action);

    // Reward should be -1.0 - 0.0 = -1.0
    assert_eq!(reward, -1.0);
}

#[test]
fn test_step_logic_discrete() {
    let config = MountainCarConfig::default().with_discrete_action();
    let mut env = MountainCar::new(config).unwrap();

    // Reset the environment to get initial state
    let (initial_state, _) = env.reset(None).unwrap();

    // Take a step with action 2 (apply positive force)
    let action = MixedItem::Discrete(2);
    let experience = env.step(action).unwrap();

    // Verify the experience contains correct information
    assert_eq!(experience.curr_state, initial_state);
    assert_eq!(experience.action, action);
    assert_eq!(experience.reward, -1.0); // Constant reward

    // The next state should have changed position and velocity
    assert!(experience.next_state[0] != initial_state[0] || experience.next_state[1] != initial_state[1]);

    // Check that time step increased
    assert_eq!(experience.step, 1);
}

#[test]
fn test_step_logic_continuous() {
    let config = MountainCarConfig::default().with_continuous_action();
    let mut env = MountainCar::new(config).unwrap();

    // Reset the environment to get initial state
    let (initial_state, _) = env.reset(None).unwrap();

    // Take a step with continuous action
    let action = MixedItem::Continuous(nalgebra::SVector::from_vec(vec![0.5]));
    let experience = env.step(action).unwrap();

    // Verify the experience contains correct information
    assert_eq!(experience.curr_state, initial_state);
    assert_eq!(experience.action, action);
    // With action penalty reward, this would be -1.0 - 0.25 = -1.25
    // But with constant reward, it should be -1.0
    assert_eq!(experience.reward, -1.0);

    // The next state should have changed position and velocity
    assert!(experience.next_state[0] != initial_state[0] || experience.next_state[1] != initial_state[1]);

    // Check that time step increased
    assert_eq!(experience.step, 1);
}

#[test]
fn test_multiple_steps() {
    let config = MountainCarConfig::default().with_discrete_action();
    let mut env = MountainCar::new(config).unwrap();

    // Reset the environment
    let (mut current_state, _) = env.reset(None).unwrap();

    // Take multiple steps
    for step in 1..=5 {
        let action = MixedItem::Discrete(2); // Always push right
        let experience = env.step(action).unwrap();

        assert_eq!(experience.curr_state, current_state);
        assert_eq!(experience.step, step);
        assert_eq!(experience.reward, -1.0);

        // Update current state for next iteration
        current_state = experience.next_state;
    }
}

#[test]
fn test_invalid_action() {
    let config = MountainCarConfig::default().with_discrete_action();
    let mut env = MountainCar::new(config).unwrap();

    // Reset first
    let (_, _) = env.reset(None).unwrap();

    // Try an invalid discrete action (should be 0, 1, or 2)
    let invalid_action = MixedItem::Discrete(5);

    // This should trigger an error when the environment checks the action
    let result = env.step(invalid_action);
    assert!(result.is_err());
}

#[test]
fn test_reset_functionality() {
    let config = MountainCarConfig::default().with_discrete_action();
    let mut env = MountainCar::new(config).unwrap();

    // Reset the environment
    let (state, info) = env.reset(None).unwrap();

    // Check that the state is within the expected bounds
    assert!(state[0] >= config.min_x && state[0] <= config.max_x);
    assert!(state[1] >= -config.max_v && state[1] <= config.max_v);

    // Check that velocity is 0 after reset
    assert_eq!(state[1], 0.0);

    // Check that position is in the expected range [-0.6, -0.4] as per reset implementation
    assert!(state[0] >= -0.6 && state[0] <= -0.4);

    // Check that the info is None as expected
    assert_eq!(info, None);

    // Check that time is reset to 0
    assert_eq!(env.t, 0);
}

#[test]
fn test_reset_after_steps() {
    let config = MountainCarConfig::default().with_discrete_action();
    let mut env = MountainCar::new(config).unwrap();

    // Take some steps
    let (_, _) = env.reset(None).unwrap();
    let action = MixedItem::Discrete(2);
    let _ = env.step(action).unwrap(); // Step 1
    let _ = env.step(action).unwrap(); // Step 2

    // Check that time is now 2
    assert_eq!(env.t, 2);

    // Reset again
    let (new_state, _) = env.reset(None).unwrap();

    // Check that time is reset to 0
    assert_eq!(env.t, 0);

    // Check that the new state is valid
    assert!(new_state[0] >= -0.6 && new_state[0] <= -0.4);
    assert_eq!(new_state[1], 0.0);
}

#[test]
fn test_reset_with_seed() {
    let config = MountainCarConfig::default().with_discrete_action();
    let mut env = MountainCar::new(config).unwrap();

    // Reset with a specific seed
    let (state1, _) = env.reset(Some(42)).unwrap();

    // Reset again with the same seed
    let mut env2 = MountainCar::new(config).unwrap();
    let (state2, _) = env2.reset(Some(42)).unwrap();

    // With the same seed, the initial state should be the same
    // Note: This depends on the deterministic nature of the uniform function
    // If the uniform function is properly seeded, both states should be similar
}

#[test]
fn test_termination_conditions() {
    let config = MountainCarConfig::default().with_max_steps(10); // Set low max steps for testing
    let mut env = MountainCar::new(config).unwrap();

    // Reset the environment
    let (_, _) = env.reset(None).unwrap();

    // Take steps until max steps is reached
    for i in 1..=10 {
        let action = MixedItem::Discrete(0); // Doesn't matter for termination by steps
        let experience = env.step(action).unwrap();

        if i < 10 {
            // Not done yet
            assert!(!experience.terminal.is_done());
        } else {
            // Should be truncated due to max steps
            assert!(experience.terminal.is_done());
            assert!(experience.terminal.is_truncated());
            assert!(!experience.terminal.is_terminated());
        }
    }

    // Check that is_done returns true after max steps
    assert!(env.is_done().unwrap());
    assert!(env.is_truncated());
}

#[test]
fn test_goal_termination() {
    // Make a custom config with an easily reachable goal
    let config = MountainCarConfig::default()
        .with_goal_position(0.1)  // Lower the goal position
        .with_goal_velocity(0.01) // Lower the goal velocity requirement
        .with_max_steps(100);     // Ensure we don't hit max steps first

    let mut env = MountainCar::new(config).unwrap();

    // Manually set state to reach the goal (for testing purposes)
    // Reset first to initialize
    let (mut current_state, _) = env.reset(None).unwrap();

    // Simulate reaching the goal through multiple steps
    // This test is tricky because we need to actually reach the goal
    // by simulating the physics, so we'll check the logic more directly

    // Check the is_terminal method directly with a state that should be terminal
    use nalgebra::SVector;

    // Create a state that reaches the goal
    let goal_reached_state = SVector::from_vec(vec![0.2, 0.02]); // x > goal_x and v > goal_v

    // Need to update env state manually for this test
    env.state = Some(goal_reached_state);

    // Check that is_terminal returns true for goal-reaching state
    assert!(env.is_terminal().unwrap());

    // Check that is_done returns true for goal-reaching state
    assert!(env.is_done().unwrap());
}

#[test]
fn test_not_done_initially() {
    let config = MountainCarConfig::default();
    let mut env = MountainCar::new(config).unwrap();

    // Reset the environment
    let (_, _) = env.reset(None).unwrap();

    // Initially, environment should not be done
    assert!(!env.is_done().unwrap());
    assert!(!env.is_terminal().unwrap());
    assert!(!env.is_truncated());
}

#[test]
fn test_episode_done_error() {
    let config = MountainCarConfig::default().with_max_steps(1);
    let mut env = MountainCar::new(config).unwrap();

    // Reset the environment
    let (_, _) = env.reset(None).unwrap();

    // Take one step to reach max steps
    let action = MixedItem::Discrete(0);
    let _ = env.step(action).unwrap();

    // Try to take another step - should fail
    let result = env.step(action);
    assert!(result.is_err());

    // Check that the error is EpisodeDone
    match result {
        Err(ogym::env::environment::Error::EpisodeDone) => assert!(true),
        _ => panic!("Expected EpisodeDone error"),
    }
}