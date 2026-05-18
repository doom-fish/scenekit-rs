use crate::color::Color;
use crate::ffi;
use crate::private::handle_type;

handle_type!(Light);

/// Mirrors `SCNLight.lightType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LightType {
    /// Corresponds to the `SCNLight.lightType::Ambient` case.
    Ambient = 0,
    /// Corresponds to the `SCNLight.lightType::Directional` case.
    Directional = 1,
    /// Corresponds to the `SCNLight.lightType::Omni` case.
    Omni = 2,
    /// Corresponds to the `SCNLight.lightType::Spot` case.
    Spot = 3,
    /// Corresponds to the `SCNLight.lightType::Area` case.
    Area = 4,
    /// Corresponds to the `SCNLight.lightType::Probe` case.
    Probe = 5,
    /// Corresponds to the `SCNLight.lightType::Ies` case.
    Ies = 6,
}

impl LightType {
    const fn from_raw(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Ambient),
            1 => Some(Self::Directional),
            2 => Some(Self::Omni),
            3 => Some(Self::Spot),
            4 => Some(Self::Area),
            5 => Some(Self::Probe),
            6 => Some(Self::Ies),
            _ => None,
        }
    }
}

/// Mirrors `SCNShadowMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ShadowMode {
    /// Corresponds to the `SCNShadowMode::Forward` case.
    Forward = 0,
    /// Corresponds to the `SCNShadowMode::Deferred` case.
    Deferred = 1,
    /// Corresponds to the `SCNShadowMode::Modulated` case.
    Modulated = 2,
}

impl ShadowMode {
    const fn from_raw(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Forward),
            1 => Some(Self::Deferred),
            2 => Some(Self::Modulated),
            _ => None,
        }
    }
}

impl Light {
    /// Creates a wrapped `SCNLight` instance.
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_light_new()) }
    }

    /// Mirrors `SCNLight.lightType`.
    #[must_use]
    pub fn light_type(&self) -> Option<LightType> {
        LightType::from_raw(unsafe { ffi::scn_light_get_type(self.ptr) })
    }

    /// Sets the `SCNLight.lightType` member.
    pub fn set_light_type(&self, light_type: LightType) {
        unsafe { ffi::scn_light_set_type(self.ptr, light_type as i32) };
    }

    /// Mirrors `SCNLight.color`.
    #[must_use]
    pub fn color(&self) -> Option<Color> {
        let mut rgba = [0.0_f32; 4];
        let ok = unsafe { ffi::scn_light_copy_color(self.ptr, rgba.as_mut_ptr().cast()) };
        ok.then(|| Color::rgba(rgba[0], rgba[1], rgba[2], rgba[3]))
    }

    /// Sets the `SCNLight.color` member.
    pub fn set_color(&self, color: Color) {
        unsafe { ffi::scn_light_set_color(self.ptr, color.r, color.g, color.b, color.a) };
    }

    /// Mirrors `SCNLight.intensity`.
    #[must_use]
    pub fn intensity(&self) -> f64 {
        unsafe { ffi::scn_light_get_intensity(self.ptr) }
    }

    /// Sets the `SCNLight.intensity` member.
    pub fn set_intensity(&self, intensity: f64) {
        unsafe { ffi::scn_light_set_intensity(self.ptr, intensity) };
    }

    /// Mirrors `SCNLight.shadowMode`.
    #[must_use]
    pub fn shadow_mode(&self) -> Option<ShadowMode> {
        ShadowMode::from_raw(unsafe { ffi::scn_light_get_shadow_mode(self.ptr) })
    }

    /// Sets the `SCNLight.shadowMode` member.
    pub fn set_shadow_mode(&self, shadow_mode: ShadowMode) {
        unsafe { ffi::scn_light_set_shadow_mode(self.ptr, shadow_mode as i32) };
    }

    /// Mirrors `SCNLight.castsShadow`.
    #[must_use]
    pub fn casts_shadow(&self) -> bool {
        unsafe { ffi::scn_light_get_casts_shadow(self.ptr) }
    }

    /// Sets the `SCNLight.castsShadow` member.
    pub fn set_casts_shadow(&self, casts_shadow: bool) {
        unsafe { ffi::scn_light_set_casts_shadow(self.ptr, casts_shadow) };
    }
}
