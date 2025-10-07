// Conversions between core number types and mina_p2p_messages types

use mina_p2p_messages::v2::{
    BlockTimeTimeStableV1, CurrencyAmountStableV1, CurrencyBalanceStableV1, CurrencyFeeStableV1,
    MinaNumbersGlobalSlotSinceGenesisMStableV1, MinaNumbersGlobalSlotSinceHardForkMStableV1,
    MinaNumbersGlobalSlotSpanStableV1, MinaStateBlockchainStateValueStableV2SignedAmount,
    SgnStableV1, SignedAmount, UnsignedExtendedUInt32StableV1,
    UnsignedExtendedUInt64Int64ForVersionTagsStableV1,
};

use crate::number::{Amount, Balance, BlockTime, Fee, Length, Nonce, Sgn, Signed, Slot, SlotSpan};

// Amount conversions
impl From<CurrencyAmountStableV1> for Amount {
    fn from(value: CurrencyAmountStableV1) -> Self {
        Self(value.as_u64())
    }
}

impl From<Amount> for CurrencyAmountStableV1 {
    fn from(value: Amount) -> Self {
        Self(UnsignedExtendedUInt64Int64ForVersionTagsStableV1(
            value.as_u64().into(),
        ))
    }
}

impl From<&Amount> for CurrencyAmountStableV1 {
    fn from(value: &Amount) -> Self {
        CurrencyAmountStableV1(UnsignedExtendedUInt64Int64ForVersionTagsStableV1(
            value.as_u64().into(),
        ))
    }
}

impl From<&Amount> for CurrencyFeeStableV1 {
    fn from(value: &Amount) -> Self {
        CurrencyFeeStableV1(UnsignedExtendedUInt64Int64ForVersionTagsStableV1(
            value.as_u64().into(),
        ))
    }
}

// Balance conversions
impl From<CurrencyAmountStableV1> for Balance {
    fn from(value: CurrencyAmountStableV1) -> Self {
        Self(value.as_u64())
    }
}

impl From<Balance> for CurrencyAmountStableV1 {
    fn from(value: Balance) -> Self {
        Self(UnsignedExtendedUInt64Int64ForVersionTagsStableV1(
            value.as_u64().into(),
        ))
    }
}

impl From<&Balance> for CurrencyBalanceStableV1 {
    fn from(value: &Balance) -> Self {
        Self((*value).into())
    }
}

// Fee conversions
impl From<&CurrencyFeeStableV1> for Fee {
    fn from(value: &CurrencyFeeStableV1) -> Self {
        Self(value.as_u64())
    }
}

impl From<&CurrencyAmountStableV1> for Fee {
    fn from(value: &CurrencyAmountStableV1) -> Self {
        Self(value.as_u64())
    }
}

impl From<&Fee> for CurrencyFeeStableV1 {
    fn from(value: &Fee) -> Self {
        Self(UnsignedExtendedUInt64Int64ForVersionTagsStableV1(
            value.as_u64().into(),
        ))
    }
}

impl From<&Fee> for CurrencyAmountStableV1 {
    fn from(value: &Fee) -> Self {
        Self(UnsignedExtendedUInt64Int64ForVersionTagsStableV1(
            value.as_u64().into(),
        ))
    }
}

// Nonce conversions
impl From<&Nonce> for UnsignedExtendedUInt32StableV1 {
    fn from(value: &Nonce) -> Self {
        Self(value.as_u32().into())
    }
}

impl From<&UnsignedExtendedUInt32StableV1> for Nonce {
    fn from(value: &UnsignedExtendedUInt32StableV1) -> Self {
        Self::from_u32(value.as_u32())
    }
}

// Slot conversions
impl From<&UnsignedExtendedUInt32StableV1> for Slot {
    fn from(value: &UnsignedExtendedUInt32StableV1) -> Self {
        Self::from_u32(value.as_u32())
    }
}

impl From<&Slot> for UnsignedExtendedUInt32StableV1 {
    fn from(value: &Slot) -> Self {
        Self(value.as_u32().into())
    }
}

impl From<&MinaNumbersGlobalSlotSinceGenesisMStableV1> for Slot {
    fn from(value: &MinaNumbersGlobalSlotSinceGenesisMStableV1) -> Self {
        let MinaNumbersGlobalSlotSinceGenesisMStableV1::SinceGenesis(slot) = value;
        Self(slot.as_u32())
    }
}

impl From<&Slot> for MinaNumbersGlobalSlotSinceGenesisMStableV1 {
    fn from(value: &Slot) -> Self {
        Self::SinceGenesis(value.as_u32().into())
    }
}

impl From<&MinaNumbersGlobalSlotSinceHardForkMStableV1> for Slot {
    fn from(value: &MinaNumbersGlobalSlotSinceHardForkMStableV1) -> Self {
        let MinaNumbersGlobalSlotSinceHardForkMStableV1::SinceHardFork(slot) = value;
        Self(slot.as_u32())
    }
}

impl From<&Slot> for MinaNumbersGlobalSlotSinceHardForkMStableV1 {
    fn from(value: &Slot) -> Self {
        Self::SinceHardFork(value.as_u32().into())
    }
}

// SlotSpan conversions
impl From<&MinaNumbersGlobalSlotSpanStableV1> for SlotSpan {
    fn from(value: &MinaNumbersGlobalSlotSpanStableV1) -> Self {
        let MinaNumbersGlobalSlotSpanStableV1::GlobalSlotSpan(span) = value;
        Self(span.as_u32())
    }
}

impl From<&SlotSpan> for MinaNumbersGlobalSlotSpanStableV1 {
    fn from(value: &SlotSpan) -> Self {
        Self::GlobalSlotSpan(value.as_u32().into())
    }
}

// Length conversions
impl From<&UnsignedExtendedUInt32StableV1> for Length {
    fn from(value: &UnsignedExtendedUInt32StableV1) -> Self {
        Self::from_u32(value.0.as_u32())
    }
}

impl From<&Length> for UnsignedExtendedUInt32StableV1 {
    fn from(value: &Length) -> Self {
        Self(value.as_u32().into())
    }
}

// Sgn conversions
impl From<SgnStableV1> for Sgn {
    fn from(value: SgnStableV1) -> Self {
        match value {
            SgnStableV1::Pos => Self::Pos,
            SgnStableV1::Neg => Self::Neg,
        }
    }
}

impl From<&Sgn> for SgnStableV1 {
    fn from(value: &Sgn) -> Self {
        match value {
            Sgn::Pos => Self::Pos,
            Sgn::Neg => Self::Neg,
        }
    }
}

// Signed<Amount> conversions
impl From<&SignedAmount> for Signed<Amount> {
    fn from(value: &SignedAmount) -> Self {
        Self {
            magnitude: Amount(value.magnitude.clone().as_u64()),
            sgn: value.sgn.clone().into(),
        }
    }
}

impl From<&Signed<Amount>> for SignedAmount {
    fn from(value: &Signed<Amount>) -> Self {
        Self {
            magnitude: (&value.magnitude).into(),
            sgn: (&value.sgn).into(),
        }
    }
}

impl From<&MinaStateBlockchainStateValueStableV2SignedAmount> for Signed<Amount> {
    fn from(value: &MinaStateBlockchainStateValueStableV2SignedAmount) -> Self {
        Self {
            magnitude: value.magnitude.clone().into(),
            sgn: value.sgn.clone().into(),
        }
    }
}

impl From<&Signed<Amount>> for MinaStateBlockchainStateValueStableV2SignedAmount {
    fn from(value: &Signed<Amount>) -> Self {
        Self {
            magnitude: (&value.magnitude).into(),
            sgn: (&value.sgn).into(),
        }
    }
}

// Signed<Fee> conversions
impl From<&SignedAmount> for Signed<Fee> {
    fn from(value: &SignedAmount) -> Self {
        Self {
            magnitude: (&value.magnitude).into(),
            sgn: value.sgn.clone().into(),
        }
    }
}

impl From<&Signed<Fee>> for SignedAmount {
    fn from(value: &Signed<Fee>) -> Self {
        Self {
            magnitude: (&value.magnitude).into(),
            sgn: (&value.sgn).into(),
        }
    }
}

// BlockTime conversions
impl From<BlockTimeTimeStableV1> for BlockTime {
    fn from(bt: BlockTimeTimeStableV1) -> Self {
        Self(bt.0 .0 .0)
    }
}

impl From<&BlockTime> for BlockTimeTimeStableV1 {
    fn from(value: &BlockTime) -> Self {
        Self(value.0.into())
    }
}
