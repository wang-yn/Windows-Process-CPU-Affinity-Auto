use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreType {
    Performance,  // P-core
    Efficient,    // E-core
    Unknown,
}

#[derive(Debug, Clone)]
pub enum DetectionMode {
    Auto,
    Manual,
    AllCores,
}

impl DetectionMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "manual" => DetectionMode::Manual,
            "all_cores" | "allcores" => DetectionMode::AllCores,
            _ => DetectionMode::Auto,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoreInfo {
    /// List of P-core (Performance cores) indices
    pub p_cores: Vec<u32>,
    /// List of E-core (Efficient cores) indices
    pub e_cores: Vec<u32>,
    /// Total number of logical processors
    pub total_cores: u32,
    /// Bitmask for P-cores
    pub p_core_mask: usize,
    /// Bitmask for E-cores
    pub e_core_mask: usize,
}

impl CoreInfo {
    pub fn new(p_cores: Vec<u32>, e_cores: Vec<u32>) -> Self {
        let total_cores = p_cores.len() + e_cores.len();
        let p_core_mask = Self::calculate_mask(&p_cores);
        let e_core_mask = Self::calculate_mask(&e_cores);

        Self {
            p_cores,
            e_cores,
            total_cores: total_cores as u32,
            p_core_mask,
            e_core_mask,
        }
    }

    pub fn all_cores(count: u32) -> Self {
        let cores: Vec<u32> = (0..count).collect();
        let mask = Self::calculate_mask(&cores);

        Self {
            p_cores: cores.clone(),
            e_cores: Vec::new(),
            total_cores: count,
            p_core_mask: mask,
            e_core_mask: 0,
        }
    }

    fn calculate_mask(cores: &[u32]) -> usize {
        cores.iter().fold(0, |mask, &core| mask | (1 << core))
    }

    pub fn has_hybrid_architecture(&self) -> bool {
        !self.p_cores.is_empty() && !self.e_cores.is_empty()
    }
}

impl fmt::Display for CoreInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CoreInfo {{ total: {}, P-cores: {:?} (mask: 0x{:X}), E-cores: {:?} (mask: 0x{:X}) }}",
            self.total_cores, self.p_cores, self.p_core_mask, self.e_cores, self.e_core_mask
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_mask_calculation() {
        let p_cores = vec![0, 1, 2, 3];
        let e_cores = vec![4, 5, 6, 7];
        let info = CoreInfo::new(p_cores, e_cores);

        // P-cores: 0b00001111 = 0x0F
        assert_eq!(info.p_core_mask, 0x0F);
        // E-cores: 0b11110000 = 0xF0
        assert_eq!(info.e_core_mask, 0xF0);
        assert_eq!(info.total_cores, 8);
        assert!(info.has_hybrid_architecture());
    }

    #[test]
    fn test_all_cores() {
        let info = CoreInfo::all_cores(8);
        assert_eq!(info.total_cores, 8);
        assert_eq!(info.p_core_mask, 0xFF);
        assert_eq!(info.e_core_mask, 0);
        assert!(!info.has_hybrid_architecture());
    }
}
