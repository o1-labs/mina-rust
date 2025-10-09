use mina_p2p_messages::v2::MinaBaseSignedCommandStableV2;
use mina_signer::{CompressedPubKey, Signature};

use crate::{
    decompress_pk,
    scan_state::{
        currency::{Amount, Fee, Nonce, Signed, Slot},
        fee_excess::FeeExcess,
    },
    AccountId, TokenId,
};

use super::{zkapp_command::AccessedOrNot, Memo, TransactionStatus};

/// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/signed_command_payload.ml#L75>
#[derive(Debug, Clone, PartialEq)]
pub struct Common {
    pub fee: Fee,
    pub fee_payer_pk: CompressedPubKey,
    pub nonce: Nonce,
    pub valid_until: Slot,
    pub memo: Memo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentPayload {
    pub receiver_pk: CompressedPubKey,
    pub amount: Amount,
}

/// <https://github.com/MinaProtocol/mina/blob/bfd1009abdbee78979ff0343cc73a3480e862f58/src/lib/mina_base/stake_delegation.ml#L11>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StakeDelegationPayload {
    SetDelegate { new_delegate: CompressedPubKey },
}

impl StakeDelegationPayload {
    /// <https://github.com/MinaProtocol/mina/blob/bfd1009abdbee78979ff0343cc73a3480e862f58/src/lib/mina_base/stake_delegation.ml#L35>
    pub fn receiver(&self) -> AccountId {
        let Self::SetDelegate { new_delegate } = self;
        AccountId::new(new_delegate.clone(), TokenId::default())
    }

    /// <https://github.com/MinaProtocol/mina/blob/bfd1009abdbee78979ff0343cc73a3480e862f58/src/lib/mina_base/stake_delegation.ml#L33>
    pub fn receiver_pk(&self) -> &CompressedPubKey {
        let Self::SetDelegate { new_delegate } = self;
        new_delegate
    }
}

/// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/signed_command_payload.mli#L24>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Payment(PaymentPayload),
    StakeDelegation(StakeDelegationPayload),
}

/// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/signed_command_payload.mli#L165>
#[derive(Debug, Clone, PartialEq)]
pub struct SignedCommandPayload {
    pub common: Common,
    pub body: Body,
}

impl SignedCommandPayload {
    pub fn create(
        fee: Fee,
        fee_payer_pk: CompressedPubKey,
        nonce: Nonce,
        valid_until: Option<Slot>,
        memo: Memo,
        body: Body,
    ) -> Self {
        Self {
            common: Common {
                fee,
                fee_payer_pk,
                nonce,
                valid_until: valid_until.unwrap_or_else(Slot::max),
                memo,
            },
            body,
        }
    }
}

/// <https://github.com/MinaProtocol/mina/blob/1551e2faaa246c01636908aabe5f7981715a10f4/src/lib/mina_base/signed_command_payload.ml#L362>
mod weight {
    use super::*;

    fn payment(_: &PaymentPayload) -> u64 {
        1
    }
    fn stake_delegation(_: &StakeDelegationPayload) -> u64 {
        1
    }
    pub fn of_body(body: &Body) -> u64 {
        match body {
            Body::Payment(p) => payment(p),
            Body::StakeDelegation(s) => stake_delegation(s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(into = "MinaBaseSignedCommandStableV2")]
#[serde(try_from = "MinaBaseSignedCommandStableV2")]
pub struct SignedCommand {
    pub payload: SignedCommandPayload,
    pub signer: CompressedPubKey, // TODO: This should be a `mina_signer::PubKey`
    pub signature: Signature,
}

impl SignedCommand {
    pub fn valid_until(&self) -> Slot {
        self.payload.common.valid_until
    }

    /// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/signed_command_payload.ml#L322>
    pub fn fee_payer(&self) -> AccountId {
        let public_key = self.payload.common.fee_payer_pk.clone();
        AccountId::new(public_key, TokenId::default())
    }

    /// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/signed_command_payload.ml#L320>
    pub fn fee_payer_pk(&self) -> &CompressedPubKey {
        &self.payload.common.fee_payer_pk
    }

    pub fn weight(&self) -> u64 {
        let Self {
            payload: SignedCommandPayload { common: _, body },
            signer: _,
            signature: _,
        } = self;
        weight::of_body(body)
    }

    /// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/signed_command_payload.ml#L318>
    pub fn fee_token(&self) -> TokenId {
        TokenId::default()
    }

    pub fn fee(&self) -> Fee {
        self.payload.common.fee
    }

    /// <https://github.com/MinaProtocol/mina/blob/05c2f73d0f6e4f1341286843814ce02dcb3919e0/src/lib/mina_base/signed_command_payload.ml#L250>
    pub fn receiver(&self) -> AccountId {
        match &self.payload.body {
            Body::Payment(payload) => {
                AccountId::new(payload.receiver_pk.clone(), TokenId::default())
            }
            Body::StakeDelegation(payload) => payload.receiver(),
        }
    }

    /// <https://github.com/MinaProtocol/mina/blob/05c2f73d0f6e4f1341286843814ce02dcb3919e0/src/lib/mina_base/signed_command_payload.ml#L234>
    pub fn receiver_pk(&self) -> &CompressedPubKey {
        match &self.payload.body {
            Body::Payment(payload) => &payload.receiver_pk,
            Body::StakeDelegation(payload) => payload.receiver_pk(),
        }
    }

    pub fn amount(&self) -> Option<Amount> {
        match &self.payload.body {
            Body::Payment(payload) => Some(payload.amount),
            Body::StakeDelegation(_) => None,
        }
    }

    pub fn nonce(&self) -> Nonce {
        self.payload.common.nonce
    }

    pub fn fee_excess(&self) -> FeeExcess {
        FeeExcess::of_single((self.fee_token(), Signed::<Fee>::of_unsigned(self.fee())))
    }

    /// <https://github.com/MinaProtocol/mina/blob/802634fdda92f5cba106fd5f98bd0037c4ec14be/src/lib/mina_base/signed_command_payload.ml#L322>
    pub fn account_access_statuses(
        &self,
        status: &TransactionStatus,
    ) -> Vec<(AccountId, AccessedOrNot)> {
        use AccessedOrNot::*;
        use TransactionStatus::*;

        match status {
            Applied => vec![(self.fee_payer(), Accessed), (self.receiver(), Accessed)],
            // Note: The fee payer is always accessed, even if the transaction fails
            // <https://github.com/MinaProtocol/mina/blob/802634fdda92f5cba106fd5f98bd0037c4ec14be/src/lib/mina_base/signed_command_payload.mli#L205>
            Failed(_) => vec![(self.fee_payer(), Accessed), (self.receiver(), NotAccessed)],
        }
    }

    pub fn accounts_referenced(&self) -> Vec<AccountId> {
        self.account_access_statuses(&TransactionStatus::Applied)
            .into_iter()
            .map(|(id, _status)| id)
            .collect()
    }

    /// <https://github.com/MinaProtocol/mina/blob/05c2f73d0f6e4f1341286843814ce02dcb3919e0/src/lib/mina_base/signed_command.ml#L401>
    pub fn public_keys(&self) -> [&CompressedPubKey; 2] {
        [self.fee_payer_pk(), self.receiver_pk()]
    }

    /// <https://github.com/MinaProtocol/mina/blob/05c2f73d0f6e4f1341286843814ce02dcb3919e0/src/lib/mina_base/signed_command.ml#L407>
    pub fn check_valid_keys(&self) -> bool {
        self.public_keys()
            .into_iter()
            .all(|pk| decompress_pk(pk).is_some())
    }
}
