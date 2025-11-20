use ogym::env::control::MountainCarConfig;
use ogym::env::control::MountainCarReward;

#[test]
fn test_mountain_car_config_default() {
    let config = MountainCarConfig::default();
    
    assert_eq!(config.f, 1e-3);
    assert_eq!(config.g, 25e-4);
    assert_eq!(config.min_x, -1.2);
    assert_eq!(config.max_x, 0.6);
    assert_eq!(config.max_v, 7e-2);
    assert_eq!(config.max_t, 200);
    assert_eq!(config.goal_x, 0.5);
    assert_eq!(config.goal_v, 0.0);
    assert_eq!(config.continuous, false);
    assert_eq!(config.reward, MountainCarReward::Constant);
}

#[test]
fn test_mountain_car_config_new() {
    let config = MountainCarConfig::new();
    let default_config = MountainCarConfig::default();
    
    assert_eq!(config, default_config);
}

#[test]
fn test_with_force() {
    let config = MountainCarConfig::default().with_force(0.5);
    
    assert_eq!(config.f, 0.5);
    // Ensure other fields remain unchanged
    assert_eq!(config.g, 25e-4);
}

#[test]
fn test_with_gravity() {
    let config = MountainCarConfig::default().with_gravity(0.1);
    
    assert_eq!(config.g, 0.1);
    // Ensure other fields remain unchanged
    assert_eq!(config.f, 1e-3);
}

#[test]
fn test_with_max_steps() {
    let config = MountainCarConfig::default().with_max_steps(500);
    
    assert_eq!(config.max_t, 500);
    // Ensure other fields remain unchanged
    assert_eq!(config.max_v, 7e-2);
}

#[test]
fn test_with_min_position() {
    let config = MountainCarConfig::default().with_min_position(-2.0);
    
    assert_eq!(config.min_x, -2.0);
    // Ensure other fields remain unchanged
    assert_eq!(config.max_x, 0.6);
}

#[test]
fn test_with_max_position() {
    let config = MountainCarConfig::default().with_max_position(1.0);
    
    assert_eq!(config.max_x, 1.0);
    // Ensure other fields remain unchanged
    assert_eq!(config.min_x, -1.2);
}

#[test]
fn test_with_max_velocity() {
    let config = MountainCarConfig::default().with_max_velocity(0.1);
    
    assert_eq!(config.max_v, 0.1);
    // Ensure other fields remain unchanged
    assert_eq!(config.max_t, 200);
}

#[test]
fn test_with_goal_position() {
    let config = MountainCarConfig::default().with_goal_position(0.8);
    
    assert_eq!(config.goal_x, 0.8);
    // Ensure other fields remain unchanged
    assert_eq!(config.goal_v, 0.0);
}

#[test]
fn test_with_goal_velocity() {
    let config = MountainCarConfig::default().with_goal_velocity(0.5);
    
    assert_eq!(config.goal_v, 0.5);
    // Ensure other fields remain unchanged
    assert_eq!(config.goal_x, 0.5);
}

#[test]
fn test_with_discrete_action() {
    let config = MountainCarConfig::default().with_continuous_action().with_discrete_action();
    
    assert_eq!(config.continuous, false);
    // Ensure other fields remain unchanged
    assert_eq!(config.f, 1e-3);
}

#[test]
fn test_with_continuous_action() {
    let config = MountainCarConfig::default().with_continuous_action();
    
    assert_eq!(config.continuous, true);
    // Ensure other fields remain unchanged
    assert_eq!(config.f, 1e-3);
}

#[test]
fn test_with_constant_reward() {
    let config = MountainCarConfig::default().with_action_penalty_reward().with_constant_reward();
    
    assert_eq!(config.reward, MountainCarReward::Constant);
    // Ensure other fields remain unchanged
    assert_eq!(config.f, 1e-3);
}

#[test]
fn test_with_action_penalty_reward() {
    let config = MountainCarConfig::default().with_action_penalty_reward();
    
    assert_eq!(config.reward, MountainCarReward::ActionPenalty);
    // Ensure other fields remain unchanged
    assert_eq!(config.f, 1e-3);
}

#[test]
fn test_mountain_car_reward_equality() {
    assert_eq!(MountainCarReward::Constant, MountainCarReward::Constant);
    assert_eq!(MountainCarReward::ActionPenalty, MountainCarReward::ActionPenalty);
    assert_ne!(MountainCarReward::Constant, MountainCarReward::ActionPenalty);
    assert_ne!(MountainCarReward::ActionPenalty, MountainCarReward::Constant);
}

#[test]
fn test_mountain_car_config_clone() {
    let config1 = MountainCarConfig::default();
    let config2 = config1.clone();
    
    assert_eq!(config1, config2);
}

#[test]
fn test_mountain_car_config_debug() {
    let config = MountainCarConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("MountainCarConfig"));
    assert!(debug_str.contains("f"));
    assert!(debug_str.contains("g"));
}

#[test]
fn test_mountain_car_config_different_configs() {
    let config1 = MountainCarConfig::default();
    let config2 = MountainCarConfig::default().with_force(0.5);
    
    assert_ne!(config1, config2);
}

#[test]
fn test_config_builder_pattern() {
    let config = MountainCarConfig::default()
        .with_force(0.01)
        .with_gravity(0.02)
        .with_max_steps(300)
        .with_min_position(-1.5)
        .with_max_position(0.8)
        .with_max_velocity(0.08)
        .with_goal_position(0.7)
        .with_goal_velocity(0.1)
        .with_continuous_action()
        .with_action_penalty_reward();
    
    assert_eq!(config.f, 0.01);
    assert_eq!(config.g, 0.02);
    assert_eq!(config.max_t, 300);
    assert_eq!(config.min_x, -1.5);
    assert_eq!(config.max_x, 0.8);
    assert_eq!(config.max_v, 0.08);
    assert_eq!(config.goal_x, 0.7);
    assert_eq!(config.goal_v, 0.1);
    assert_eq!(config.continuous, true);
    assert_eq!(config.reward, MountainCarReward::ActionPenalty);
}