use std::fmt;

trait UnitBase {
    fn conversion_factor_to_base(&self) -> f64;
}

#[derive(Debug, Clone, Copy)]
pub enum WeightUnit {
    Kilogram,
    Gram,
    Milligram,
    Microgram,
    Nanogram,
    Picogram,
}

#[derive(Debug, Clone, Copy)]
pub enum VolumeUnit {
    Liter,
    Deciliter,
    Milliliter,
    Microliter,
    Nanoliter,
    Picoliter,
}

impl fmt::Display for WeightUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WeightUnit::Kilogram => write!(f, "Kilogram"),
            WeightUnit::Gram => write!(f, "Gram"),
            WeightUnit::Milligram => write!(f, "Milligram"),
            WeightUnit::Microgram => write!(f, "Microgram"),
            WeightUnit::Nanogram => write!(f, "Nanogram"),
            WeightUnit::Picogram => write!(f, "Picogram"),
        }
    }
}

impl fmt::Display for VolumeUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VolumeUnit::Liter => write!(f, "Liter"),
            VolumeUnit::Deciliter => write!(f, "Deciliter"),
            VolumeUnit::Milliliter => write!(f, "Milliliter"),
            VolumeUnit::Microliter => write!(f, "Microliter"),
            VolumeUnit::Nanoliter => write!(f, "Nanoliter"),
            VolumeUnit::Picoliter => write!(f, "Picoliter"),
        }
    }
}

impl WeightUnit {
    pub fn conversion_factor_to_base(&self) -> f64 {
        match self {
            WeightUnit::Kilogram => 1_000.0,
            WeightUnit::Gram => 1.0,
            WeightUnit::Milligram => 0.001,
            WeightUnit::Microgram => 0.000_001,
            WeightUnit::Nanogram => 0.000_000_001,
            WeightUnit::Picogram => 0.000_000_000_001,
        }
    }
}

impl VolumeUnit {
    pub fn conversion_factor_to_base(&self) -> f64 {
        match self {
            VolumeUnit::Liter => 1.0,
            VolumeUnit::Deciliter => 0.1,
            VolumeUnit::Milliliter => 0.001,
            VolumeUnit::Microliter => 0.000_001,
            VolumeUnit::Nanoliter => 0.000_000_001,
            VolumeUnit::Picoliter => 0.000_000_000_001,
        }
    }
}

pub fn convert_units(
    value: f64,
    from_weight: WeightUnit,
    from_volume: VolumeUnit,
    to_weight: WeightUnit,
    to_volume: VolumeUnit,
) -> f64 {
    let weight_conversion_factor =
        from_weight.conversion_factor_to_base() / to_weight.conversion_factor_to_base();
    let volume_conversion_factor =
        to_volume.conversion_factor_to_base() / from_volume.conversion_factor_to_base();

    value * weight_conversion_factor * volume_conversion_factor
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_value_1() {
        assert_eq!(
            convert_units(
                1.0,
                WeightUnit::Kilogram,
                VolumeUnit::Liter,
                WeightUnit::Gram,
                VolumeUnit::Milliliter
            ),
            1.0
        );
    }

    #[test]
    fn test_convert_value_2() {
        assert_eq!(
            convert_units(
                1.0,
                WeightUnit::Kilogram,
                VolumeUnit::Liter,
                WeightUnit::Gram,
                VolumeUnit::Deciliter
            ),
            100.0
        );
    }
}
