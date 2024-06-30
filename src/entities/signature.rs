use crate::components::ComponentKind;
use std::ops;

pub struct Signature {
    value: u32,
}

impl ops::BitOrAssign for Signature {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

impl From<ComponentKind> for Signature {
    fn from(value: ComponentKind) -> Self {
        if let Some((i, _)) = ComponentKind::VALUES
            .iter()
            .enumerate()
            .find(|(_, kind)| value == **kind)
        {
            return Self { value: 0x0001 << i };
        }
        Self { value: 0x0000 }
    }
}

impl From<Vec<ComponentKind>> for Signature {
    fn from(value: Vec<ComponentKind>) -> Self {
        let mut composite_signature = Self::new();
        for kind in value {
            composite_signature |= kind.into();
        }

        composite_signature
    }
}

impl From<Signature> for Vec<ComponentKind> {
    fn from(signature: Signature) -> Self {
        let mut kinds = Vec::new();
        for (i, kind) in ComponentKind::VALUES.iter().enumerate() {
            let kind_signature: u32 = 0x0001 << i;
            if signature.value & kind_signature != 0 {
                kinds.push(*kind);
            }
        }

        kinds
    }
}

impl Signature {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn equals(&self, other: u32) -> bool {
        self.value & other != 0
    }
}
