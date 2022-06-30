// SPDX-License-Identifier: MIT
use super::*;

impl From<SpecificAddress> for u8 {
    fn from(specific: SpecificAddress) -> Self {
        match specific {
            SpecificAddress::Unknown
            | SpecificAddress::Subscribers
            | SpecificAddress::Broadcast => specific.to_glib() as u8,
            // Unlikely.
            SpecificAddress::__Unknown(_) => u8::MAX,
        }
    }
}

impl From<SpecificClientId> for u8 {
    fn from(specific: SpecificClientId) -> Self {
        match specific {
            SpecificClientId::System | SpecificClientId::Dummy | SpecificClientId::Oss => {
                specific.to_glib() as u8
            }
            // Unlikely.
            SpecificClientId::__Unknown(_) => u8::MAX,
        }
    }
}

impl From<SpecificPortId> for u8 {
    fn from(specific: SpecificPortId) -> Self {
        match specific {
            SpecificPortId::Timer | SpecificPortId::Announce => specific.to_glib() as u8,
            // Unlikely.
            SpecificPortId::__Unknown(_) => u8::MAX,
        }
    }
}

impl From<SpecificQueueId> for u8 {
    fn from(specific: SpecificQueueId) -> Self {
        match specific {
            SpecificQueueId::Direct => specific.to_glib() as u8,
            // Unlikely.
            SpecificQueueId::__Unknown(_) => u8::MAX,
        }
    }
}
