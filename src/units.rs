pub(crate) const VBUS_VOLTAGE_COEFF: u16 = 17;
pub(crate) const BATT_VOLTAGE_COEFF: u16 = 11;

pub(crate) const VBUS_CURRENT_COEFF: u16 = 375;
pub(crate) const VBUS_CURRENT_DIV: u16 = 1000;

pub(crate) const BATT_CURRENT_COEFF: u16 = 1;
pub(crate) const BATT_CURRENT_DIV: u16 = 2;

/// Defines a 12-bit voltage reading.
#[derive(Debug, Copy, Clone)]
pub struct Voltage {
    raw: u16,
    coeff: u16,
}

impl Voltage {
    pub(crate) fn new(raw: u16, coeff: u16) -> Self {
        Self {
            raw,
            coeff
        }
    }

    /// Returns milliVolts value.
    pub fn as_millivolts(&self) -> u16 {
        self.raw * self.coeff / 10
    }

    /// Returns volts value.
    pub fn as_volts(&self) -> f32 {
        self.as_millivolts() as f32 / 1000.0_f32
    }

    /// Returns raw ADC reading value.
    pub fn raw(&self) -> u16 {
        self.raw
    }
}

/// Defines a 13-bit current reading.
#[derive(Debug, Copy, Clone)]
pub struct Current {
    raw: u16,
    coeff: u16,
    div: u16,
}

impl Current {
    pub(crate) fn new(raw: u16, coeff: u16, div: u16) -> Self {
        Self {
            raw,
            coeff,
            div,
        }
    }

    /// Returns milliAmperes value.
    pub fn as_milliamps(&self) -> u16 {
        self.raw * self.coeff / self.div
    }

    /// Returns Amperes value.
    pub fn as_amperes(&self) -> f32 {
        self.as_milliamps() as f32 / 1000.0_f32
    }

    /// Returns raw ADC reading value.
    pub fn raw(&self) -> u16 {
        self.raw
    }
}