#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

mod private;

pub mod action;
pub mod camera;
pub mod color;
pub mod error;
pub mod ffi;
pub mod geometry;
pub mod light;
pub mod material;
pub mod math;
pub mod node;
pub mod physics;
pub mod renderer;
pub mod scene;

pub use action::Action;
pub use camera::Camera;
pub use color::Color;
pub use error::SceneKitError;
pub use geometry::Geometry;
pub use light::{Light, LightType, ShadowMode};
pub use material::{Material, MaterialProperty};
pub use math::{Matrix4, Vector3, Vector4};
pub use node::Node;
pub use physics::{PhysicsBody, PhysicsBodyType};
pub use renderer::{read_texture_bytes, LoadAction, RenderPassDescriptor, Renderer, StoreAction};
pub use scene::Scene;

pub use apple_cf::cg::{CGImage, CGPoint, CGRect, CGSize};

pub mod prelude {
    pub use crate::{
        read_texture_bytes, Action, Camera, Color, Geometry, Light, LightType, LoadAction,
        Material, MaterialProperty, Matrix4, Node, PhysicsBody, PhysicsBodyType,
        RenderPassDescriptor, Renderer, Scene, SceneKitError, ShadowMode, StoreAction, Vector3,
        Vector4,
    };
}
