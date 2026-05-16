#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

mod private;

pub mod action;
pub mod animation;
pub mod audio;
pub mod camera;
pub mod color;
pub mod constraint;
pub mod error;
pub mod ffi;
pub mod geometry;
pub mod hit_test;
pub mod light;
pub mod material;
pub mod math;
pub mod node;
pub mod particle_system;
pub mod physics;
pub mod renderer;
pub mod scene;
pub mod technique;
pub mod transaction;
pub mod view;

pub use action::Action;
pub use animation::{Animation, AnimationPlayer};
pub use audio::{AudioPlayer, AudioSource};
pub use camera::Camera;
pub use color::Color;
pub use constraint::Constraint;
pub use error::SceneKitError;
pub use geometry::Geometry;
pub use hit_test::{HitTestResult, HitTestResults};
pub use light::{Light, LightType, ShadowMode};
pub use material::{Material, MaterialProperty};
pub use math::{Matrix4, Vector3, Vector4};
pub use node::Node;
pub use particle_system::ParticleSystem;
pub use physics::{PhysicsBody, PhysicsBodyType};
pub use renderer::{read_texture_bytes, LoadAction, RenderPassDescriptor, Renderer, StoreAction};
pub use scene::Scene;
pub use technique::Technique;
pub use transaction::Transaction;
pub use view::View;

pub use apple_cf::cg::{CGImage, CGPoint, CGRect, CGSize};

pub mod prelude {
    pub use crate::{
        read_texture_bytes, Action, Animation, AnimationPlayer, AudioPlayer, AudioSource, Camera,
        Color, Constraint, Geometry, HitTestResult, HitTestResults, Light, LightType, LoadAction,
        Material, MaterialProperty, Matrix4, Node, ParticleSystem, PhysicsBody, PhysicsBodyType,
        RenderPassDescriptor, Renderer, Scene, SceneKitError, ShadowMode, StoreAction, Technique,
        Transaction, Vector3, Vector4, View,
    };
}
