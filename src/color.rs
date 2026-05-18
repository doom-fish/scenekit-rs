/// Represents RGBA color components consumed by SceneKit APIs.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red channel consumed by SceneKit color APIs.
    pub r: f32,
    /// Green channel consumed by SceneKit color APIs.
    pub g: f32,
    /// Blue channel consumed by SceneKit color APIs.
    pub b: f32,
    /// Alpha channel consumed by SceneKit color APIs.
    pub a: f32,
}

impl Color {
    /// Builds a color value consumed by SceneKit RGBA APIs.
    #[must_use]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Builds an opaque color value consumed by SceneKit RGB APIs.
    #[must_use]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// Returns the `black` color commonly used with SceneKit materials and views.
    #[must_use]
    pub const fn black() -> Self {
        Self::rgb(0.0, 0.0, 0.0)
    }

    /// Returns the `white` color commonly used with SceneKit materials and views.
    #[must_use]
    pub const fn white() -> Self {
        Self::rgb(1.0, 1.0, 1.0)
    }

    /// Returns the `red` color commonly used with SceneKit materials and views.
    #[must_use]
    pub const fn red() -> Self {
        Self::rgb(1.0, 0.0, 0.0)
    }

    /// Returns the `green` color commonly used with SceneKit materials and views.
    #[must_use]
    pub const fn green() -> Self {
        Self::rgb(0.0, 1.0, 0.0)
    }

    /// Returns the `blue` color commonly used with SceneKit materials and views.
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
