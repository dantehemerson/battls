#[derive(Debug)]
pub struct Battery {
    pub name: String,
    pub manufacturer: String,
    pub capacity: u8,
    pub status: String,
    pub cycle_count: Option<u32>,
    pub energy_full: u64,
    pub energy_full_design: u64,
    pub voltage_now: u64,
    pub power_now: u64,
}

impl Battery {
    pub fn health(&self) -> Option<f64> {
        if self.energy_full_design == 0 {
            None
        } else {
            Some(self.energy_full as f64 / self.energy_full_design as f64 * 100.0)
        }
    }

    pub fn voltage(&self) -> Option<f64> {
        if self.voltage_now == 0 {
            None
        } else {
            Some(self.voltage_now as f64 / 1_000_000.0)
        }
    }

    pub fn power(&self) -> Option<f64> {
        if self.power_now == 0 {
            None
        } else {
            Some(self.power_now as f64 / 1_000_000.0)
        }
    }
}

