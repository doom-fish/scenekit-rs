#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

pub type ScnActionCallback = extern "C" fn(*mut c_void, *mut c_void, f64);
pub type ScnDropCallback = extern "C" fn(*mut c_void);

extern "C" {
    pub fn scn_release(handle: *mut c_void);

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
}
