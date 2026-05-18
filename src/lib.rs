#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]

mod private;

/// Action bindings around `SCNAction`.
pub mod action;
/// Animation bindings around `SCNAnimation` and `SCNAnimationPlayer`.
pub mod animation;
/// Audio bindings around `SCNAudioSource` and `SCNAudioPlayer`.
pub mod audio;
/// Camera bindings around `SCNCamera`.
pub mod camera;
/// Camera-controller bindings around `SCNCameraController`.
pub mod camera_controller;
/// Color helpers used by SceneKit APIs.
pub mod color;
/// Constraint bindings around `SCNConstraint`.
pub mod constraint;
/// Delegate bridges for SceneKit callback protocols.
pub mod delegates;
/// Error helpers for SceneKit operations.
pub mod error;
/// Extended constraint bindings for SceneKit constraint subclasses.
pub mod extended_constraints;
/// Extended geometry bindings for SceneKit geometry helpers.
pub mod extended_geometry;
/// Extended physics bindings for SceneKit physics helpers.
pub mod extended_physics;
pub mod ffi;
/// Geometry bindings around `SCNGeometry`.
pub mod geometry;
/// Hit-test bindings around `SCNHitTestResult`.
pub mod hit_test;
/// Light bindings around `SCNLight`.
pub mod light;
/// Material bindings around `SCNMaterial` and `SCNMaterialProperty`.
pub mod material;
/// Math helpers matching SceneKit vector and matrix layouts.
pub mod math;
/// Node bindings around `SCNNode`.
pub mod node;
/// Particle-system bindings around `SCNParticleSystem`.
pub mod particle_system;
/// Physics-body bindings around `SCNPhysicsBody`.
pub mod physics;
/// Physics-world bindings around `SCNPhysicsWorld` and `SCNPhysicsContact`.
pub mod physics_world;
/// Shader-program bindings around `SCNProgram` and `SCNShadable`.
pub mod program;
/// Protocol bridges mirroring SceneKit Objective-C protocols.
pub mod protocols;
/// Renderer bindings around `SCNRenderer`.
pub mod renderer;
/// Scene bindings around `SCNScene`.
pub mod scene;
/// Renderer-protocol bindings around `SCNSceneRenderer`.
pub mod scene_renderer;
/// Scene-source bindings around `SCNSceneSource`.
pub mod scene_source;
/// SpriteKit overlay helpers used by SceneKit renderers.
pub mod spritekit;
/// SceneKit constants, enums, and math helpers.
pub mod symbols;
/// Technique bindings around `SCNTechnique`.
pub mod technique;
/// Transaction bindings around `SCNTransaction`.
pub mod transaction;
/// View bindings around `SCNView`.
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
    AvoidOccluderConstraintDelegateCallbacks, NodeRendererDelegate, NodeRendererDelegateCallbacks,
    SceneExportDelegate,
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
pub use protocols::{
    Actionable, Animatable, AnimationEvent, BoundingVolume, TechniqueSupport, TimingFunction,
};
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

/// Re-exports the primary SceneKit wrapper types, constants, and traits.
pub mod prelude {
    pub use crate::protocols::{
        Actionable, Animatable, AnimationEvent, BoundingVolume, TechniqueSupport, TimingFunction,
    };
    pub use crate::symbols::*;
    pub use crate::{
        read_texture_bytes, AccelerationConstraint, Action, Animation, AnimationPlayer,
        AntialiasingMode, AudioEngine, AudioEnvironmentNode, AudioPlayer, AudioSource,
        AvoidOccluderConstraint, AvoidOccluderConstraintDelegate,
        AvoidOccluderConstraintDelegateCallbacks, BillboardConstraint, BufferFrequency,
        BufferStream, Camera, CameraControlConfiguration, CameraController,
        CameraControllerDelegate, CameraControllerDelegateCallbacks, Capsule, Color, Constraint,
        DebugOptions, Geometry, GeometryElement, GeometrySource, GeometryTessellator,
        HitTestResult, HitTestResults, IKConstraint, InteractionMode, LevelOfDetail, Light,
        LightType, LoadAction, Material, MaterialProperty, Matrix4, Morpher, Node,
        NodeRendererDelegate, NodeRendererDelegateCallbacks, ParticlePropertyController,
        ParticleSystem, PhysicsBallSocketJoint, PhysicsBehavior, PhysicsBody, PhysicsBodyType,
        PhysicsConeTwistJoint, PhysicsContact, PhysicsContactDelegate,
        PhysicsContactDelegateCallbacks, PhysicsField, PhysicsHingeJoint, PhysicsShape,
        PhysicsSliderJoint, PhysicsVehicle, PhysicsVehicleWheel, PhysicsWorld, Prepareable,
        Program, ProgramBufferBinding, ProgramDelegate, Pyramid, ReferenceNode,
        RenderPassDescriptor, Renderer, RenderingAPI, ReplicatorConstraint, Scene,
        SceneExportDelegate, SceneKitError, SceneRenderer, SceneRendererDelegate,
        SceneRendererDelegateCallbacks, SceneSource, SceneSourceEntryClass, SceneSourceStatus,
        Shadable, ShadowMode, Shape, Skinner, SliderConstraint, SpriteScene, SpriteTransition,
        StoreAction, Technique, Torus, Transaction, TransformConstraint, Tube, Vector3, Vector4,
        View,
    };
}
