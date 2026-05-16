use crate::color::Color;
use crate::ffi;
use crate::private::handle_type;

handle_type!(Light);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LightType {
    Ambient = 0,
    Directional = 1,
    Omni = 2,
    Spot = 3,
    Area = 4,
    Probe = 5,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ShadowMode {
    Forward = 0,
    Deferred = 1,
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
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_light_new()) }
    }

    #[must_use]
    pub fn light_type(&self) -> Option<LightType> {
        LightType::from_raw(unsafe { ffi::scn_light_get_type(self.ptr) })
    }

    pub fn set_light_type(&self, light_type: LightType) {
        unsafe { ffi::scn_light_set_type(self.ptr, light_type as i32) };
    }

    #[must_use]
    pub fn color(&self) -> Option<Color> {
        let mut rgba = [0.0_f32; 4];
        let ok = unsafe { ffi::scn_light_copy_color(self.ptr, rgba.as_mut_ptr().cast()) };
        ok.then(|| Color::rgba(rgba[0], rgba[1], rgba[2], rgba[3]))
    }

    pub fn set_color(&self, color: Color) {
        unsafe { ffi::scn_light_set_color(self.ptr, color.r, color.g, color.b, color.a) };
    }

    #[must_use]
    pub fn intensity(&self) -> f64 {
        unsafe { ffi::scn_light_get_intensity(self.ptr) }
    }

    pub fn set_intensity(&self, intensity: f64) {
        unsafe { ffi::scn_light_set_intensity(self.ptr, intensity) };
    }

    #[must_use]
    pub fn shadow_mode(&self) -> Option<ShadowMode> {
        ShadowMode::from_raw(unsafe { ffi::scn_light_get_shadow_mode(self.ptr) })
    }

    pub fn set_shadow_mode(&self, shadow_mode: ShadowMode) {
        unsafe { ffi::scn_light_set_shadow_mode(self.ptr, shadow_mode as i32) };
    }

    #[must_use]
    pub fn casts_shadow(&self) -> bool {
        unsafe { ffi::scn_light_get_casts_shadow(self.ptr) }
    }

    pub fn set_casts_shadow(&self, casts_shadow: bool) {
        unsafe { ffi::scn_light_set_casts_shadow(self.ptr, casts_shadow) };
    }
}
