use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::{CStr, CString};
use std::path::Path;

use apple_cf::cg::CGImage;

use crate::error::{take_error, take_string, SceneKitError};
use crate::extended_constraints::AvoidOccluderConstraint;
use crate::node::Node;
use crate::private::{cstring_from_path, invoke_callback, CallbackState, DelegateObject};
use crate::renderer::Renderer;
use crate::scene::Scene;

type ReleaseContext = unsafe extern "C" fn(*mut c_void);
type NodePairCallback = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void);
type NodePairPredicate = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> bool;
type WriteImageCallback =
    unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_char, *const c_char) -> *mut c_char;

extern "C" {
    fn scn_node_renderer_delegate_new(
        context: *mut c_void,
        release_context: ReleaseContext,
        render_callback: NodePairCallback,
    ) -> *mut c_void;
    fn scn_node_get_renderer_delegate(node: *mut c_void) -> *mut c_void;
    fn scn_node_set_renderer_delegate(node: *mut c_void, delegate: *mut c_void);

    fn scn_avoid_occluder_constraint_delegate_new(
        context: *mut c_void,
        release_context: ReleaseContext,
        should_avoid_callback: NodePairPredicate,
        did_avoid_callback: NodePairCallback,
    ) -> *mut c_void;
    fn scn_avoid_occluder_constraint_get_delegate(constraint: *mut c_void) -> *mut c_void;
    fn scn_avoid_occluder_constraint_set_delegate(constraint: *mut c_void, delegate: *mut c_void);

    fn scn_scene_export_delegate_new(
        context: *mut c_void,
        release_context: ReleaseContext,
        write_image_callback: WriteImageCallback,
    ) -> *mut c_void;
    fn scn_scene_write_to_url(
        scene: *mut c_void,
        path: *const c_char,
        delegate: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;

    fn scn_export_javascript_module(context: *mut c_void);
}

type NodeRendererCallback = Box<dyn FnMut(&Node, &Renderer) + Send>;
type AvoidOccluderShouldCallback = Box<dyn FnMut(&Node, &Node) -> bool + Send>;
type AvoidOccluderDidCallback = Box<dyn FnMut(&Node, &Node) + Send>;
type SceneExportWriteImageCallback =
    Box<dyn FnMut(&CGImage, &str, Option<&str>) -> Option<String> + Send>;

/// Stores Rust callbacks backing `SCNNodeRendererDelegate`.
#[derive(Default)]
pub struct NodeRendererDelegateCallbacks {
    render: Option<NodeRendererCallback>,
}

impl NodeRendererDelegateCallbacks {
    /// Creates a wrapped `SCNNodeRendererDelegate` instance.
    #[must_use]
    pub const fn new() -> Self {
        Self { render: None }
    }

    /// Mirrors `SCNNodeRendererDelegate.onRender`.
    #[must_use]
    pub fn on_render<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&Node, &Renderer) + Send + 'static,
    {
        self.render = Some(Box::new(callback));
        self
    }
}

/// Wraps `SCNNodeRendererDelegate`.
#[derive(Debug)]
pub struct NodeRendererDelegate {
    inner: DelegateObject<NodeRendererDelegateCallbacks>,
}

unsafe extern "C" fn node_renderer_trampoline(
    context: *mut c_void,
    node: *mut c_void,
    renderer: *mut c_void,
) {
    if node.is_null() || renderer.is_null() {
        return;
    }
    let node = unsafe { Node::from_raw_borrowed(node) };
    let renderer = unsafe { Renderer::from_raw_borrowed(renderer) };
    unsafe {
        invoke_callback::<NodeRendererDelegateCallbacks, _>(
            context,
            "scenekit::NodeRendererDelegate::render",
            |callbacks| {
                if let Some(render) = callbacks.render.as_mut() {
                    render(&node, &renderer);
                }
            },
        );
    }
}

impl NodeRendererDelegate {
    /// Creates a wrapped `SCNNodeRendererDelegate` instance.
    #[must_use]
    pub fn new(callbacks: NodeRendererDelegateCallbacks) -> Option<Self> {
        DelegateObject::new(callbacks, |context| unsafe {
            scn_node_renderer_delegate_new(
                context,
                CallbackState::<NodeRendererDelegateCallbacks>::RELEASE,
                node_renderer_trampoline,
            )
        })
        .map(|inner| Self { inner })
    }

    /// Returns the Objective-C pointer backing this `SCNNodeRendererDelegate` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.inner.as_ptr()
    }
}

/// Stores Rust callbacks backing `SCNAvoidOccluderConstraintDelegate`.
#[derive(Default)]
pub struct AvoidOccluderConstraintDelegateCallbacks {
    should_avoid_occluder: Option<AvoidOccluderShouldCallback>,
    did_avoid_occluder: Option<AvoidOccluderDidCallback>,
}

impl AvoidOccluderConstraintDelegateCallbacks {
    /// Creates a wrapped `SCNAvoidOccluderConstraintDelegate` instance.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            should_avoid_occluder: None,
            did_avoid_occluder: None,
        }
    }

    /// Mirrors `SCNAvoidOccluderConstraintDelegate.onShouldAvoidOccluder`.
    #[must_use]
    pub fn on_should_avoid_occluder<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&Node, &Node) -> bool + Send + 'static,
    {
        self.should_avoid_occluder = Some(Box::new(callback));
        self
    }

    /// Mirrors `SCNAvoidOccluderConstraintDelegate.onDidAvoidOccluder`.
    #[must_use]
    pub fn on_did_avoid_occluder<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&Node, &Node) + Send + 'static,
    {
        self.did_avoid_occluder = Some(Box::new(callback));
        self
    }
}

/// Wraps `SCNAvoidOccluderConstraintDelegate`.
#[derive(Debug)]
pub struct AvoidOccluderConstraintDelegate {
    inner: DelegateObject<AvoidOccluderConstraintDelegateCallbacks>,
}

unsafe extern "C" fn avoid_occluder_should_trampoline(
    context: *mut c_void,
    occluder: *mut c_void,
    node: *mut c_void,
) -> bool {
    if occluder.is_null() || node.is_null() {
        return true;
    }
    let occluder = unsafe { Node::from_raw_borrowed(occluder) };
    let node = unsafe { Node::from_raw_borrowed(node) };
    unsafe {
        invoke_callback::<AvoidOccluderConstraintDelegateCallbacks, _>(
            context,
            "scenekit::AvoidOccluderConstraintDelegate::should_avoid_occluder",
            |callbacks| {
                callbacks
                    .should_avoid_occluder
                    .as_mut()
                    .map(|callback| callback(&occluder, &node))
            },
        )
    }
    .flatten()
    .unwrap_or(true)
}

unsafe extern "C" fn avoid_occluder_did_trampoline(
    context: *mut c_void,
    occluder: *mut c_void,
    node: *mut c_void,
) {
    if occluder.is_null() || node.is_null() {
        return;
    }
    let occluder = unsafe { Node::from_raw_borrowed(occluder) };
    let node = unsafe { Node::from_raw_borrowed(node) };
    unsafe {
        invoke_callback::<AvoidOccluderConstraintDelegateCallbacks, _>(
            context,
            "scenekit::AvoidOccluderConstraintDelegate::did_avoid_occluder",
            |callbacks| {
                if let Some(callback) = callbacks.did_avoid_occluder.as_mut() {
                    callback(&occluder, &node);
                }
            },
        );
    }
}

impl AvoidOccluderConstraintDelegate {
    /// Creates a wrapped `SCNAvoidOccluderConstraintDelegate` instance.
    #[must_use]
    pub fn new(callbacks: AvoidOccluderConstraintDelegateCallbacks) -> Option<Self> {
        DelegateObject::new(callbacks, |context| unsafe {
            scn_avoid_occluder_constraint_delegate_new(
                context,
                CallbackState::<AvoidOccluderConstraintDelegateCallbacks>::RELEASE,
                avoid_occluder_should_trampoline,
                avoid_occluder_did_trampoline,
            )
        })
        .map(|inner| Self { inner })
    }

    /// Returns the Objective-C pointer backing this `SCNAvoidOccluderConstraintDelegate` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.inner.as_ptr()
    }
}

/// Wraps `SCNSceneExportDelegate`.
#[derive(Debug)]
pub struct SceneExportDelegate {
    inner: DelegateObject<SceneExportWriteImageCallback>,
}

unsafe extern "C" fn scene_export_write_image_trampoline(
    context: *mut c_void,
    image: *mut c_void,
    document_url: *const c_char,
    original_image_url: *const c_char,
) -> *mut c_char {
    if image.is_null() {
        return ptr::null_mut();
    }
    let image = unsafe { CGImage::from_raw(image) };
    if document_url.is_null() {
        return ptr::null_mut();
    }
    let document_url = unsafe { CStr::from_ptr(document_url) }
        .to_string_lossy()
        .into_owned();
    let original_image_url = (!original_image_url.is_null()).then(|| {
        unsafe { CStr::from_ptr(original_image_url) }
            .to_string_lossy()
            .into_owned()
    });
    unsafe {
        invoke_callback::<SceneExportWriteImageCallback, _>(
            context,
            "scenekit::SceneExportDelegate::write_image",
            |callback| callback(&image, &document_url, original_image_url.as_deref()),
        )
    }
    .flatten()
    .and_then(|path| CString::new(path).ok())
    .map_or(ptr::null_mut(), |path| unsafe {
        libc::strdup(path.as_ptr())
    })
}

impl SceneExportDelegate {
    /// Creates a wrapped `SCNSceneExportDelegate` instance.
    #[must_use]
    pub fn new<F>(callback: F) -> Option<Self>
    where
        F: FnMut(&CGImage, &str, Option<&str>) -> Option<String> + Send + 'static,
    {
        let callback: SceneExportWriteImageCallback = Box::new(callback);
        DelegateObject::new(callback, |context| unsafe {
            scn_scene_export_delegate_new(
                context,
                CallbackState::<SceneExportWriteImageCallback>::RELEASE,
                scene_export_write_image_trampoline,
            )
        })
        .map(|inner| Self { inner })
    }

    /// Returns the Objective-C pointer backing this `SCNSceneExportDelegate` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.inner.as_ptr()
    }
}

impl Node {
    /// Sets the `SCNNode.rendererDelegate` member.
    pub fn set_renderer_delegate(&self, delegate: Option<&NodeRendererDelegate>) {
        unsafe {
            scn_node_set_renderer_delegate(
                self.as_ptr(),
                delegate.map_or(ptr::null_mut(), NodeRendererDelegate::as_ptr),
            );
        };
    }

    /// Mirrors `SCNNode.rendererDelegate`.
    #[must_use]
    pub fn renderer_delegate(&self) -> Option<NodeRendererDelegate> {
        DelegateObject::from_retained(unsafe { scn_node_get_renderer_delegate(self.as_ptr()) })
            .map(|inner| NodeRendererDelegate { inner })
    }
}

impl AvoidOccluderConstraint {
    /// Sets the `SCNAvoidOccluderConstraint.delegate` member.
    pub fn set_delegate(&self, delegate: Option<&AvoidOccluderConstraintDelegate>) {
        unsafe {
            scn_avoid_occluder_constraint_set_delegate(
                self.as_ptr(),
                delegate.map_or(ptr::null_mut(), AvoidOccluderConstraintDelegate::as_ptr),
            );
        };
    }

    /// Mirrors `SCNAvoidOccluderConstraint.delegate`.
    #[must_use]
    pub fn delegate(&self) -> Option<AvoidOccluderConstraintDelegate> {
        DelegateObject::from_retained(unsafe {
            scn_avoid_occluder_constraint_get_delegate(self.as_ptr())
        })
        .map(|inner| AvoidOccluderConstraintDelegate { inner })
    }
}

impl Scene {
    /// Mirrors `SCNScene.writeToUrl`.
    pub fn write_to_url(
        &self,
        path: impl AsRef<Path>,
        delegate: Option<&SceneExportDelegate>,
    ) -> Result<(), SceneKitError> {
        let path = cstring_from_path(path.as_ref())
            .ok_or_else(|| SceneKitError::new("path contains an interior NUL byte"))?;
        let mut error = ptr::null_mut();
        let written = unsafe {
            scn_scene_write_to_url(
                self.as_ptr(),
                path.as_ptr(),
                delegate.map_or(ptr::null_mut(), SceneExportDelegate::as_ptr),
                &raw mut error,
            )
        };
        if written {
            drop(unsafe { take_string(error) });
            Ok(())
        } else {
            Err(unsafe {
                take_error(
                    error,
                    "SCNScene.write(to:options:delegate:progressHandler:) failed",
                )
            })
        }
    }
}

/// # Safety
///
/// `context` must be a valid Objective-C `JSContext *` pointer.
pub unsafe fn export_javascript_module(context: *mut c_void) {
    scn_export_javascript_module(context);
}
