---
id: TASK-6
title: 'Fix LunarLander: reset(seed) ignored, is_terminal() inconsistent with step()'
status: Done
assignee: []
created_date: '2026-07-13 12:42'
updated_date: '2026-07-13 22:40'
labels: []
dependencies: []
priority: medium
ordinal: 6000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
src/env/rapier/lunar_lander/env.rs's reset() (env.rs:411) uses the global rand::rng() directly instead of a seeded RNG, so reset(seed) is non-reproducible despite the Environment trait's reset(seed: Option<u64>) signature implying seed control. BipedalWalker already does this correctly (stores a StdRng, reseeds via SeedableRng::seed_from_u64 in reset(seed)) so the pattern is known. Separately, step() computes terminated = is_game_over() || is_landed() (crash/out-of-screen/landed), but the standalone Environment::is_terminal() (env.rs:476-482) only re-checks out-of-screen, so it disagrees with step()'s terminated flag if called independently.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 LunarLander stores a seeded StdRng (mirroring BipedalWalker's pattern) and reset(Some(seed)) produces identical episodes across runs, verified by a test
- [x] #2 is_terminal() accounts for crash and landed state, not just out-of-screen, matching step()'s terminated computation
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added rng: StdRng field (seeded with a fixed default at construction, mirroring BipedalWalker's pattern), reseeded in reset(seed) when seed is Some. Replaced the two rand::rng() (global thread-local) call sites -- create_lander's initial impulse and apply_engine_forces' per-step dispersion -- with self.rng. This also incidentally fixed a pre-existing test flakiness: several LunarLander tests constructed via new() (which calls reset(None)) were previously drawing from the SAME global RNG as other tests running in parallel, causing occasional divergent physics outcomes (test_step_main_engine_discrete/continuous failed intermittently under 
running 289 tests
test env::control::acrobot::config::tests::test_default_values ... ok
test env::control::acrobot::config::tests::test_dynamics_mode_switching ... ok
test env::control::acrobot::config::tests::test_new ... ok
test env::control::acrobot::config::tests::test_with_link_lengths ... ok
test env::control::acrobot::config::tests::test_with_dt ... ok
test env::control::acrobot::config::tests::test_with_link_masses ... ok
test env::control::acrobot::config::tests::test_with_link_moi ... ok
test env::control::acrobot::config::tests::test_with_gravity ... ok
test env::control::acrobot::config::tests::test_with_link_com_positions ... ok
test env::control::acrobot::config::tests::test_with_max_steps ... ok
test env::control::acrobot::config::tests::test_with_max_velocities ... ok
test env::control::acrobot::config::tests::test_with_torque_noise ... ok
test env::control::acrobot::env::tests::test_constraint_velocity_clamping ... ok
test env::control::acrobot::env::tests::test_constraint_wrapping ... ok
test env::control::acrobot::env::tests::test_initialization_continuous ... ok
test env::control::acrobot::env::tests::test_initialization_discrete ... ok
test env::control::acrobot::env::tests::test_reset ... ok
test env::control::acrobot::env::tests::test_step_continuous_actions ... ok
test env::control::acrobot::config::tests::test_builder_chaining ... ok
test env::control::acrobot::env::tests::test_step_discrete_actions ... ok
test env::control::acrobot::env::tests::test_step_invalid_action_bounds ... ok
test env::control::acrobot::env::tests::test_step_invalid_action_mismatch ... ok
test env::control::acrobot::env::tests::test_terminal_condition ... ok
test env::control::acrobot::env::tests::test_torque_noise ... ok
test env::control::acrobot::config::tests::test_action_space_type_switching ... ok
test env::control::acrobot::env::tests::test_truncation ... ok
test env::control::cart_pole::config::tests::test_action_toggle ... ok
test env::control::cart_pole::config::tests::test_builder_methods_numeric ... ok
test env::control::cart_pole::config::tests::test_default_config ... ok
test env::control::cart_pole::config::tests::test_integrator_toggle ... ok
test env::control::cart_pole::config::tests::test_new_config ... ok
test env::control::cart_pole::config::tests::test_theta_configuration ... ok
test env::control::cart_pole::env::tests::test_compute_acceleration_values ... ok
test env::control::cart_pole::config::tests::test_clone_and_debug ... ok
test env::control::cart_pole::env::tests::test_euler_and_semi_implicit ... ok
test env::control::cart_pole::env::tests::test_integrate_switches ... ok
test env::control::cart_pole::env::tests::test_is_terminal_true_and_false ... ok
test env::control::cart_pole::env::tests::test_force_discrete_and_continuous ... ok
test env::control::cart_pole::env::tests::test_is_truncated ... ok
test env::control::cart_pole::env::tests::test_new_discrete_and_continuous ... ok
test env::control::cart_pole::env::tests::test_state_and_not_initialized ... ok
test env::control::cart_pole::env::tests::test_step_advances_and_returns_experience ... ok
test env::control::cart_pole::env::tests::test_step_errors_if_done ... ok
test env::control::mountain_car::config::tests::test_builder_methods ... ok
test env::control::mountain_car::config::tests::test_clone_and_debug ... ok
test env::control::mountain_car::config::tests::test_continuous_action_toggle ... ok
test env::control::mountain_car::config::tests::test_default_config ... ok
test env::control::cart_pole::env::tests::test_reset_initializes_state ... ok
test env::control::mountain_car::config::tests::test_new_config ... ok
test env::control::mountain_car::config::tests::test_reward_toggle ... ok
test env::control::mountain_car::env::tests::test_episode_done_error ... ok
test env::control::mountain_car::env::tests::test_goal_termination ... ok
test env::control::mountain_car::env::tests::test_mountain_car_clamp_velocity_at_boundary ... ok
test env::control::mountain_car::env::tests::test_mountain_car_config_clone ... ok
test env::control::mountain_car::env::tests::test_mountain_car_config_debug ... ok
test env::control::mountain_car::env::tests::test_config_builder_pattern ... ok
test env::control::mountain_car::env::tests::test_mountain_car_config_default ... ok
test env::control::mountain_car::env::tests::test_mountain_car_config_different_configs ... ok
test env::control::mountain_car::env::tests::test_mountain_car_config_new ... ok
test env::control::mountain_car::env::tests::test_mountain_car_force_continuous ... ok
test env::control::mountain_car::env::tests::test_mountain_car_force_discrete ... ok
test env::control::mountain_car::env::tests::test_mountain_car_new_continuous ... ok
test env::control::mountain_car::env::tests::test_mountain_car_new_discrete ... ok
test env::control::mountain_car::env::tests::test_mountain_car_reward_equality ... ok
test env::control::mountain_car::env::tests::test_physics_simulation ... ok
test env::control::mountain_car::env::tests::test_physics_with_velocity_clamping ... ok
test env::control::mountain_car::env::tests::test_invalid_action ... ok
test env::control::mountain_car::env::tests::test_reset_functionality ... ok
test env::control::mountain_car::env::tests::test_reward_action_penalty_discrete ... ok
test env::control::mountain_car::env::tests::test_reward_constant ... ok
test env::control::mountain_car::env::tests::test_state_method_error ... ok
test env::control::mountain_car::env::tests::test_state_space_bounds ... ok
test env::control::mountain_car::env::tests::test_step_logic_discrete ... ok
test env::control::mountain_car::env::tests::test_termination_conditions ... ok
test env::control::mountain_car::env::tests::test_with_action_penalty_reward ... ok
test env::control::mountain_car::env::tests::test_with_constant_reward ... ok
test env::control::mountain_car::env::tests::test_with_continuous_action ... ok
test env::control::mountain_car::env::tests::test_with_discrete_action ... ok
test env::control::mountain_car::env::tests::test_with_force ... ok
test env::control::mountain_car::env::tests::test_reward_action_penalty_continuous ... ok
test env::control::mountain_car::env::tests::test_with_goal_velocity ... ok
test env::control::mountain_car::env::tests::test_with_gravity ... ok
test env::control::mountain_car::env::tests::test_with_max_position ... ok
test env::control::mountain_car::env::tests::test_with_max_steps ... ok
test env::control::mountain_car::env::tests::test_with_max_velocity ... ok
test env::control::mountain_car::env::tests::test_with_min_position ... ok
test env::control::pendulum::config::tests::test_builder_chaining ... ok
test env::control::pendulum::config::tests::test_builder_continuous_action ... ok
test env::control::pendulum::config::tests::test_builder_discrete_action ... ok
test env::control::mountain_car::env::tests::test_with_goal_position ... ok
test env::control::pendulum::config::tests::test_builder_force ... ok
test env::control::pendulum::config::tests::test_builder_gravity ... ok
test env::control::pendulum::config::tests::test_builder_initial_angle ... ok
test env::control::pendulum::config::tests::test_builder_initial_velocity ... ok
test env::control::pendulum::config::tests::test_builder_length ... ok
test env::control::pendulum::config::tests::test_builder_mass ... ok
test env::control::pendulum::config::tests::test_builder_max_steps ... ok
test env::control::pendulum::config::tests::test_builder_max_torque ... ok
test env::control::pendulum::config::tests::test_builder_max_velocity ... ok
test env::control::pendulum::config::tests::test_builder_timestep ... ok
test env::control::pendulum::config::tests::test_debug_implementation ... ok
test env::control::pendulum::config::tests::test_default_values ... ok
test env::control::pendulum::config::tests::test_new ... ok
test env::control::pendulum::env::tests::test_cost_calculation ... ok
test env::control::pendulum::env::tests::test_invalid_action_space ... ok
test env::control::pendulum::env::tests::test_new_initialization ... ok
test env::control::pendulum::env::tests::test_normalize_angle ... ok
test env::control::pendulum::env::tests::test_reset ... ok
test env::control::pendulum::env::tests::test_state_getter ... ok
test env::control::pendulum::config::tests::test_clone_and_copy ... ok
test env::control::pendulum::env::tests::test_step_continuous_logic ... ok
test env::control::pendulum::env::tests::test_step_uninitialized ... ok
test env::control::pendulum::env::tests::test_truncation ... ok
test env::environment::error::tests::test_clone_and_partial_eq ... ok
test env::environment::error::tests::test_error_messages_simple_variants ... ok
test env::environment::error::tests::test_error_messages_with_fields ... ok
test env::environment::experience::tests::test_experience_clone_and_copy ... ok
test env::environment::experience::tests::test_experience_debug ... ok
test env::environment::experience::tests::test_experience_equality ... ok
test env::environment::experience::tests::test_experience_new ... ok
test env::environment::single::tests::test_is_done_default_impl ... ok
test env::environment::single::tests::test_mock_interaction ... ok
test env::environment::single::tests::test_to_terminal_default_impl ... ok
test env::control::pendulum::env::tests::test_step_discrete_logic ... ok
test env::environment::terminal::tests::test_from_flags ... ok
test env::environment::terminal::tests::test_terminal_both ... ok
test env::environment::terminal::tests::test_terminal_ongoing ... ok
test env::environment::terminal::tests::test_terminal_terminate ... ok
test env::environment::terminal::tests::test_terminal_truncate ... ok
test env::mujoco::ant::config::tests::test_builder_pattern ... ok
test env::mujoco::ant::env::tests::test_builder_methods_compile ... ok
test env::environment::terminal::tests::test_derive_traits ... ok
test env::mujoco::half_cheetah::config::tests::test_builder_customization ... ok
test env::mujoco::ant::env::tests::test_env_with_mock_xml ... ok
test env::mujoco::half_cheetah::config::tests::test_builder_defaults ... ok
test env::mujoco::half_cheetah::env::tests::test_env_initialization ... ok
test env::mujoco::ant::env::tests::test_step_cycle ... ok
test env::mujoco::half_cheetah::env::tests::test_observation_dimensions ... ok
test env::mujoco::half_cheetah::env::tests::test_truncation_at_max_episode_steps ... ok
test env::mujoco::half_cheetah::config::tests::test_invalid_frame_skip - should panic ... ok
test env::mujoco::half_cheetah::env::tests::test_step_logic_and_reward ... ok
test env::mujoco::half_cheetah::config::tests::test_invalid_noise_scale - should panic ... ok
test env::mujoco::humanoid::config::tests::test_default_values ... ok
test env::mujoco::half_cheetah::env::tests::test_reset_determinism_and_noise ... ok
test env::mujoco::humanoid::config::tests::test_builder_pattern ... ok
test env::mujoco::half_cheetah::env::tests::test_action_bounds_check ... ok
test env::mujoco::ant::env::tests::test_env_with_real_default_xml ... ok
test env::mujoco::hopper::env::tests::test_truncation_at_max_episode_steps ... ok
test env::mujoco::humanoid::env::tests::test_functional_center_of_mass ... ok
test env::mujoco::humanoid::env::tests::test_observation_dimensions ... ok
test env::mujoco::inverted_double_pendulum::env::tests::test_observation_dimension_and_layout ... ok
test env::mujoco::inverted_double_pendulum::env::tests::test_termination_based_on_tip_height ... ok
test env::mujoco::humanoid::env::tests::test_reset_noise_functional ... ok
test env::mujoco::humanoid::env::tests::test_sanity_initialization ... ok
test env::mujoco::mjenv::tests::test_body_lookup_and_position ... ok
test env::mujoco::humanoid::env::tests::test_step_cycle_and_physics ... ok
test env::mujoco::inverted_double_pendulum::env::tests::test_truncation_at_max_episode_steps ... ok
test env::mujoco::inverted_double_pendulum::env::tests::test_reward_matches_tip_based_formula ... ok
test env::mujoco::mjenv::tests::test_initialization_failure ... ok
test env::mujoco::mjenv::tests::test_initialization_success ... ok
test env::mujoco::mjenv::tests::test_matrix_data_access ... ok
test env::mujoco::mjenv::tests::test_set_state_and_reset ... ok
test env::mujoco::mjenv::tests::test_simulation_integration ... ok
test env::mujoco::inverted_pendulum::env::tests::test_truncation_at_max_episode_steps ... ok
test env::mujoco::mjenv::tests::test_state_vector_concatenation ... ok
test env::mujoco::reacher::env::tests::test_observation_dimension_matches_config ... ok
test env::mujoco::humanoid::env::tests::test_truncation ... ok
test env::mujoco::pusher::env::tests::test_body_lookups_match_model_xml ... ok
test env::mujoco::pusher::env::tests::test_reward_uses_correct_body_positions ... ok
test env::mujoco::reacher::env::tests::test_reward_uses_correct_body_positions ... ok
test env::mujoco::reacher::env::tests::test_target_body_resolves_to_target_joints_not_arm_links ... ok
test env::mujoco::pusher::env::tests::test_truncation_at_max_episode_steps ... ok
test env::rapier::bipedal_walker::config::tests::test_builder_methods ... ok
test env::rapier::bipedal_walker::config::tests::test_clone ... ok
test env::rapier::bipedal_walker::config::tests::test_builder_chaining_order ... ok
test env::rapier::bipedal_walker::config::tests::test_default_scaled_hull_vertices ... ok
test env::rapier::bipedal_walker::config::tests::test_get_scaled_hull_vertices ... ok
test env::rapier::bipedal_walker::config::tests::test_default_configuration_values ... ok
test env::mujoco::reacher::env::tests::test_truncation_at_max_episode_steps ... ok
test env::mujoco::pusher::env::tests::test_observation_dimension_matches_config ... ok
test env::rapier::bipedal_walker::config::tests::test_serialization ... ok
test env::rapier::bipedal_walker::config::tests::test_new_same_as_default ... ok
test env::mujoco::walker2d::env::tests::test_truncation_at_max_episode_steps ... ok
test env::rapier::bipedal_walker::env::tests::test_bipedal_walker_instantiation ... ok
test env::rapier::bipedal_walker::env::tests::test_compute_state_vector_integrity ... ok
test env::mujoco::swimmer::env::tests::test_truncation_at_max_episode_steps ... ok
test env::rapier::bipedal_walker::env::tests::test_destroy_world_cleanup ... ok
test env::rapier::bipedal_walker::env::tests::test_hardcore_mode_config ... ok
test env::mujoco::humanoid_standup::env::tests::test_truncation_at_max_episode_steps ... ok
test env::rapier::bipedal_walker::env::tests::test_legs_and_joints_are_constructed ... ok
test env::rapier::bipedal_walker::env::tests::test_deterministic_seeding ... ok
test env::rapier::bipedal_walker::env::tests::test_different_seeds_produce_different_terrains ... ok
test env::rapier::bipedal_walker::env::tests::test_reset_initializes_physics_world ... ok
test env::rapier::bipedal_walker::terrain::tests::test_create_polygon_terrain ... ok
test env::rapier::bipedal_walker::terrain::tests::test_coordinate_monotonicity ... ok
test env::rapier::bipedal_walker::terrain::tests::test_generate_basic_structure ... ok
test env::rapier::bipedal_walker::terrain::tests::test_generate_hardcore_mode ... ok
test env::rapier::bipedal_walker::env::tests::test_lidar_reflects_real_distances ... ok
test env::rapier::bipedal_walker::terrain::tests::test_terrain_generator_creation ... ok
test env::rapier::bipedal_walker::utils::tests::test_clear_collisions_empty_queue ... ok
test env::rapier::bipedal_walker::utils::tests::test_leg_data_debug_clone ... ok
test env::rapier::bipedal_walker::utils::tests::test_clear_collisions_drains_queue ... ok
test env::rapier::bipedal_walker::env::tests::test_step_increases_counters ... ok
test env::rapier::bipedal_walker::terrain::tests::test_startpad_flatness ... ok
test env::rapier::lunar_lander::config::tests::test_action_space_configuration ... ok
test env::rapier::bipedal_walker::utils::tests::test_leg_data_modification ... ok
test env::rapier::lunar_lander::config::tests::test_default_configuration ... ok
test env::rapier::bipedal_walker::utils::tests::test_leg_data_new ... ok
test env::rapier::lunar_lander::config::tests::test_builder_compound_setters ... ok
test env::rapier::lunar_lander::config::tests::test_builder_simple_setters ... ok
test env::rapier::lunar_lander::config::tests::test_wind_configuration ... ok
test env::rapier::lunar_lander::config::tests::test_new_alias ... ok
test env::rapier::lunar_lander::config::tests::test_scaled_dimension_calculations ... ok
test env::rapier::lunar_lander::config::tests::test_traits ... ok
test env::rapier::lunar_lander::env::tests::test_action_space_modes ... ok
test env::rapier::lunar_lander::env::tests::test_initialization_and_reset ... ok
test env::rapier::lunar_lander::env::tests::test_out_of_bounds_detection ... ok
test env::rapier::lunar_lander::env::tests::test_is_terminal_matches_step_terminated_on_landing ... ok
test env::rapier::lunar_lander::env::tests::test_is_terminal_matches_step_terminated_on_crash ... ok
test env::rapier::bipedal_walker::env::tests::test_motor_control_clamping ... ok
test env::rapier::lunar_lander::env::tests::test_reward_shaping ... ok
test env::rapier::lunar_lander::env::tests::test_step_main_engine_discrete ... ok
test env::rapier::lunar_lander::env::tests::test_wind_application ... ok
test env::rapier::lunar_lander::terrain::tests::test_helipad_flatness ... ok
test env::rapier::lunar_lander::terrain::tests::test_moon_colliders_count ... ok
test env::rapier::lunar_lander::env::tests::test_truncation ... ok
test env::rapier::lunar_lander::env::tests::test_step_main_engine_continuous ... ok
test env::rapier::lunar_lander::terrain::tests::test_moon_colliders_properties ... ok
test env::rapier::lunar_lander::terrain::tests::test_moon_generation_basics ... ok
test env::rapier::lunar_lander::utils::tests::test_constants_validity ... ok
test env::rapier::lunar_lander::utils::tests::test_helipad_default ... ok
test env::rapier::lunar_lander::utils::tests::test_helipad_instantiation ... ok
test env::rapier::lunar_lander::utils::tests::test_lander_poly_values ... ok
test env::rapier::lunar_lander::utils::tests::test_leg_instantiation ... ok
test env::rapier::world::tests::test_add_rigid_body_and_collider ... ok
test env::rapier::world::tests::test_default_physics_world ... ok
test env::rapier::world::tests::test_new_physics_world_initialization ... ok
test env::rapier::world::tests::test_reset_world ... ok
test env::rapier::world::tests::test_step_with_dt_simulation ... ok
test spaces::boxed::tests::test_bounds_trait ... ok
test env::rapier::world::tests::test_collision_event_handling ... ok
test spaces::boxed::tests::test_contains ... ok
test spaces::boxed::tests::test_debug_clone_partial_eq ... ok
test spaces::boxed::tests::test_new_equal_bounds ... ok
test spaces::boxed::tests::test_new_valid_bounds ... ok
test env::rapier::world::tests::test_step_simulation ... ok
test spaces::boxed::tests::test_shape ... ok
test spaces::boxed::tests::test_uniform_helper_invalid_range ... ok
test spaces::boxed::tests::test_uniform_helper_seeded ... ok
test spaces::discrete::tests::test_bounds ... ok
test spaces::discrete::tests::test_contains ... ok
test spaces::discrete::tests::test_debug_clone_partial_eq ... ok
test spaces::discrete::tests::test_new_invalid_size ... ok
test spaces::boxed::tests::test_sample ... ok
test spaces::discrete::tests::test_new_valid_size ... ok
test spaces::boxed::tests::test_new_invalid_bounds ... ok
test env::rapier::lunar_lander::env::tests::test_step_gravity ... ok
test spaces::discrete::tests::test_shape ... ok
test spaces::error::tests::test_clone_and_copy ... ok
test spaces::error::tests::test_equality ... ok
test spaces::error::tests::test_error_display_implementation ... ok
test spaces::error::tests::test_error_traits ... ok
test spaces::error::tests::test_from_rand_error ... ok
test spaces::mixed::tests::test_contains_type_mismatch ... ok
test spaces::mixed::tests::test_contains_valid_types_check_bounds ... ok
test spaces::discrete::tests::test_sample_range ... ok
test spaces::mixed::tests::test_shape_delegation ... ok
test spaces::multi_discrete::tests::test_contains ... ok
test spaces::multi_discrete::tests::test_conversion_logic_simple ... ok
test spaces::mixed::tests::test_continuous_construction_and_sample ... ok
test spaces::mixed::tests::test_mixed_bounds ... ok
test spaces::multi_discrete::tests::test_conversion_round_trip ... ok
test spaces::multi_discrete::tests::test_new_invalid_dimension_size ... ok
test spaces::multi_discrete::tests::test_new_invalid_empty ... ok
test spaces::multi_discrete::tests::test_new_valid ... ok
test spaces::multi_discrete::tests::test_overflow_protection ... ok
test spaces::multi_discrete::tests::test_sample ... ok
test spaces::multi_discrete::tests::test_shape_and_bounds ... ok
test spaces::multi_discrete::tests::test_to_u32_errors ... ok
test spaces::multi_discrete::tests::test_total_combinations ... ok
test spaces::multi_discrete::tests::test_u32_to_multi_discrete_errors ... ok
test spaces::space::tests::test_env_space_debug_and_clone ... ok
test spaces::space::tests::test_env_space_struct ... ok
test spaces::space::tests::test_space_trait_implementation ... ok
test spaces::mixed::tests::test_discrete_construction_and_sample ... ok
test env::rapier::bipedal_walker::env::tests::test_episode_truncation_limit ... ok
test env::rapier::lunar_lander::env::tests::test_seeded_reset_is_reproducible ... ok
test env::rapier::bipedal_walker::env::tests::test_action_drives_leg_motors ... ok
test env::rapier::bipedal_walker::env::tests::test_fall_detection_sets_game_over ... ok

test result: ok. 289 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.37s, confirmed via 5 repeated full-suite runs now all passing 289/289). Fixed is_terminal() to compute is_game_over() || is_landed() (matching step()'s terminated flag) instead of only checking out-of-screen.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
LunarLander now uses a per-instance seeded StdRng (reset(Some(seed)) is fully reproducible, verified by a new test running two envs in lockstep) instead of the global thread-local RNG, and is_terminal() now matches step()'s terminated computation (crash/out-of-screen/landed, not just out-of-screen). Also resolved a latent test-suite flakiness caused by the shared global RNG. Verified with 3 new tests plus full suite (289/289 passing across 5 repeated runs).
<!-- SECTION:FINAL_SUMMARY:END -->
