#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

pub type ScnActionCallback = extern "C" fn(*mut c_void, *mut c_void, f64);
pub type ScnDropCallback = extern "C" fn(*mut c_void);
pub type ScnTimeCallback = extern "C" fn(*mut c_void, f64);
pub type ScnSceneCallback = extern "C" fn(*mut c_void, *mut c_void, f64);
pub type ScnVoidCallback = extern "C" fn(*mut c_void);
pub type ScnContactCallback = extern "C" fn(*mut c_void, *mut c_void);
pub type ScnProgramErrorCallback = extern "C" fn(*mut c_void, *mut c_char);
pub type ScnProgramBufferBindingCallback = extern "C" fn(*mut c_void, *mut c_void);

extern "C" {
    pub fn scn_release(handle: *mut c_void);
    pub fn scn_constant_lookup(name: *const c_char) -> *mut c_char;

    pub fn scn_scene_new() -> *mut c_void;
    pub fn scn_scene_new_named(name: *const c_char) -> *mut c_void;
    pub fn scn_scene_new_url(path: *const c_char, out_error: *mut *mut c_char) -> *mut c_void;
    pub fn scn_scene_root_node(scene: *mut c_void) -> *mut c_void;
    pub fn scn_scene_background(scene: *mut c_void) -> *mut c_void;
    pub fn scn_scene_lighting_environment(scene: *mut c_void) -> *mut c_void;
    pub fn scn_scene_set_fog_color(scene: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    pub fn scn_scene_copy_fog_color(scene: *mut c_void, out_rgba: *mut c_void) -> bool;

    pub fn scn_node_new() -> *mut c_void;
    pub fn scn_node_new_with_geometry(geometry: *mut c_void) -> *mut c_void;
    pub fn scn_node_add_child(parent: *mut c_void, child: *mut c_void);
    pub fn scn_node_remove_from_parent(node: *mut c_void);
    pub fn scn_node_copy_name(node: *mut c_void) -> *mut c_char;
    pub fn scn_node_set_name(node: *mut c_void, name: *const c_char);
    pub fn scn_node_get_transform(node: *mut c_void, out_matrix: *mut c_void) -> bool;
    pub fn scn_node_set_transform(node: *mut c_void, matrix: *mut c_void);
    pub fn scn_node_get_position(node: *mut c_void, out_vec3: *mut c_void) -> bool;
    pub fn scn_node_set_position(node: *mut c_void, vec3: *mut c_void);
    pub fn scn_node_get_rotation(node: *mut c_void, out_vec4: *mut c_void) -> bool;
    pub fn scn_node_set_rotation(node: *mut c_void, vec4: *mut c_void);
    pub fn scn_node_get_scale(node: *mut c_void, out_vec3: *mut c_void) -> bool;
    pub fn scn_node_set_scale(node: *mut c_void, vec3: *mut c_void);
    pub fn scn_node_get_euler_angles(node: *mut c_void, out_vec3: *mut c_void) -> bool;
    pub fn scn_node_set_euler_angles(node: *mut c_void, vec3: *mut c_void);
    pub fn scn_node_get_pivot(node: *mut c_void, out_matrix: *mut c_void) -> bool;
    pub fn scn_node_set_pivot(node: *mut c_void, matrix: *mut c_void);
    pub fn scn_node_get_hidden(node: *mut c_void) -> bool;
    pub fn scn_node_set_hidden(node: *mut c_void, hidden: bool);
    pub fn scn_node_get_geometry(node: *mut c_void) -> *mut c_void;
    pub fn scn_node_set_geometry(node: *mut c_void, geometry: *mut c_void);
    pub fn scn_node_get_light(node: *mut c_void) -> *mut c_void;
    pub fn scn_node_set_light(node: *mut c_void, light: *mut c_void);
    pub fn scn_node_get_camera(node: *mut c_void) -> *mut c_void;
    pub fn scn_node_set_camera(node: *mut c_void, camera: *mut c_void);
    pub fn scn_node_get_physics_body(node: *mut c_void) -> *mut c_void;
    pub fn scn_node_set_physics_body(node: *mut c_void, physics_body: *mut c_void);
    pub fn scn_node_run_action(node: *mut c_void, action: *mut c_void);

    pub fn scn_geometry_new_box(
        width: f64,
        height: f64,
        length: f64,
        chamfer_radius: f64,
    ) -> *mut c_void;
    pub fn scn_geometry_new_sphere(radius: f64) -> *mut c_void;
    pub fn scn_geometry_new_cylinder(radius: f64, height: f64) -> *mut c_void;
    pub fn scn_geometry_new_cone(top_radius: f64, bottom_radius: f64, height: f64) -> *mut c_void;
    pub fn scn_geometry_new_plane(width: f64, height: f64) -> *mut c_void;
    pub fn scn_geometry_new_floor() -> *mut c_void;
    pub fn scn_geometry_new_text(string: *const c_char, extrusion_depth: f64) -> *mut c_void;
    pub fn scn_geometry_new_from_mdl_mesh(mesh: *mut c_void) -> *mut c_void;
    pub fn scn_geometry_first_material(geometry: *mut c_void) -> *mut c_void;
    pub fn scn_geometry_set_first_material(geometry: *mut c_void, material: *mut c_void);

    pub fn scn_material_new() -> *mut c_void;
    pub fn scn_material_diffuse(material: *mut c_void) -> *mut c_void;
    pub fn scn_material_normal(material: *mut c_void) -> *mut c_void;
    pub fn scn_material_specular(material: *mut c_void) -> *mut c_void;
    pub fn scn_material_emission(material: *mut c_void) -> *mut c_void;
    pub fn scn_material_ambient(material: *mut c_void) -> *mut c_void;
    pub fn scn_material_transparent(material: *mut c_void) -> *mut c_void;
    pub fn scn_material_multiply(material: *mut c_void) -> *mut c_void;

    pub fn scn_material_property_set_color(property: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    pub fn scn_material_property_copy_color(property: *mut c_void, out_rgba: *mut c_void) -> bool;
    pub fn scn_material_property_set_cg_image(property: *mut c_void, image: *mut c_void);
    pub fn scn_material_property_set_metal_texture(property: *mut c_void, texture: *mut c_void);
    pub fn scn_material_property_set_file_url(property: *mut c_void, path: *const c_char);
    pub fn scn_material_property_clear_contents(property: *mut c_void);
    pub fn scn_material_property_get_intensity(property: *mut c_void) -> f64;
    pub fn scn_material_property_set_intensity(property: *mut c_void, intensity: f64);

    pub fn scn_camera_new() -> *mut c_void;
    pub fn scn_camera_get_field_of_view(camera: *mut c_void) -> f64;
    pub fn scn_camera_set_field_of_view(camera: *mut c_void, field_of_view: f64);
    pub fn scn_camera_get_z_near(camera: *mut c_void) -> f64;
    pub fn scn_camera_set_z_near(camera: *mut c_void, z_near: f64);
    pub fn scn_camera_get_z_far(camera: *mut c_void) -> f64;
    pub fn scn_camera_set_z_far(camera: *mut c_void, z_far: f64);
    pub fn scn_camera_get_projection_transform(
        camera: *mut c_void,
        out_matrix: *mut c_void,
    ) -> bool;
    pub fn scn_camera_set_projection_transform(camera: *mut c_void, matrix: *mut c_void);

    pub fn scn_light_new() -> *mut c_void;
    pub fn scn_light_get_type(light: *mut c_void) -> i32;
    pub fn scn_light_set_type(light: *mut c_void, light_type: i32);
    pub fn scn_light_copy_color(light: *mut c_void, out_rgba: *mut c_void) -> bool;
    pub fn scn_light_set_color(light: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    pub fn scn_light_get_intensity(light: *mut c_void) -> f64;
    pub fn scn_light_set_intensity(light: *mut c_void, intensity: f64);
    pub fn scn_light_get_shadow_mode(light: *mut c_void) -> i32;
    pub fn scn_light_set_shadow_mode(light: *mut c_void, shadow_mode: i32);
    pub fn scn_light_get_casts_shadow(light: *mut c_void) -> bool;
    pub fn scn_light_set_casts_shadow(light: *mut c_void, casts_shadow: bool);

    pub fn scn_action_move_to(x: f32, y: f32, z: f32, duration: f64) -> *mut c_void;
    pub fn scn_action_move_by(x: f32, y: f32, z: f32, duration: f64) -> *mut c_void;
    pub fn scn_action_rotate_by(x: f32, y: f32, z: f32, duration: f64) -> *mut c_void;
    pub fn scn_action_scale_by(scale: f32, duration: f64) -> *mut c_void;
    pub fn scn_action_sequence(actions: *mut c_void, count: usize) -> *mut c_void;
    pub fn scn_action_group(actions: *mut c_void, count: usize) -> *mut c_void;
    pub fn scn_action_repeat(action: *mut c_void, count: usize) -> *mut c_void;
    pub fn scn_action_repeat_forever(action: *mut c_void) -> *mut c_void;
    pub fn scn_action_custom(
        duration: f64,
        context: *mut c_void,
        callback: ScnActionCallback,
        drop_callback: ScnDropCallback,
    ) -> *mut c_void;

    pub fn scn_physics_body_static() -> *mut c_void;
    pub fn scn_physics_body_dynamic() -> *mut c_void;
    pub fn scn_physics_body_kinematic() -> *mut c_void;
    pub fn scn_physics_body_get_type(body: *mut c_void) -> i32;
    pub fn scn_physics_body_set_type(body: *mut c_void, body_type: i32);
    pub fn scn_physics_body_get_mass(body: *mut c_void) -> f64;
    pub fn scn_physics_body_set_mass(body: *mut c_void, mass: f64);
    pub fn scn_physics_body_get_restitution(body: *mut c_void) -> f64;
    pub fn scn_physics_body_set_restitution(body: *mut c_void, restitution: f64);
    pub fn scn_physics_body_get_friction(body: *mut c_void) -> f64;
    pub fn scn_physics_body_set_friction(body: *mut c_void, friction: f64);
    pub fn scn_physics_body_apply_force(body: *mut c_void, x: f32, y: f32, z: f32, impulse: bool);

    pub fn scn_render_pass_descriptor_new_for_texture(
        texture: *mut c_void,
        clear_r: f64,
        clear_g: f64,
        clear_b: f64,
        clear_a: f64,
        load_action: i32,
        store_action: i32,
    ) -> *mut c_void;
    pub fn scn_renderer_new(device: *mut c_void) -> *mut c_void;
    pub fn scn_renderer_set_scene(renderer: *mut c_void, scene: *mut c_void);
    pub fn scn_renderer_set_point_of_view(renderer: *mut c_void, node: *mut c_void);
    pub fn scn_renderer_render(
        renderer: *mut c_void,
        time: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        command_buffer: *mut c_void,
        pass_descriptor: *mut c_void,
    );
    pub fn scn_texture_copy_bytes(
        texture: *mut c_void,
        out_bytes: *mut c_void,
        bytes_per_row: usize,
    ) -> bool;

    pub fn scn_animation_new_opacity(from: f32, to: f32, duration: f64) -> *mut c_void;
    pub fn scn_animation_get_duration(animation: *mut c_void) -> f64;
    pub fn scn_animation_set_duration(animation: *mut c_void, duration: f64);
    pub fn scn_animation_get_repeat_count(animation: *mut c_void) -> f64;
    pub fn scn_animation_set_repeat_count(animation: *mut c_void, repeat_count: f64);
    pub fn scn_animation_get_autoreverses(animation: *mut c_void) -> bool;
    pub fn scn_animation_set_autoreverses(animation: *mut c_void, autoreverses: bool);
    pub fn scn_animation_get_uses_scene_time_base(animation: *mut c_void) -> bool;
    pub fn scn_animation_set_uses_scene_time_base(
        animation: *mut c_void,
        uses_scene_time_base: bool,
    );
    pub fn scn_animation_player_new(animation: *mut c_void) -> *mut c_void;
    pub fn scn_animation_player_animation(player: *mut c_void) -> *mut c_void;
    pub fn scn_animation_player_get_speed(player: *mut c_void) -> f64;
    pub fn scn_animation_player_set_speed(player: *mut c_void, speed: f64);
    pub fn scn_animation_player_get_paused(player: *mut c_void) -> bool;
    pub fn scn_animation_player_set_paused(player: *mut c_void, paused: bool);
    pub fn scn_animation_player_play(player: *mut c_void);
    pub fn scn_animation_player_stop(player: *mut c_void);
    pub fn scn_node_add_animation_player(
        node: *mut c_void,
        player: *mut c_void,
        key: *const c_char,
    );
    pub fn scn_node_animation_player(node: *mut c_void, key: *const c_char) -> *mut c_void;

    pub fn scn_transaction_begin();
    pub fn scn_transaction_commit();
    pub fn scn_transaction_flush();
    pub fn scn_transaction_get_animation_duration() -> f64;
    pub fn scn_transaction_set_animation_duration(animation_duration: f64);
    pub fn scn_transaction_get_disable_actions() -> bool;
    pub fn scn_transaction_set_disable_actions(disable_actions: bool);

    pub fn scn_constraint_new_look_at(target: *mut c_void) -> *mut c_void;
    pub fn scn_constraint_new_distance(target: *mut c_void) -> *mut c_void;
    pub fn scn_constraint_get_influence_factor(constraint: *mut c_void) -> f64;
    pub fn scn_constraint_set_influence_factor(constraint: *mut c_void, influence_factor: f64);
    pub fn scn_constraint_get_gimbal_lock_enabled(constraint: *mut c_void) -> bool;
    pub fn scn_constraint_set_gimbal_lock_enabled(
        constraint: *mut c_void,
        gimbal_lock_enabled: bool,
    );
    pub fn scn_constraint_get_minimum_distance(constraint: *mut c_void) -> f64;
    pub fn scn_constraint_set_minimum_distance(constraint: *mut c_void, minimum_distance: f64);
    pub fn scn_constraint_get_maximum_distance(constraint: *mut c_void) -> f64;
    pub fn scn_constraint_set_maximum_distance(constraint: *mut c_void, maximum_distance: f64);
    pub fn scn_node_set_constraints(node: *mut c_void, constraints: *mut c_void, count: usize);
    pub fn scn_node_constraints_count(node: *mut c_void) -> usize;

    pub fn scn_particle_system_new() -> *mut c_void;
    pub fn scn_particle_system_get_birth_rate(system: *mut c_void) -> f64;
    pub fn scn_particle_system_set_birth_rate(system: *mut c_void, birth_rate: f64);
    pub fn scn_particle_system_get_life_span(system: *mut c_void) -> f64;
    pub fn scn_particle_system_set_life_span(system: *mut c_void, life_span: f64);
    pub fn scn_particle_system_get_loops(system: *mut c_void) -> bool;
    pub fn scn_particle_system_set_loops(system: *mut c_void, loops: bool);
    pub fn scn_node_add_particle_system(node: *mut c_void, system: *mut c_void);
    pub fn scn_node_remove_all_particle_systems(node: *mut c_void);
    pub fn scn_node_particle_system_count(node: *mut c_void) -> usize;

    pub fn scn_audio_source_new_url(path: *const c_char) -> *mut c_void;
    pub fn scn_audio_source_get_volume(source: *mut c_void) -> f32;
    pub fn scn_audio_source_set_volume(source: *mut c_void, volume: f32);
    pub fn scn_audio_source_get_positional(source: *mut c_void) -> bool;
    pub fn scn_audio_source_set_positional(source: *mut c_void, positional: bool);
    pub fn scn_audio_source_get_loops(source: *mut c_void) -> bool;
    pub fn scn_audio_source_set_loops(source: *mut c_void, loops: bool);
    pub fn scn_audio_source_load(source: *mut c_void);
    pub fn scn_audio_player_new(source: *mut c_void) -> *mut c_void;
    pub fn scn_audio_player_source(player: *mut c_void) -> *mut c_void;
    pub fn scn_node_add_audio_player(node: *mut c_void, player: *mut c_void);
    pub fn scn_node_remove_all_audio_players(node: *mut c_void);
    pub fn scn_node_audio_player_count(node: *mut c_void) -> usize;

    pub fn scn_view_new(width: f64, height: f64) -> *mut c_void;
    pub fn scn_view_set_scene(view: *mut c_void, scene: *mut c_void);
    pub fn scn_view_scene(view: *mut c_void) -> *mut c_void;
    pub fn scn_view_set_point_of_view(view: *mut c_void, node: *mut c_void);
    pub fn scn_view_point_of_view(view: *mut c_void) -> *mut c_void;
    pub fn scn_view_get_allows_camera_control(view: *mut c_void) -> bool;
    pub fn scn_view_set_allows_camera_control(view: *mut c_void, allows_camera_control: bool);
    pub fn scn_view_get_renders_continuously(view: *mut c_void) -> bool;
    pub fn scn_view_set_renders_continuously(view: *mut c_void, renders_continuously: bool);
    pub fn scn_view_copy_background_color(view: *mut c_void, out_rgba: *mut c_void) -> bool;
    pub fn scn_view_set_background_color(view: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    pub fn scn_view_snapshot_size(view: *mut c_void, out_size: *mut f64) -> bool;
    pub fn scn_view_get_preferred_frames_per_second(view: *mut c_void) -> isize;
    pub fn scn_view_set_preferred_frames_per_second(
        view: *mut c_void,
        preferred_frames_per_second: isize,
    );

    pub fn scn_view_hit_test(view: *mut c_void, x: f64, y: f64) -> *mut c_void;
    pub fn scn_hit_test_results_count(results: *mut c_void) -> usize;
    pub fn scn_hit_test_results_get(results: *mut c_void, index: usize) -> *mut c_void;
    pub fn scn_hit_test_result_node(result: *mut c_void) -> *mut c_void;
    pub fn scn_hit_test_result_world_coordinates(
        result: *mut c_void,
        out_vec3: *mut c_void,
    ) -> bool;

    pub fn scn_technique_new_minimal_draw_scene() -> *mut c_void;
    pub fn scn_technique_dictionary_key_count(technique: *mut c_void) -> usize;
    pub fn scn_technique_set_float_symbol(technique: *mut c_void, key: *const c_char, value: f64);
    pub fn scn_technique_get_float_symbol(
        technique: *mut c_void,
        key: *const c_char,
        out_value: *mut f64,
    ) -> bool;
    pub fn scn_view_set_technique(view: *mut c_void, technique: *mut c_void);
    pub fn scn_view_technique(view: *mut c_void) -> *mut c_void;

    pub fn scn_scene_renderer_delegate_new(
        context: *mut c_void,
        release_context: ScnDropCallback,
        update: ScnTimeCallback,
        did_apply_animations: ScnTimeCallback,
        did_simulate_physics: ScnTimeCallback,
        did_apply_constraints: ScnTimeCallback,
        will_render_scene: ScnSceneCallback,
        did_render_scene: ScnSceneCallback,
    ) -> *mut c_void;
    pub fn scn_scene_renderer_get_scene(renderer: *mut c_void) -> *mut c_void;
    pub fn scn_scene_renderer_set_scene(renderer: *mut c_void, scene: *mut c_void);
    pub fn scn_scene_renderer_get_scene_time(renderer: *mut c_void) -> f64;
    pub fn scn_scene_renderer_set_scene_time(renderer: *mut c_void, scene_time: f64);
    pub fn scn_scene_renderer_get_point_of_view(renderer: *mut c_void) -> *mut c_void;
    pub fn scn_scene_renderer_set_point_of_view(renderer: *mut c_void, point_of_view: *mut c_void);
    pub fn scn_scene_renderer_get_playing(renderer: *mut c_void) -> bool;
    pub fn scn_scene_renderer_set_playing(renderer: *mut c_void, playing: bool);
    pub fn scn_scene_renderer_get_loops(renderer: *mut c_void) -> bool;
    pub fn scn_scene_renderer_set_loops(renderer: *mut c_void, loops: bool);
    pub fn scn_scene_renderer_get_autoenables_default_lighting(renderer: *mut c_void) -> bool;
    pub fn scn_scene_renderer_set_autoenables_default_lighting(
        renderer: *mut c_void,
        autoenables_default_lighting: bool,
    );
    pub fn scn_scene_renderer_get_jittering_enabled(renderer: *mut c_void) -> bool;
    pub fn scn_scene_renderer_set_jittering_enabled(renderer: *mut c_void, jittering_enabled: bool);
    pub fn scn_scene_renderer_get_temporal_antialiasing_enabled(renderer: *mut c_void) -> bool;
    pub fn scn_scene_renderer_set_temporal_antialiasing_enabled(
        renderer: *mut c_void,
        temporal_antialiasing_enabled: bool,
    );
    pub fn scn_scene_renderer_get_shows_statistics(renderer: *mut c_void) -> bool;
    pub fn scn_scene_renderer_set_shows_statistics(renderer: *mut c_void, shows_statistics: bool);
    pub fn scn_scene_renderer_get_debug_options(renderer: *mut c_void) -> usize;
    pub fn scn_scene_renderer_set_debug_options(renderer: *mut c_void, debug_options: usize);
    pub fn scn_scene_renderer_get_rendering_api(renderer: *mut c_void) -> i32;
    pub fn scn_scene_renderer_set_delegate(renderer: *mut c_void, delegate: *mut c_void);
    pub fn scn_scene_renderer_test_invoke_delegate_update(renderer: *mut c_void, time: f64);
    pub fn scn_scene_renderer_test_invoke_delegate_will_render_scene(
        renderer: *mut c_void,
        time: f64,
    );
    pub fn scn_scene_renderer_test_invoke_delegate_did_render_scene(
        renderer: *mut c_void,
        time: f64,
    );
    pub fn scn_view_get_antialiasing_mode(view: *mut c_void) -> i32;
    pub fn scn_view_set_antialiasing_mode(view: *mut c_void, antialiasing_mode: i32);

    pub fn scn_camera_controller_delegate_new(
        context: *mut c_void,
        release_context: ScnDropCallback,
        inertia_will_start: ScnVoidCallback,
        inertia_did_end: ScnVoidCallback,
    ) -> *mut c_void;
    pub fn scn_view_camera_control_configuration(view: *mut c_void) -> *mut c_void;
    pub fn scn_view_default_camera_controller(view: *mut c_void) -> *mut c_void;
    pub fn scn_camera_control_configuration_get_auto_switch_to_free_camera(
        configuration: *mut c_void,
    ) -> bool;
    pub fn scn_camera_control_configuration_set_auto_switch_to_free_camera(
        configuration: *mut c_void,
        auto_switch_to_free_camera: bool,
    );
    pub fn scn_camera_control_configuration_get_allows_translation(
        configuration: *mut c_void,
    ) -> bool;
    pub fn scn_camera_control_configuration_set_allows_translation(
        configuration: *mut c_void,
        allows_translation: bool,
    );
    pub fn scn_camera_control_configuration_get_fly_mode_velocity(
        configuration: *mut c_void,
    ) -> f64;
    pub fn scn_camera_control_configuration_set_fly_mode_velocity(
        configuration: *mut c_void,
        fly_mode_velocity: f64,
    );
    pub fn scn_camera_control_configuration_get_pan_sensitivity(configuration: *mut c_void) -> f64;
    pub fn scn_camera_control_configuration_set_pan_sensitivity(
        configuration: *mut c_void,
        pan_sensitivity: f64,
    );
    pub fn scn_camera_control_configuration_get_truck_sensitivity(
        configuration: *mut c_void,
    ) -> f64;
    pub fn scn_camera_control_configuration_set_truck_sensitivity(
        configuration: *mut c_void,
        truck_sensitivity: f64,
    );
    pub fn scn_camera_control_configuration_get_rotation_sensitivity(
        configuration: *mut c_void,
    ) -> f64;
    pub fn scn_camera_control_configuration_set_rotation_sensitivity(
        configuration: *mut c_void,
        rotation_sensitivity: f64,
    );
    pub fn scn_camera_controller_set_delegate(controller: *mut c_void, delegate: *mut c_void);
    pub fn scn_camera_controller_get_point_of_view(controller: *mut c_void) -> *mut c_void;
    pub fn scn_camera_controller_set_point_of_view(
        controller: *mut c_void,
        point_of_view: *mut c_void,
    );
    pub fn scn_camera_controller_get_interaction_mode(controller: *mut c_void) -> i32;
    pub fn scn_camera_controller_set_interaction_mode(
        controller: *mut c_void,
        interaction_mode: i32,
    );
    pub fn scn_camera_controller_get_target(
        controller: *mut c_void,
        out_vector: *mut c_void,
    ) -> bool;
    pub fn scn_camera_controller_set_target(controller: *mut c_void, target: *mut c_void);
    pub fn scn_camera_controller_get_automatic_target(controller: *mut c_void) -> bool;
    pub fn scn_camera_controller_set_automatic_target(
        controller: *mut c_void,
        automatic_target: bool,
    );
    pub fn scn_camera_controller_get_world_up(
        controller: *mut c_void,
        out_vector: *mut c_void,
    ) -> bool;
    pub fn scn_camera_controller_set_world_up(controller: *mut c_void, world_up: *mut c_void);
    pub fn scn_camera_controller_get_inertia_enabled(controller: *mut c_void) -> bool;
    pub fn scn_camera_controller_set_inertia_enabled(
        controller: *mut c_void,
        inertia_enabled: bool,
    );
    pub fn scn_camera_controller_get_inertia_friction(controller: *mut c_void) -> f32;
    pub fn scn_camera_controller_set_inertia_friction(
        controller: *mut c_void,
        inertia_friction: f32,
    );
    pub fn scn_camera_controller_get_inertia_running(controller: *mut c_void) -> bool;
    pub fn scn_camera_controller_get_minimum_vertical_angle(controller: *mut c_void) -> f32;
    pub fn scn_camera_controller_set_minimum_vertical_angle(
        controller: *mut c_void,
        minimum_vertical_angle: f32,
    );
    pub fn scn_camera_controller_get_maximum_vertical_angle(controller: *mut c_void) -> f32;
    pub fn scn_camera_controller_set_maximum_vertical_angle(
        controller: *mut c_void,
        maximum_vertical_angle: f32,
    );
    pub fn scn_camera_controller_get_minimum_horizontal_angle(controller: *mut c_void) -> f32;
    pub fn scn_camera_controller_set_minimum_horizontal_angle(
        controller: *mut c_void,
        minimum_horizontal_angle: f32,
    );
    pub fn scn_camera_controller_get_maximum_horizontal_angle(controller: *mut c_void) -> f32;
    pub fn scn_camera_controller_set_maximum_horizontal_angle(
        controller: *mut c_void,
        maximum_horizontal_angle: f32,
    );
    pub fn scn_camera_controller_translate_in_camera_space(
        controller: *mut c_void,
        delta_x: f32,
        delta_y: f32,
        delta_z: f32,
    );
    pub fn scn_camera_controller_frame_nodes(
        controller: *mut c_void,
        nodes: *mut c_void,
        count: usize,
    );
    pub fn scn_camera_controller_rotate_by(controller: *mut c_void, delta_x: f32, delta_y: f32);
    pub fn scn_camera_controller_roll_by(
        controller: *mut c_void,
        delta: f32,
        point_x: f64,
        point_y: f64,
        viewport_width: f64,
        viewport_height: f64,
    );
    pub fn scn_camera_controller_dolly_by(
        controller: *mut c_void,
        delta: f32,
        point_x: f64,
        point_y: f64,
        viewport_width: f64,
        viewport_height: f64,
    );
    pub fn scn_camera_controller_roll_around_target(controller: *mut c_void, delta: f32);
    pub fn scn_camera_controller_dolly_to_target(controller: *mut c_void, delta: f32);
    pub fn scn_camera_controller_clear_roll(controller: *mut c_void);
    pub fn scn_camera_controller_stop_inertia(controller: *mut c_void);
    pub fn scn_camera_controller_begin_interaction(
        controller: *mut c_void,
        location_x: f64,
        location_y: f64,
        viewport_width: f64,
        viewport_height: f64,
    );
    pub fn scn_camera_controller_continue_interaction(
        controller: *mut c_void,
        location_x: f64,
        location_y: f64,
        viewport_width: f64,
        viewport_height: f64,
        sensitivity: f64,
    );
    pub fn scn_camera_controller_end_interaction(
        controller: *mut c_void,
        location_x: f64,
        location_y: f64,
        viewport_width: f64,
        viewport_height: f64,
        velocity_x: f64,
        velocity_y: f64,
    );
    pub fn scn_camera_controller_test_invoke_delegate_inertia_will_start(controller: *mut c_void);
    pub fn scn_camera_controller_test_invoke_delegate_inertia_did_end(controller: *mut c_void);

    pub fn scn_scene_source_new_url(
        path: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn scn_scene_source_new_data(
        bytes: *const c_void,
        length: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn scn_scene_source_copy_url(scene_source: *mut c_void) -> *mut c_char;
    pub fn scn_scene_source_new_scene(
        scene_source: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn scn_scene_source_copy_property_for_key(
        scene_source: *mut c_void,
        key: *const c_char,
    ) -> *mut c_char;
    pub fn scn_scene_source_copy_identifiers_of_entries(
        scene_source: *mut c_void,
        entry_class: i32,
    ) -> *mut c_char;

    pub fn scn_program_delegate_new(
        context: *mut c_void,
        release_context: ScnDropCallback,
        handle_error: ScnProgramErrorCallback,
    ) -> *mut c_void;
    pub fn scn_program_buffer_binding_new(
        context: *mut c_void,
        release_context: ScnDropCallback,
        callback: ScnProgramBufferBindingCallback,
    ) -> *mut c_void;
    pub fn scn_program_new() -> *mut c_void;
    pub fn scn_program_copy_vertex_shader(program: *mut c_void) -> *mut c_char;
    pub fn scn_program_set_vertex_shader(program: *mut c_void, vertex_shader: *const c_char);
    pub fn scn_program_copy_fragment_shader(program: *mut c_void) -> *mut c_char;
    pub fn scn_program_set_fragment_shader(program: *mut c_void, fragment_shader: *const c_char);
    pub fn scn_program_copy_geometry_shader(program: *mut c_void) -> *mut c_char;
    pub fn scn_program_set_geometry_shader(program: *mut c_void, geometry_shader: *const c_char);
    pub fn scn_program_copy_tessellation_control_shader(program: *mut c_void) -> *mut c_char;
    pub fn scn_program_set_tessellation_control_shader(
        program: *mut c_void,
        tessellation_control_shader: *const c_char,
    );
    pub fn scn_program_copy_tessellation_evaluation_shader(program: *mut c_void) -> *mut c_char;
    pub fn scn_program_set_tessellation_evaluation_shader(
        program: *mut c_void,
        tessellation_evaluation_shader: *const c_char,
    );
    pub fn scn_program_copy_vertex_function_name(program: *mut c_void) -> *mut c_char;
    pub fn scn_program_set_vertex_function_name(
        program: *mut c_void,
        vertex_function_name: *const c_char,
    );
    pub fn scn_program_copy_fragment_function_name(program: *mut c_void) -> *mut c_char;
    pub fn scn_program_set_fragment_function_name(
        program: *mut c_void,
        fragment_function_name: *const c_char,
    );
    pub fn scn_program_get_opaque(program: *mut c_void) -> bool;
    pub fn scn_program_set_opaque(program: *mut c_void, opaque: bool);
    pub fn scn_program_set_semantic(
        program: *mut c_void,
        semantic: *const c_char,
        symbol: *const c_char,
        mapping_channel: isize,
        has_mapping_channel: bool,
    );
    pub fn scn_program_copy_semantic_for_symbol(
        program: *mut c_void,
        symbol: *const c_char,
    ) -> *mut c_char;
    pub fn scn_program_set_delegate(program: *mut c_void, delegate: *mut c_void);
    pub fn scn_program_set_buffer_binding(
        program: *mut c_void,
        name: *const c_char,
        frequency: i32,
        binding: *mut c_void,
    );
    pub fn scn_material_get_program(material: *mut c_void) -> *mut c_void;
    pub fn scn_material_set_program(material: *mut c_void, program: *mut c_void);
    pub fn scn_material_copy_shader_modifier(
        material: *mut c_void,
        entry_point: *const c_char,
    ) -> *mut c_char;
    pub fn scn_material_set_shader_modifier(
        material: *mut c_void,
        entry_point: *const c_char,
        shader_modifier: *const c_char,
    );
    pub fn scn_geometry_get_program(geometry: *mut c_void) -> *mut c_void;
    pub fn scn_geometry_set_program(geometry: *mut c_void, program: *mut c_void);
    pub fn scn_geometry_copy_shader_modifier(
        geometry: *mut c_void,
        entry_point: *const c_char,
    ) -> *mut c_char;
    pub fn scn_geometry_set_shader_modifier(
        geometry: *mut c_void,
        entry_point: *const c_char,
        shader_modifier: *const c_char,
    );
    pub fn scn_buffer_stream_write_bytes(
        buffer_stream: *mut c_void,
        bytes: *const c_void,
        length: usize,
    );
    pub fn scn_program_test_invoke_delegate_handle_error(
        program: *mut c_void,
        message: *const c_char,
    );
    pub fn scn_program_test_invoke_buffer_binding(
        program: *mut c_void,
        name: *const c_char,
    ) -> isize;

    pub fn scn_scene_physics_world(scene: *mut c_void) -> *mut c_void;
    pub fn scn_physics_contact_delegate_new(
        context: *mut c_void,
        release_context: ScnDropCallback,
        did_begin_contact: ScnContactCallback,
        did_update_contact: ScnContactCallback,
        did_end_contact: ScnContactCallback,
    ) -> *mut c_void;
    pub fn scn_physics_world_get_gravity(world: *mut c_void, out_vector: *mut c_void) -> bool;
    pub fn scn_physics_world_set_gravity(world: *mut c_void, gravity: *mut c_void);
    pub fn scn_physics_world_get_speed(world: *mut c_void) -> f64;
    pub fn scn_physics_world_set_speed(world: *mut c_void, speed: f64);
    pub fn scn_physics_world_get_time_step(world: *mut c_void) -> f64;
    pub fn scn_physics_world_set_time_step(world: *mut c_void, time_step: f64);
    pub fn scn_physics_world_set_contact_delegate(world: *mut c_void, delegate: *mut c_void);
    pub fn scn_physics_world_update_collision_pairs(world: *mut c_void);
    pub fn scn_physics_world_contact_test_with_body_count(
        world: *mut c_void,
        body: *mut c_void,
    ) -> usize;
    pub fn scn_physics_world_contact_test_between_bodies_count(
        world: *mut c_void,
        body_a: *mut c_void,
        body_b: *mut c_void,
    ) -> usize;
    pub fn scn_physics_contact_get_node_a(contact: *mut c_void) -> *mut c_void;
    pub fn scn_physics_contact_get_node_b(contact: *mut c_void) -> *mut c_void;
    pub fn scn_physics_contact_get_contact_point(
        contact: *mut c_void,
        out_vector: *mut c_void,
    ) -> bool;
    pub fn scn_physics_contact_get_contact_normal(
        contact: *mut c_void,
        out_vector: *mut c_void,
    ) -> bool;
    pub fn scn_physics_contact_get_collision_impulse(contact: *mut c_void) -> f64;
    pub fn scn_physics_contact_get_penetration_distance(contact: *mut c_void) -> f64;
    pub fn scn_physics_contact_get_sweep_test_fraction(contact: *mut c_void) -> f64;
    pub fn scn_physics_world_test_invoke_delegate_did_begin(world: *mut c_void);
    pub fn scn_physics_world_test_invoke_delegate_did_update(world: *mut c_void);
    pub fn scn_physics_world_test_invoke_delegate_did_end(world: *mut c_void);
}
