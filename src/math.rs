#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *const f32 {
        &self.x
    }

    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        &mut self.x
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *const f32 {
        &self.x
    }

    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        &mut self.x
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4 {
    pub elements: [f32; 16],
}

impl Matrix4 {
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

    #[must_use]
    pub const fn from_elements(elements: [f32; 16]) -> Self {
        Self { elements }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *const f32 {
        self.elements.as_ptr()
    }

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
