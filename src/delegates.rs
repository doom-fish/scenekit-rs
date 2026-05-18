use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;

use crate::extended_constraints::AvoidOccluderConstraint;
use crate::node::Node;
use crate::private::cstring_from_path;
use crate::renderer::Renderer;
use crate::scene::Scene;

extern "C" {
    fn scn_node_renderer_delegate_new(
        context: *mut c_void,
        release_context: extern "C" fn(*mut c_void),
        render_callback: extern "C" fn(*mut c_void, *mut c_void, *mut c_void),
    ) -> *mut c_void;
    fn scn_node_get_renderer_delegate(node: *mut c_void) -> *mut c_void;
    fn scn_node_set_renderer_delegate(node: *mut c_void, delegate: *mut c_void);
    fn scn_node_test_invoke_renderer_delegate(node: *mut c_void, renderer: *mut c_void);

    fn scn_avoid_occluder_constraint_delegate_new(
        context: *mut c_void,
        release_context: extern "C" fn(*mut c_void),
        should_avoid_callback: extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> bool,
        did_avoid_callback: extern "C" fn(*mut c_void, *mut c_void, *mut c_void),
    ) -> *mut c_void;
    fn scn_avoid_occluder_constraint_get_delegate(constraint: *mut c_void) -> *mut c_void;
    fn scn_avoid_occluder_constraint_set_delegate(constraint: *mut c_void, delegate: *mut c_void);
    fn scn_avoid_occluder_constraint_test_invoke_should(
        constraint: *mut c_void,
        occluder: *mut c_void,
        node: *mut c_void,
    ) -> bool;
    fn scn_avoid_occluder_constraint_test_invoke_did(
        constraint: *mut c_void,
        occluder: *mut c_void,
        node: *mut c_void,
    );

    fn scn_scene_export_delegate_new(
        context: *mut c_void,
        release_context: extern "C" fn(*mut c_void),
        write_image_callback: extern "C" fn(
            *mut c_void,
            *const c_char,
            *const c_char,
        ) -> *const c_char,
    ) -> *mut c_void;
    fn scn_scene_write_to_url(
        scene: *mut c_void,
        path: *const c_char,
        delegate: *mut c_void,
    ) -> bool;

    fn scn_export_javascript_module(context: *mut c_void);
}

type NodeRendererCallback = Box<dyn FnMut(&Node, &Renderer)>;
type AvoidOccluderShouldCallback = Box<dyn FnMut(&Node, &Node) -> bool>;
type AvoidOccluderDidCallback = Box<dyn FnMut(&Node, &Node)>;
type SceneExportWriteImageCallback = Box<dyn FnMut(&str, Option<&str>) -> Option<String>>;

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
        F: FnMut(&Node, &Renderer) + 'static,
    {
        self.render = Some(Box::new(callback));
        self
    }
}

struct NodeRendererDelegateState {
    callbacks: NodeRendererDelegateCallbacks,
}

/// Wraps `SCNNodeRendererDelegate`.
pub struct NodeRendererDelegate {
    ptr: *mut c_void,
}

impl core::fmt::Debug for NodeRendererDelegate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NodeRendererDelegate")
            .field("ptr", &self.ptr)
            .finish()
    }
}

impl Drop for NodeRendererDelegate {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { crate::ffi::scn_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

unsafe fn node_renderer_state_from_context<'a>(
    context: *mut c_void,
) -> &'a mut NodeRendererDelegateState {
    &mut *context.cast::<NodeRendererDelegateState>()
}

extern "C" fn release_node_renderer_context(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(context.cast::<NodeRendererDelegateState>()));
    }
}

extern "C" fn node_renderer_trampoline(
    context: *mut c_void,
    node: *mut c_void,
    renderer: *mut c_void,
) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if context.is_null() || node.is_null() || renderer.is_null() {
            return;
        }
        let state = unsafe { node_renderer_state_from_context(context) };
        if let Some(callback) = state.callbacks.render.as_mut() {
            let node = unsafe { Node::from_raw_borrowed(node) };
            let renderer = unsafe { Renderer::from_raw_borrowed(renderer) };
            callback(&node, &renderer);
        }
    }));
}

impl NodeRendererDelegate {
    /// Creates a wrapped `SCNNodeRendererDelegate` instance.
    #[must_use]
    pub fn new(callbacks: NodeRendererDelegateCallbacks) -> Option<Self> {
        let state = Box::new(NodeRendererDelegateState { callbacks });
        let context = Box::into_raw(state).cast::<c_void>();
        let ptr = unsafe {
            scn_node_renderer_delegate_new(
                context,
                release_node_renderer_context,
                node_renderer_trampoline,
            )
        };
        if ptr.is_null() {
            release_node_renderer_context(context);
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Returns the Objective-C pointer backing this `SCNNodeRendererDelegate` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
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
        F: FnMut(&Node, &Node) -> bool + 'static,
    {
        self.should_avoid_occluder = Some(Box::new(callback));
        self
    }

    /// Mirrors `SCNAvoidOccluderConstraintDelegate.onDidAvoidOccluder`.
    #[must_use]
    pub fn on_did_avoid_occluder<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&Node, &Node) + 'static,
    {
        self.did_avoid_occluder = Some(Box::new(callback));
        self
    }
}

struct AvoidOccluderConstraintDelegateState {
    callbacks: AvoidOccluderConstraintDelegateCallbacks,
}

/// Wraps `SCNAvoidOccluderConstraintDelegate`.
pub struct AvoidOccluderConstraintDelegate {
    ptr: *mut c_void,
}

impl core::fmt::Debug for AvoidOccluderConstraintDelegate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AvoidOccluderConstraintDelegate")
            .field("ptr", &self.ptr)
            .finish()
    }
}

impl Drop for AvoidOccluderConstraintDelegate {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { crate::ffi::scn_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

unsafe fn avoid_occluder_state_from_context<'a>(
    context: *mut c_void,
) -> &'a mut AvoidOccluderConstraintDelegateState {
    &mut *context.cast::<AvoidOccluderConstraintDelegateState>()
}

extern "C" fn release_avoid_occluder_context(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(
            context.cast::<AvoidOccluderConstraintDelegateState>(),
        ));
    }
}

extern "C" fn avoid_occluder_should_trampoline(
    context: *mut c_void,
    occluder: *mut c_void,
    node: *mut c_void,
) -> bool {
    catch_unwind(AssertUnwindSafe(|| {
        if context.is_null() || occluder.is_null() || node.is_null() {
            return true;
        }
        let state = unsafe { avoid_occluder_state_from_context(context) };
        state
            .callbacks
            .should_avoid_occluder
            .as_mut()
            .map_or(true, |callback| {
                let occluder = unsafe { Node::from_raw_borrowed(occluder) };
                let node = unsafe { Node::from_raw_borrowed(node) };
                callback(&occluder, &node)
            })
    }))
    .unwrap_or(true)
}

extern "C" fn avoid_occluder_did_trampoline(
    context: *mut c_void,
    occluder: *mut c_void,
    node: *mut c_void,
) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if context.is_null() || occluder.is_null() || node.is_null() {
            return;
        }
        let state = unsafe { avoid_occluder_state_from_context(context) };
        if let Some(callback) = state.callbacks.did_avoid_occluder.as_mut() {
            let occluder = unsafe { Node::from_raw_borrowed(occluder) };
            let node = unsafe { Node::from_raw_borrowed(node) };
            callback(&occluder, &node);
        }
    }));
}

impl AvoidOccluderConstraintDelegate {
    /// Creates a wrapped `SCNAvoidOccluderConstraintDelegate` instance.
    #[must_use]
    pub fn new(callbacks: AvoidOccluderConstraintDelegateCallbacks) -> Option<Self> {
        let state = Box::new(AvoidOccluderConstraintDelegateState { callbacks });
        let context = Box::into_raw(state).cast::<c_void>();
        let ptr = unsafe {
            scn_avoid_occluder_constraint_delegate_new(
                context,
                release_avoid_occluder_context,
                avoid_occluder_should_trampoline,
                avoid_occluder_did_trampoline,
            )
        };
        if ptr.is_null() {
            release_avoid_occluder_context(context);
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Returns the Objective-C pointer backing this `SCNAvoidOccluderConstraintDelegate` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

struct SceneExportDelegateState {
    callback: SceneExportWriteImageCallback,
    returned_path: Option<CString>,
}

/// Wraps `SCNSceneExportDelegate`.
pub struct SceneExportDelegate {
    ptr: *mut c_void,
}

impl core::fmt::Debug for SceneExportDelegate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SceneExportDelegate")
            .field("ptr", &self.ptr)
            .finish()
    }
}

impl Drop for SceneExportDelegate {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { crate::ffi::scn_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

unsafe fn scene_export_state_from_context<'a>(
    context: *mut c_void,
) -> &'a mut SceneExportDelegateState {
    &mut *context.cast::<SceneExportDelegateState>()
}

extern "C" fn release_scene_export_context(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(context.cast::<SceneExportDelegateState>()));
    }
}

extern "C" fn scene_export_write_image_trampoline(
    context: *mut c_void,
    document_url: *const c_char,
    original_image_url: *const c_char,
) -> *const c_char {
    catch_unwind(AssertUnwindSafe(|| {
        if context.is_null() || document_url.is_null() {
            return ptr::null();
        }
        let state = unsafe { scene_export_state_from_context(context) };
        let document_url = unsafe { std::ffi::CStr::from_ptr(document_url) }
            .to_string_lossy()
            .into_owned();
        let original_image_url = (!original_image_url.is_null()).then(|| unsafe {
            std::ffi::CStr::from_ptr(original_image_url)
                .to_string_lossy()
                .into_owned()
        });
        state.returned_path =
            (state.callback)(document_url.as_str(), original_image_url.as_deref())
                .and_then(|path| CString::new(path).ok());
        state
            .returned_path
            .as_ref()
            .map_or(ptr::null(), |path| path.as_ptr())
    }))
    .unwrap_or(ptr::null())
}

impl SceneExportDelegate {
    /// Creates a wrapped `SCNSceneExportDelegate` instance.
    #[must_use]
    pub fn new<F>(callback: F) -> Option<Self>
    where
        F: FnMut(&str, Option<&str>) -> Option<String> + 'static,
    {
        let state = Box::new(SceneExportDelegateState {
            callback: Box::new(callback),
            returned_path: None,
        });
        let context = Box::into_raw(state).cast::<c_void>();
        let ptr = unsafe {
            scn_scene_export_delegate_new(
                context,
                release_scene_export_context,
                scene_export_write_image_trampoline,
            )
        };
        if ptr.is_null() {
            release_scene_export_context(context);
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Returns the Objective-C pointer backing this `SCNSceneExportDelegate` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
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
        unsafe {
            Some(NodeRendererDelegate {
                ptr: scn_node_get_renderer_delegate(self.as_ptr()),
            })
            .filter(|d| !d.ptr.is_null())
        }
    }

    /// Mirrors `SCNNode.testInvokeRendererDelegate`.
    pub fn test_invoke_renderer_delegate(&self, renderer: &Renderer) {
        unsafe { scn_node_test_invoke_renderer_delegate(self.as_ptr(), renderer.as_ptr()) };
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
        unsafe {
            Some(AvoidOccluderConstraintDelegate {
                ptr: scn_avoid_occluder_constraint_get_delegate(self.as_ptr()),
            })
            .filter(|d| !d.ptr.is_null())
        }
    }

    /// Mirrors `SCNAvoidOccluderConstraint.testInvokeShouldAvoidOccluder`.
    #[must_use]
    pub fn test_invoke_should_avoid_occluder(&self, occluder: &Node, node: &Node) -> bool {
        unsafe {
            scn_avoid_occluder_constraint_test_invoke_should(
                self.as_ptr(),
                occluder.as_ptr(),
                node.as_ptr(),
            )
        }
    }

    /// Mirrors `SCNAvoidOccluderConstraint.testInvokeDidAvoidOccluder`.
    pub fn test_invoke_did_avoid_occluder(&self, occluder: &Node, node: &Node) {
        unsafe {
            scn_avoid_occluder_constraint_test_invoke_did(
                self.as_ptr(),
                occluder.as_ptr(),
                node.as_ptr(),
            );
        };
    }
}

impl Scene {
    /// Mirrors `SCNScene.writeToUrl`.
    #[must_use]
    pub fn write_to_url(
        &self,
        path: impl AsRef<Path>,
        delegate: Option<&SceneExportDelegate>,
    ) -> bool {
        let Some(path) = cstring_from_path(path.as_ref()) else {
            return false;
        };
        unsafe {
            scn_scene_write_to_url(
                self.as_ptr(),
                path.as_ptr(),
                delegate.map_or(ptr::null_mut(), SceneExportDelegate::as_ptr),
            )
        }
    }
}

/// # Safety
///
/// `context` must be a valid Objective-C `JSContext *` pointer.
pub unsafe fn export_javascript_module(context: *mut c_void) {
    scn_export_javascript_module(context);
}
