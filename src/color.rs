#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    #[must_use]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    #[must_use]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    #[must_use]
    pub const fn black() -> Self {
        Self::rgb(0.0, 0.0, 0.0)
    }

    #[must_use]
    pub const fn white() -> Self {
        Self::rgb(1.0, 1.0, 1.0)
    }

    #[must_use]
    pub const fn red() -> Self {
        Self::rgb(1.0, 0.0, 0.0)
    }

    #[must_use]
    pub const fn green() -> Self {
        Self::rgb(0.0, 1.0, 0.0)
    }

    #[must_use]
    pub const fn blue() -> Self {
        Self::rgb(0.0, 0.0, 1.0)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}
