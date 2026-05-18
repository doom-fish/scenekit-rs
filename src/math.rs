/// Represents an `SCNVector3` value.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector3 {
    /// X component matching the `SCNVector3` layout.
    pub x: f32,
    /// Y component matching the `SCNVector3` layout.
    pub y: f32,
    /// Z component matching the `SCNVector3` layout.
    pub z: f32,
}

impl Vector3 {
    /// Creates a wrapped `SCNVector3` instance.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns the zero-valued `SCNVector3` constant.
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Returns the Objective-C pointer backing this `SCNVector3` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *const f32 {
        &self.x
    }

    /// Returns the raw pointer used to drive the `SCNVector3` protocol surface.
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        &mut self.x
    }
}

/// Represents an `SCNVector4` value.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector4 {
    /// X component matching the `SCNVector4` layout.
    pub x: f32,
    /// Y component matching the `SCNVector4` layout.
    pub y: f32,
    /// Z component matching the `SCNVector4` layout.
    pub z: f32,
    /// W component matching the `SCNVector4` layout.
    pub w: f32,
}

impl Vector4 {
    /// Creates a wrapped `SCNVector4` instance.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Returns the Objective-C pointer backing this `SCNVector4` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *const f32 {
        &self.x
    }

    /// Returns the raw pointer used to drive the `SCNVector4` protocol surface.
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        &mut self.x
    }
}

/// Represents an `SCNMatrix4` value.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4 {
    /// Column-major elements matching the `SCNMatrix4` memory layout.
    pub elements: [f32; 16],
}

impl Matrix4 {
    /// Returns the `SCNMatrix4Identity` transform.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            elements: [
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    /// Builds a `SCNMatrix4` value from raw matrix elements.
    #[must_use]
    pub const fn from_elements(elements: [f32; 16]) -> Self {
        Self { elements }
    }

    /// Returns the Objective-C pointer backing this `SCNMatrix4` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *const f32 {
        self.elements.as_ptr()
    }

    /// Returns the raw pointer used to drive the `SCNMatrix4` protocol surface.
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        self.elements.as_mut_ptr()
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self::identity()
    }
}
