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
pub mod camera_controller;
pub mod color;
pub mod constraint;
pub mod delegates;
pub mod error;
pub mod extended_constraints;
pub mod extended_geometry;
pub mod extended_physics;
pub mod ffi;
pub mod geometry;
pub mod hit_test;
pub mod light;
pub mod material;
pub mod math;
pub mod node;
pub mod particle_system;
pub mod physics;
pub mod physics_world;
pub mod program;
pub mod protocols;
pub mod renderer;
pub mod scene;
pub mod scene_renderer;
pub mod scene_source;
pub mod symbols;
pub mod spritekit;
pub mod technique;
pub mod transaction;
pub mod view;

pub use action::Action;
pub use animation::{Animation, AnimationPlayer};
pub use audio::{AudioPlayer, AudioSource};
pub use camera::Camera;
pub use camera_controller::{
    CameraControlConfiguration, CameraController, CameraControllerDelegate,
    CameraControllerDelegateCallbacks, InteractionMode,
};
pub use color::Color;
pub use constraint::Constraint;
pub use delegates::{
    export_javascript_module, AvoidOccluderConstraintDelegate,
    AvoidOccluderConstraintDelegateCallbacks, NodeRendererDelegate,
    NodeRendererDelegateCallbacks, SceneExportDelegate,
};
pub use error::SceneKitError;
pub use extended_constraints::{
    AccelerationConstraint, AvoidOccluderConstraint, BillboardConstraint, IKConstraint,
    ReplicatorConstraint, SliderConstraint, TransformConstraint,
};
pub use extended_geometry::{
    Capsule, GeometryElement, GeometrySource, GeometryTessellator, LevelOfDetail, Morpher,
    ParticlePropertyController, Pyramid, ReferenceNode, Shape, Skinner, Torus, Tube,
};
pub use extended_physics::{
    PhysicsBallSocketJoint, PhysicsBehavior, PhysicsConeTwistJoint, PhysicsField,
    PhysicsHingeJoint, PhysicsShape, PhysicsSliderJoint, PhysicsVehicle, PhysicsVehicleWheel,
};
pub use geometry::Geometry;
pub use hit_test::{HitTestResult, HitTestResults};
pub use light::{Light, LightType, ShadowMode};
pub use material::{Material, MaterialProperty};
pub use math::{Matrix4, Vector3, Vector4};
pub use node::Node;
pub use particle_system::ParticleSystem;
pub use physics::{PhysicsBody, PhysicsBodyType};
pub use physics_world::{
    PhysicsContact, PhysicsContactDelegate, PhysicsContactDelegateCallbacks, PhysicsWorld,
};
pub use program::{
    BufferFrequency, BufferStream, Program, ProgramBufferBinding, ProgramDelegate, Shadable,
};
pub use protocols::{Actionable, Animatable, AnimationEvent, BoundingVolume, TechniqueSupport, TimingFunction};
pub use renderer::{read_texture_bytes, LoadAction, RenderPassDescriptor, Renderer, StoreAction};
pub use scene::Scene;
pub use scene_renderer::{
    AntialiasingMode, AudioEngine, AudioEnvironmentNode, DebugOptions, MetalCommandQueue,
    MetalDeviceHandle, MetalRenderCommandEncoder, Prepareable, RenderingAPI, SceneRenderer,
    SceneRendererDelegate, SceneRendererDelegateCallbacks,
};
pub use scene_source::{SceneSource, SceneSourceEntryClass, SceneSourceStatus};
pub use spritekit::{SpriteScene, SpriteTransition};
pub use symbols::*;
pub use technique::Technique;
pub use transaction::Transaction;
pub use view::View;

pub use apple_cf::cg::{CGImage, CGPoint, CGRect, CGSize};

pub mod prelude {
    pub use crate::{
        read_texture_bytes, Action, Animation, AnimationPlayer, AntialiasingMode, AudioEngine,
        AudioEnvironmentNode, AudioPlayer, AccelerationConstraint, AudioSource, AvoidOccluderConstraint,
        AvoidOccluderConstraintDelegate, AvoidOccluderConstraintDelegateCallbacks,
        BillboardConstraint, BufferFrequency, BufferStream, Camera,
        CameraControlConfiguration, CameraController, CameraControllerDelegate,
        CameraControllerDelegateCallbacks, Capsule, Color, Constraint, DebugOptions,
        Geometry, GeometryElement, GeometrySource, GeometryTessellator, HitTestResult,
        HitTestResults, IKConstraint, InteractionMode, LevelOfDetail, Light,
        LightType, LoadAction, Material, MaterialProperty, Matrix4, Morpher, Node,
        NodeRendererDelegate, NodeRendererDelegateCallbacks, ParticlePropertyController,
        ParticleSystem, PhysicsBallSocketJoint, PhysicsBehavior, PhysicsBody,
        PhysicsBodyType, PhysicsConeTwistJoint, PhysicsContact, PhysicsContactDelegate,
        PhysicsContactDelegateCallbacks, PhysicsField, PhysicsHingeJoint, PhysicsShape,
        PhysicsSliderJoint, PhysicsVehicle, PhysicsVehicleWheel, PhysicsWorld, Program,
        ProgramBufferBinding, ProgramDelegate, Pyramid, ReferenceNode, RenderPassDescriptor,
        Renderer, RenderingAPI, ReplicatorConstraint, Scene, SceneExportDelegate,
        SceneKitError, SceneRenderer, SceneRendererDelegate, SceneRendererDelegateCallbacks,
        Prepareable,
        SceneSource, SceneSourceEntryClass, SceneSourceStatus, Shadable, ShadowMode,
        Shape, Skinner, SliderConstraint, SpriteScene, SpriteTransition, StoreAction,
        Technique, Torus, Transaction, TransformConstraint, Tube, Vector3, Vector4, View,
    };
    pub use crate::protocols::{
        Actionable, Animatable, AnimationEvent, BoundingVolume, TechniqueSupport, TimingFunction,
    };
    pub use crate::symbols::*;
}
