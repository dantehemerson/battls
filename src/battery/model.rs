#[derive(Debug)]
pub struct Battery {
    pub name: String,
    pub capacity: u8,
    pub status: String,
    pub cycle_count: Option<u32>,
    pub energy_full: u64,
    pub energy_full_design: u64,
}

impl Battery {
    pub fn health(&self) -> Option<f64> {
        if self.energy_full_design == 0 {
            None
        } else {
            Some(self.energy_full as f64 / self.energy_full_design as f64 * 100.0)
        }
    }
}

