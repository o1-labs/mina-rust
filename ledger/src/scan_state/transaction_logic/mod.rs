use self::{
    local_state::{apply_zkapp_command_first_pass, apply_zkapp_command_second_pass, LocalStateEnv},
    protocol_state::{GlobalState, ProtocolStateView},
    signed_command::{SignedCommand, SignedCommandPayload},
    transaction_applied::{
        signed_command_applied::{self, SignedCommandApplied},
        TransactionApplied,
    },
    zkapp_command::{AccessedOrNot, ZkAppCommand},
};
use super::{
    currency::{Amount, Balance, Fee, Magnitude, Nonce, Signed, Slot},
    fee_excess::FeeExcess,
    fee_rate::FeeRate,
    scan_state::transaction_snark::OneOrTwo,
};
use crate::{
    scan_state::transaction_logic::{
        transaction_applied::{CommandApplied, Varying},
        zkapp_command::MaybeWithStatus,
    },
    sparse_ledger::LedgerIntf,
    zkapps::non_snark::LedgerNonSnark,
    Account, AccountId, BaseLedger, ControlTag, Timing, TokenId, VerificationKeyWire,
};
use mina_core::constants::ConstraintConstants;
use mina_curves::pasta::Fp;
use mina_macros::SerdeYojsonEnum;
use mina_p2p_messages::{
    bigint::InvalidBigInt,
    binprot,
    v2::{MinaBaseUserCommandStableV2, MinaTransactionTransactionStableV2},
};
use mina_signer::CompressedPubKey;
use poseidon::hash::params::MINA_ZKAPP_MEMO;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Display,
};

pub mod local_state;
pub mod protocol_state;
pub mod signed_command;
pub mod transaction_applied;
pub mod transaction_partially_applied;
pub mod transaction_union_payload;
pub mod transaction_witness;
pub mod valid;
pub mod verifiable;
pub mod zkapp_command;
pub mod zkapp_statement;
pub use transaction_partially_applied::{
    apply_transaction_first_pass, apply_transaction_second_pass, apply_transactions,
    apply_user_command, set_with_location, AccountState,
};
pub use transaction_union_payload::{
    account_check_timing, add_amount, checked_cons_signed_command_payload,
    cons_signed_command_payload, cons_zkapp_command_commitment, get_with_location, sub_amount,
    timing_error_to_user_command_status, validate_nonces, validate_timing, Body, Common,
    ExistingOrNew, Tag, TimingValidation, TransactionUnion, TransactionUnionPayload,
};

/// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/transaction_status.ml#L9>
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TransactionFailure {
    Predicate,
    SourceNotPresent,
    ReceiverNotPresent,
    AmountInsufficientToCreateAccount,
    CannotPayCreationFeeInToken,
    SourceInsufficientBalance,
    SourceMinimumBalanceViolation,
    ReceiverAlreadyExists,
    TokenOwnerNotCaller,
    Overflow,
    GlobalExcessOverflow,
    LocalExcessOverflow,
    LocalSupplyIncreaseOverflow,
    GlobalSupplyIncreaseOverflow,
    SignedCommandOnZkappAccount,
    ZkappAccountNotPresent,
    UpdateNotPermittedBalance,
    UpdateNotPermittedAccess,
    UpdateNotPermittedTiming,
    UpdateNotPermittedDelegate,
    UpdateNotPermittedAppState,
    UpdateNotPermittedVerificationKey,
    UpdateNotPermittedActionState,
    UpdateNotPermittedZkappUri,
    UpdateNotPermittedTokenSymbol,
    UpdateNotPermittedPermissions,
    UpdateNotPermittedNonce,
    UpdateNotPermittedVotingFor,
    ZkappCommandReplayCheckFailed,
    FeePayerNonceMustIncrease,
    FeePayerMustBeSigned,
    AccountBalancePreconditionUnsatisfied,
    AccountNoncePreconditionUnsatisfied,
    AccountReceiptChainHashPreconditionUnsatisfied,
    AccountDelegatePreconditionUnsatisfied,
    AccountActionStatePreconditionUnsatisfied,
    AccountAppStatePreconditionUnsatisfied(u64),
    AccountProvedStatePreconditionUnsatisfied,
    AccountIsNewPreconditionUnsatisfied,
    ProtocolStatePreconditionUnsatisfied,
    UnexpectedVerificationKeyHash,
    ValidWhilePreconditionUnsatisfied,
    IncorrectNonce,
    InvalidFeeExcess,
    Cancelled,
}

impl Display for TransactionFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Predicate => "Predicate",
            Self::SourceNotPresent => "Source_not_present",
            Self::ReceiverNotPresent => "Receiver_not_present",
            Self::AmountInsufficientToCreateAccount => "Amount_insufficient_to_create_account",
            Self::CannotPayCreationFeeInToken => "Cannot_pay_creation_fee_in_token",
            Self::SourceInsufficientBalance => "Source_insufficient_balance",
            Self::SourceMinimumBalanceViolation => "Source_minimum_balance_violation",
            Self::ReceiverAlreadyExists => "Receiver_already_exists",
            Self::TokenOwnerNotCaller => "Token_owner_not_caller",
            Self::Overflow => "Overflow",
            Self::GlobalExcessOverflow => "Global_excess_overflow",
            Self::LocalExcessOverflow => "Local_excess_overflow",
            Self::LocalSupplyIncreaseOverflow => "Local_supply_increase_overflow",
            Self::GlobalSupplyIncreaseOverflow => "Global_supply_increase_overflow",
            Self::SignedCommandOnZkappAccount => "Signed_command_on_zkapp_account",
            Self::ZkappAccountNotPresent => "Zkapp_account_not_present",
            Self::UpdateNotPermittedBalance => "Update_not_permitted_balance",
            Self::UpdateNotPermittedAccess => "Update_not_permitted_access",
            Self::UpdateNotPermittedTiming => "Update_not_permitted_timing",
            Self::UpdateNotPermittedDelegate => "update_not_permitted_delegate",
            Self::UpdateNotPermittedAppState => "Update_not_permitted_app_state",
            Self::UpdateNotPermittedVerificationKey => "Update_not_permitted_verification_key",
            Self::UpdateNotPermittedActionState => "Update_not_permitted_action_state",
            Self::UpdateNotPermittedZkappUri => "Update_not_permitted_zkapp_uri",
            Self::UpdateNotPermittedTokenSymbol => "Update_not_permitted_token_symbol",
            Self::UpdateNotPermittedPermissions => "Update_not_permitted_permissions",
            Self::UpdateNotPermittedNonce => "Update_not_permitted_nonce",
            Self::UpdateNotPermittedVotingFor => "Update_not_permitted_voting_for",
            Self::ZkappCommandReplayCheckFailed => "Zkapp_command_replay_check_failed",
            Self::FeePayerNonceMustIncrease => "Fee_payer_nonce_must_increase",
            Self::FeePayerMustBeSigned => "Fee_payer_must_be_signed",
            Self::AccountBalancePreconditionUnsatisfied => {
                "Account_balance_precondition_unsatisfied"
            }
            Self::AccountNoncePreconditionUnsatisfied => "Account_nonce_precondition_unsatisfied",
            Self::AccountReceiptChainHashPreconditionUnsatisfied => {
                "Account_receipt_chain_hash_precondition_unsatisfied"
            }
            Self::AccountDelegatePreconditionUnsatisfied => {
                "Account_delegate_precondition_unsatisfied"
            }
            Self::AccountActionStatePreconditionUnsatisfied => {
                "Account_action_state_precondition_unsatisfied"
            }
            Self::AccountAppStatePreconditionUnsatisfied(i) => {
                return write!(f, "Account_app_state_{}_precondition_unsatisfied", i);
            }
            Self::AccountProvedStatePreconditionUnsatisfied => {
                "Account_proved_state_precondition_unsatisfied"
            }
            Self::AccountIsNewPreconditionUnsatisfied => "Account_is_new_precondition_unsatisfied",
            Self::ProtocolStatePreconditionUnsatisfied => "Protocol_state_precondition_unsatisfied",
            Self::IncorrectNonce => "Incorrect_nonce",
            Self::InvalidFeeExcess => "Invalid_fee_excess",
            Self::Cancelled => "Cancelled",
            Self::UnexpectedVerificationKeyHash => "Unexpected_verification_key_hash",
            Self::ValidWhilePreconditionUnsatisfied => "Valid_while_precondition_unsatisfied",
        };

        write!(f, "{}", message)
    }
}

/// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/transaction_status.ml#L452>
#[derive(SerdeYojsonEnum, Debug, Clone, PartialEq, Eq)]
pub enum TransactionStatus {
    Applied,
    Failed(Vec<Vec<TransactionFailure>>),
}

impl TransactionStatus {
    pub fn is_applied(&self) -> bool {
        matches!(self, Self::Applied)
    }
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed(_))
    }
}

/// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/with_status.ml#L6>
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct WithStatus<T> {
    pub data: T,
    pub status: TransactionStatus,
}

impl<T> WithStatus<T> {
    pub fn applied(data: T) -> Self {
        Self {
            data,
            status: TransactionStatus::Applied,
        }
    }

    pub fn failed(data: T, failures: Vec<Vec<TransactionFailure>>) -> Self {
        Self {
            data,
            status: TransactionStatus::Failed(failures),
        }
    }

    pub fn map<F, R>(&self, fun: F) -> WithStatus<R>
    where
        F: Fn(&T) -> R,
    {
        WithStatus {
            data: fun(&self.data),
            status: self.status.clone(),
        }
    }

    pub fn into_map<F, R>(self, fun: F) -> WithStatus<R>
    where
        F: Fn(T) -> R,
    {
        WithStatus {
            data: fun(self.data),
            status: self.status,
        }
    }
}

pub trait GenericCommand {
    fn fee(&self) -> Fee;
    fn forget(&self) -> UserCommand;
}

pub trait GenericTransaction: Sized {
    fn is_fee_transfer(&self) -> bool;
    fn is_coinbase(&self) -> bool;
    fn is_command(&self) -> bool;
}

impl<T> GenericCommand for WithStatus<T>
where
    T: GenericCommand,
{
    fn fee(&self) -> Fee {
        self.data.fee()
    }

    fn forget(&self) -> UserCommand {
        self.data.forget()
    }
}

/// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/fee_transfer.ml#L19>
#[derive(Debug, Clone, PartialEq)]
pub struct SingleFeeTransfer {
    pub receiver_pk: CompressedPubKey,
    pub fee: Fee,
    pub fee_token: TokenId,
}

impl SingleFeeTransfer {
    pub fn receiver(&self) -> AccountId {
        AccountId {
            public_key: self.receiver_pk.clone(),
            token_id: self.fee_token.clone(),
        }
    }

    pub fn create(receiver_pk: CompressedPubKey, fee: Fee, fee_token: TokenId) -> Self {
        Self {
            receiver_pk,
            fee,
            fee_token,
        }
    }
}

/// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/fee_transfer.ml#L68>
#[derive(Debug, Clone, PartialEq)]
pub struct FeeTransfer(pub(super) OneOrTwo<SingleFeeTransfer>);

impl std::ops::Deref for FeeTransfer {
    type Target = OneOrTwo<SingleFeeTransfer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FeeTransfer {
    pub fn fee_tokens(&self) -> impl Iterator<Item = &TokenId> {
        self.0.iter().map(|fee_transfer| &fee_transfer.fee_token)
    }

    pub fn receiver_pks(&self) -> impl Iterator<Item = &CompressedPubKey> {
        self.0.iter().map(|fee_transfer| &fee_transfer.receiver_pk)
    }

    pub fn receivers(&self) -> impl Iterator<Item = AccountId> + '_ {
        self.0.iter().map(|fee_transfer| AccountId {
            public_key: fee_transfer.receiver_pk.clone(),
            token_id: fee_transfer.fee_token.clone(),
        })
    }

    /// <https://github.com/MinaProtocol/mina/blob/e5183ca1dde1c085b4c5d37d1d9987e24c294c32/src/lib/mina_base/fee_transfer.ml#L109>
    pub fn fee_excess(&self) -> Result<FeeExcess, String> {
        let one_or_two = self.0.map(|SingleFeeTransfer { fee, fee_token, .. }| {
            (fee_token.clone(), Signed::<Fee>::of_unsigned(*fee).negate())
        });
        FeeExcess::of_one_or_two(one_or_two)
    }

    /// <https://github.com/MinaProtocol/mina/blob/05c2f73d0f6e4f1341286843814ce02dcb3919e0/src/lib/mina_base/fee_transfer.ml#L84>
    pub fn of_singles(singles: OneOrTwo<SingleFeeTransfer>) -> Result<Self, String> {
        match singles {
            OneOrTwo::One(a) => Ok(Self(OneOrTwo::One(a))),
            OneOrTwo::Two((one, two)) => {
                if one.fee_token == two.fee_token {
                    Ok(Self(OneOrTwo::Two((one, two))))
                } else {
                    // Necessary invariant for the transaction snark: we should never have
                    // fee excesses in multiple tokens simultaneously.
                    Err(format!(
                        "Cannot combine single fee transfers with incompatible tokens: {:?} <> {:?}",
                        one, two
                    ))
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CoinbaseFeeTransfer {
    pub receiver_pk: CompressedPubKey,
    pub fee: Fee,
}

impl CoinbaseFeeTransfer {
    pub fn create(receiver_pk: CompressedPubKey, fee: Fee) -> Self {
        Self { receiver_pk, fee }
    }

    pub fn receiver(&self) -> AccountId {
        AccountId {
            public_key: self.receiver_pk.clone(),
            token_id: TokenId::default(),
        }
    }
}

/// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/coinbase.ml#L17>
#[derive(Debug, Clone, PartialEq)]
pub struct Coinbase {
    pub receiver: CompressedPubKey,
    pub amount: Amount,
    pub fee_transfer: Option<CoinbaseFeeTransfer>,
}

impl Coinbase {
    fn is_valid(&self) -> bool {
        match &self.fee_transfer {
            None => true,
            Some(CoinbaseFeeTransfer { fee, .. }) => Amount::of_fee(fee) <= self.amount,
        }
    }

    pub fn create(
        amount: Amount,
        receiver: CompressedPubKey,
        fee_transfer: Option<CoinbaseFeeTransfer>,
    ) -> Result<Coinbase, String> {
        let mut this = Self {
            receiver: receiver.clone(),
            amount,
            fee_transfer,
        };

        if this.is_valid() {
            let adjusted_fee_transfer = this.fee_transfer.as_ref().and_then(|ft| {
                if receiver != ft.receiver_pk {
                    Some(ft.clone())
                } else {
                    None
                }
            });
            this.fee_transfer = adjusted_fee_transfer;
            Ok(this)
        } else {
            Err("Coinbase.create: invalid coinbase".to_string())
        }
    }

    /// <https://github.com/MinaProtocol/mina/blob/f6756507ff7380a691516ce02a3cf7d9d32915ae/src/lib/mina_base/coinbase.ml#L76>
    fn expected_supply_increase(&self) -> Result<Amount, String> {
        let Self {
            amount,
            fee_transfer,
            ..
        } = self;

        match fee_transfer {
            None => Ok(*amount),
            Some(CoinbaseFeeTransfer { fee, .. }) => amount
                .checked_sub(&Amount::of_fee(fee))
                // The substraction result is ignored here
                .map(|_| *amount)
                .ok_or_else(|| "Coinbase underflow".to_string()),
        }
    }

    pub fn fee_excess(&self) -> Result<FeeExcess, String> {
        self.expected_supply_increase().map(|_| FeeExcess::empty())
    }

    /// <https://github.com/MinaProtocol/mina/blob/05c2f73d0f6e4f1341286843814ce02dcb3919e0/src/lib/mina_base/coinbase.ml#L39>
    pub fn receiver(&self) -> AccountId {
        AccountId::new(self.receiver.clone(), TokenId::default())
    }

    /// <https://github.com/MinaProtocol/mina/blob/2ff0292b637684ce0372e7b8e23ec85404dc5091/src/lib/mina_base/coinbase.ml#L51>
    pub fn account_access_statuses(
        &self,
        status: &TransactionStatus,
    ) -> Vec<(AccountId, zkapp_command::AccessedOrNot)> {
        let access_status = match status {
            TransactionStatus::Applied => zkapp_command::AccessedOrNot::Accessed,
            TransactionStatus::Failed(_) => zkapp_command::AccessedOrNot::NotAccessed,
        };

        let mut ids = Vec::with_capacity(2);

        if let Some(fee_transfer) = self.fee_transfer.as_ref() {
            ids.push((fee_transfer.receiver(), access_status.clone()));
        };

        ids.push((self.receiver(), access_status));

        ids
    }

    /// <https://github.com/MinaProtocol/mina/blob/2ff0292b637684ce0372e7b8e23ec85404dc5091/src/lib/mina_base/coinbase.ml#L61>
    pub fn accounts_referenced(&self) -> Vec<AccountId> {
        self.account_access_statuses(&TransactionStatus::Applied)
            .into_iter()
            .map(|(id, _status)| id)
            .collect()
    }
}

/// 0th byte is a tag to distinguish digests from other data
/// 1st byte is length, always 32 for digests
/// bytes 2 to 33 are data, 0-right-padded if length is less than 32
///
#[derive(Clone, PartialEq)]
pub struct Memo(pub [u8; 34]);

impl std::fmt::Debug for Memo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::staged_ledger::hash::OCamlString;

        // Display like OCaml
        // Example: "\000 \014WQ\192&\229C\178\232\171.\176`\153\218\161\209\229\223Gw\143w\135\250\171E\205\241/\227\168"

        f.write_fmt(format_args!("\"{}\"", self.0.to_ocaml_str()))
    }
}

impl std::str::FromStr for Memo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length = std::cmp::min(s.len(), Self::DIGEST_LENGTH) as u8;
        let mut memo: [u8; Self::MEMO_LENGTH] = std::array::from_fn(|i| (i == 0) as u8);
        memo[Self::TAG_INDEX] = Self::BYTES_TAG;
        memo[Self::LENGTH_INDEX] = length;
        let padded = format!("{s:\0<32}");
        memo[2..].copy_from_slice(
            &padded.as_bytes()[..std::cmp::min(padded.len(), Self::DIGEST_LENGTH)],
        );
        Ok(Memo(memo))
    }
}

impl std::fmt::Display for Memo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0[0] != Self::BYTES_TAG {
            return Err(std::fmt::Error);
        }

        let length = self.0[1] as usize;
        let memo_slice = &self.0[2..2 + length];
        let memo_str = String::from_utf8_lossy(memo_slice).to_string();
        let trimmed = memo_str.trim_end_matches('\0').to_string();

        write!(f, "{trimmed}")
    }
}

impl Memo {
    const TAG_INDEX: usize = 0;
    const LENGTH_INDEX: usize = 1;

    const DIGEST_TAG: u8 = 0x00;
    const BYTES_TAG: u8 = 0x01;

    const DIGEST_LENGTH: usize = 32; // Blake2.digest_size_in_bytes
    const DIGEST_LENGTH_BYTE: u8 = Self::DIGEST_LENGTH as u8;

    /// +2 for tag and length bytes
    const MEMO_LENGTH: usize = Self::DIGEST_LENGTH + 2;

    const MAX_INPUT_LENGTH: usize = Self::DIGEST_LENGTH;

    const MAX_DIGESTIBLE_STRING_LENGTH: usize = 1000;

    pub fn to_bits(&self) -> [bool; std::mem::size_of::<Self>() * 8] {
        use crate::proofs::transaction::legacy_input::BitsIterator;

        const NBYTES: usize = 34;
        const NBITS: usize = NBYTES * 8;
        assert_eq!(std::mem::size_of::<Self>(), NBYTES);

        let mut iter = BitsIterator {
            index: 0,
            number: self.0,
        }
        .take(NBITS);
        std::array::from_fn(|_| iter.next().unwrap())
    }

    pub fn hash(&self) -> Fp {
        use poseidon::hash::{hash_with_kimchi, legacy};

        // For some reason we are mixing legacy inputs and "new" hashing
        let mut inputs = legacy::Inputs::new();
        inputs.append_bytes(&self.0);
        hash_with_kimchi(&MINA_ZKAPP_MEMO, &inputs.to_fields())
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    /// <https://github.com/MinaProtocol/mina/blob/3a78f0e0c1343d14e2729c8b00205baa2ec70c93/src/lib/mina_base/signed_command_memo.ml#L151>
    pub fn dummy() -> Self {
        // TODO
        Self([0; 34])
    }

    pub fn empty() -> Self {
        let mut array = [0; 34];
        array[0] = 1;
        Self(array)
    }

    /// Example:
    /// "\000 \014WQ\192&\229C\178\232\171.\176`\153\218\161\209\229\223Gw\143w\135\250\171E\205\241/\227\168"
    #[cfg(test)]
    pub fn from_ocaml_str(s: &str) -> Self {
        use crate::staged_ledger::hash::OCamlString;

        Self(<[u8; 34]>::from_ocaml_str(s))
    }

    pub fn with_number(number: usize) -> Self {
        let s = format!("{:034}", number);
        assert_eq!(s.len(), 34);
        Self(s.into_bytes().try_into().unwrap())
    }

    /// <https://github.com/MinaProtocol/mina/blob/d7dad23d8ea2052f515f5d55d187788fe0701c7f/src/lib/mina_base/signed_command_memo.ml#L103>
    fn create_by_digesting_string_exn(s: &str) -> Self {
        if s.len() > Self::MAX_DIGESTIBLE_STRING_LENGTH {
            panic!("Too_long_digestible_string");
        }

        let mut memo = [0; 34];
        memo[Self::TAG_INDEX] = Self::DIGEST_TAG;
        memo[Self::LENGTH_INDEX] = Self::DIGEST_LENGTH_BYTE;

        use blake2::{
            digest::{Update, VariableOutput},
            Blake2bVar,
        };
        let mut hasher = Blake2bVar::new(32).expect("Invalid Blake2bVar output size");
        hasher.update(s.as_bytes());
        hasher.finalize_variable(&mut memo[2..]).unwrap();

        Self(memo)
    }

    /// <https://github.com/MinaProtocol/mina/blob/d7dad23d8ea2052f515f5d55d187788fe0701c7f/src/lib/mina_base/signed_command_memo.ml#L193>
    pub fn gen() -> Self {
        use rand::distributions::{Alphanumeric, DistString};
        let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 50);

        Self::create_by_digesting_string_exn(&random_string)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UserCommand {
    SignedCommand(Box<signed_command::SignedCommand>),
    ZkAppCommand(Box<zkapp_command::ZkAppCommand>),
}

impl From<&UserCommand> for MinaBaseUserCommandStableV2 {
    fn from(user_command: &UserCommand) -> Self {
        match user_command {
            UserCommand::SignedCommand(signed_command) => {
                MinaBaseUserCommandStableV2::SignedCommand((&(*(signed_command.clone()))).into())
            }
            UserCommand::ZkAppCommand(zkapp_command) => {
                MinaBaseUserCommandStableV2::ZkappCommand((&(*(zkapp_command.clone()))).into())
            }
        }
    }
}

impl TryFrom<&MinaBaseUserCommandStableV2> for UserCommand {
    type Error = InvalidBigInt;

    fn try_from(user_command: &MinaBaseUserCommandStableV2) -> Result<Self, Self::Error> {
        match user_command {
            MinaBaseUserCommandStableV2::SignedCommand(signed_command) => Ok(
                UserCommand::SignedCommand(Box::new(signed_command.try_into()?)),
            ),
            MinaBaseUserCommandStableV2::ZkappCommand(zkapp_command) => Ok(
                UserCommand::ZkAppCommand(Box::new(zkapp_command.try_into()?)),
            ),
        }
    }
}

impl binprot::BinProtWrite for UserCommand {
    fn binprot_write<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        let p2p: MinaBaseUserCommandStableV2 = self.into();
        p2p.binprot_write(w)
    }
}

impl binprot::BinProtRead for UserCommand {
    fn binprot_read<R: std::io::Read + ?Sized>(r: &mut R) -> Result<Self, binprot::Error> {
        let p2p = MinaBaseUserCommandStableV2::binprot_read(r)?;
        match UserCommand::try_from(&p2p) {
            Ok(cmd) => Ok(cmd),
            Err(e) => Err(binprot::Error::CustomError(Box::new(e))),
        }
    }
}

impl UserCommand {
    /// <https://github.com/MinaProtocol/mina/blob/2ff0292b637684ce0372e7b8e23ec85404dc5091/src/lib/mina_base/user_command.ml#L239>
    pub fn account_access_statuses(
        &self,
        status: &TransactionStatus,
    ) -> Vec<(AccountId, AccessedOrNot)> {
        match self {
            UserCommand::SignedCommand(cmd) => cmd.account_access_statuses(status).to_vec(),
            UserCommand::ZkAppCommand(cmd) => cmd.account_access_statuses(status),
        }
    }

    /// <https://github.com/MinaProtocol/mina/blob/2ff0292b637684ce0372e7b8e23ec85404dc5091/src/lib/mina_base/user_command.ml#L247>
    pub fn accounts_referenced(&self) -> Vec<AccountId> {
        self.account_access_statuses(&TransactionStatus::Applied)
            .into_iter()
            .map(|(id, _status)| id)
            .collect()
    }

    pub fn fee_payer(&self) -> AccountId {
        match self {
            UserCommand::SignedCommand(cmd) => cmd.fee_payer(),
            UserCommand::ZkAppCommand(cmd) => cmd.fee_payer(),
        }
    }

    pub fn valid_until(&self) -> Slot {
        match self {
            UserCommand::SignedCommand(cmd) => cmd.valid_until(),
            UserCommand::ZkAppCommand(cmd) => {
                let ZkAppCommand { fee_payer, .. } = &**cmd;
                fee_payer.body.valid_until.unwrap_or_else(Slot::max)
            }
        }
    }

    pub fn applicable_at_nonce(&self) -> Nonce {
        match self {
            UserCommand::SignedCommand(cmd) => cmd.nonce(),
            UserCommand::ZkAppCommand(cmd) => cmd.applicable_at_nonce(),
        }
    }

    pub fn expected_target_nonce(&self) -> Nonce {
        self.applicable_at_nonce().succ()
    }

    /// <https://github.com/MinaProtocol/mina/blob/05c2f73d0f6e4f1341286843814ce02dcb3919e0/src/lib/mina_base/user_command.ml#L192>
    pub fn fee(&self) -> Fee {
        match self {
            UserCommand::SignedCommand(cmd) => cmd.fee(),
            UserCommand::ZkAppCommand(cmd) => cmd.fee(),
        }
    }

    pub fn weight(&self) -> u64 {
        match self {
            UserCommand::SignedCommand(cmd) => cmd.weight(),
            UserCommand::ZkAppCommand(cmd) => cmd.weight(),
        }
    }

    /// Fee per weight unit
    pub fn fee_per_wu(&self) -> FeeRate {
        FeeRate::make_exn(self.fee(), self.weight())
    }

    pub fn fee_token(&self) -> TokenId {
        match self {
            UserCommand::SignedCommand(cmd) => cmd.fee_token(),
            UserCommand::ZkAppCommand(cmd) => cmd.fee_token(),
        }
    }

    pub fn extract_vks(&self) -> Vec<(AccountId, VerificationKeyWire)> {
        match self {
            UserCommand::SignedCommand(_) => vec![],
            UserCommand::ZkAppCommand(zkapp) => zkapp.extract_vks(),
        }
    }

    /// <https://github.com/MinaProtocol/mina/blob/436023ba41c43a50458a551b7ef7a9ae61670b25/src/lib/mina_base/user_command.ml#L339>
    pub fn to_valid_unsafe(self) -> valid::UserCommand {
        match self {
            UserCommand::SignedCommand(cmd) => valid::UserCommand::SignedCommand(cmd),
            UserCommand::ZkAppCommand(cmd) => {
                valid::UserCommand::ZkAppCommand(Box::new(zkapp_command::valid::ZkAppCommand {
                    zkapp_command: *cmd,
                }))
            }
        }
    }

    /// <https://github.com/MinaProtocol/mina/blob/3fe924c80a4d01f418b69f27398f5f93eb652514/src/lib/mina_base/user_command.ml#L162>
    pub fn to_verifiable<F>(
        &self,
        status: &TransactionStatus,
        find_vk: F,
    ) -> Result<verifiable::UserCommand, String>
    where
        F: Fn(Fp, &AccountId) -> Result<VerificationKeyWire, String>,
    {
        use verifiable::UserCommand::{SignedCommand, ZkAppCommand};
        match self {
            UserCommand::SignedCommand(cmd) => Ok(SignedCommand(cmd.clone())),
            UserCommand::ZkAppCommand(zkapp) => Ok(ZkAppCommand(Box::new(
                zkapp_command::verifiable::create(zkapp, status.is_failed(), find_vk)?,
            ))),
        }
    }

    pub fn load_vks_from_ledger(
        account_ids: HashSet<AccountId>,
        ledger: &crate::Mask,
    ) -> HashMap<AccountId, VerificationKeyWire> {
        let ids: Vec<_> = account_ids.iter().cloned().collect();
        let locations: Vec<_> = ledger
            .location_of_account_batch(&ids)
            .into_iter()
            .filter_map(|(_, addr)| addr)
            .collect();
        ledger
            .get_batch(&locations)
            .into_iter()
            .filter_map(|(_, account)| {
                let account = account.unwrap();
                let zkapp = account.zkapp.as_ref()?;
                let vk = zkapp.verification_key.clone()?;
                Some((account.id(), vk))
            })
            .collect()
    }

    pub fn load_vks_from_ledger_accounts(
        accounts: &BTreeMap<AccountId, Account>,
    ) -> HashMap<AccountId, VerificationKeyWire> {
        accounts
            .iter()
            .filter_map(|(_, account)| {
                let zkapp = account.zkapp.as_ref()?;
                let vk = zkapp.verification_key.clone()?;
                Some((account.id(), vk))
            })
            .collect()
    }

    pub fn to_all_verifiable<S, F>(
        ts: Vec<MaybeWithStatus<UserCommand>>,
        load_vk_cache: F,
    ) -> Result<Vec<MaybeWithStatus<verifiable::UserCommand>>, String>
    where
        S: zkapp_command::ToVerifiableStrategy,
        F: Fn(HashSet<AccountId>) -> S::Cache,
    {
        let accounts_referenced: HashSet<AccountId> = ts
            .iter()
            .flat_map(|cmd| match cmd.cmd() {
                UserCommand::SignedCommand(_) => Vec::new(),
                UserCommand::ZkAppCommand(cmd) => cmd.accounts_referenced(),
            })
            .collect();
        let mut vk_cache = load_vk_cache(accounts_referenced);

        ts.into_iter()
            .map(|cmd| {
                let is_failed = cmd.is_failed();
                let MaybeWithStatus { cmd, status } = cmd;
                match cmd {
                    UserCommand::SignedCommand(c) => Ok(MaybeWithStatus {
                        cmd: verifiable::UserCommand::SignedCommand(c),
                        status,
                    }),
                    UserCommand::ZkAppCommand(c) => {
                        let zkapp_verifiable = S::create_all(&c, is_failed, &mut vk_cache)?;
                        Ok(MaybeWithStatus {
                            cmd: verifiable::UserCommand::ZkAppCommand(Box::new(zkapp_verifiable)),
                            status,
                        })
                    }
                }
            })
            .collect()
    }

    fn has_insufficient_fee(&self) -> bool {
        /// `minimum_user_command_fee`
        const MINIMUM_USER_COMMAND_FEE: Fee = Fee::from_u64(1000000);
        self.fee() < MINIMUM_USER_COMMAND_FEE
    }

    fn has_zero_vesting_period(&self) -> bool {
        match self {
            UserCommand::SignedCommand(_cmd) => false,
            UserCommand::ZkAppCommand(cmd) => cmd.has_zero_vesting_period(),
        }
    }

    fn is_incompatible_version(&self) -> bool {
        match self {
            UserCommand::SignedCommand(_cmd) => false,
            UserCommand::ZkAppCommand(cmd) => cmd.is_incompatible_version(),
        }
    }

    fn is_disabled(&self) -> bool {
        match self {
            UserCommand::SignedCommand(_cmd) => false,
            UserCommand::ZkAppCommand(_cmd) => false, // Mina_compile_config.zkapps_disabled
        }
    }

    fn valid_size(&self) -> Result<(), String> {
        match self {
            UserCommand::SignedCommand(_cmd) => Ok(()),
            UserCommand::ZkAppCommand(cmd) => cmd.valid_size(),
        }
    }

    pub fn check_well_formedness(&self) -> Result<(), Vec<WellFormednessError>> {
        let mut errors: Vec<_> = [
            (
                Self::has_insufficient_fee as fn(_) -> _,
                WellFormednessError::InsufficientFee,
            ),
            (
                Self::has_zero_vesting_period,
                WellFormednessError::ZeroVestingPeriod,
            ),
            (
                Self::is_incompatible_version,
                WellFormednessError::IncompatibleVersion,
            ),
            (
                Self::is_disabled,
                WellFormednessError::TransactionTypeDisabled,
            ),
        ]
        .iter()
        .filter_map(|(fun, e)| if fun(self) { Some(e.clone()) } else { None })
        .collect();

        if let Err(e) = self.valid_size() {
            errors.push(WellFormednessError::ZkappTooBig(e));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, thiserror::Error)]
pub enum WellFormednessError {
    #[error("Insufficient Fee")]
    InsufficientFee,
    #[error("Zero vesting period")]
    ZeroVestingPeriod,
    #[error("Zkapp too big: {0}")]
    ZkappTooBig(String),
    #[error("Transaction type disabled")]
    TransactionTypeDisabled,
    #[error("Incompatible version")]
    IncompatibleVersion,
}

impl GenericCommand for UserCommand {
    fn fee(&self) -> Fee {
        match self {
            UserCommand::SignedCommand(cmd) => cmd.fee(),
            UserCommand::ZkAppCommand(cmd) => cmd.fee(),
        }
    }

    fn forget(&self) -> UserCommand {
        self.clone()
    }
}

impl GenericTransaction for Transaction {
    fn is_fee_transfer(&self) -> bool {
        matches!(self, Transaction::FeeTransfer(_))
    }
    fn is_coinbase(&self) -> bool {
        matches!(self, Transaction::Coinbase(_))
    }
    fn is_command(&self) -> bool {
        matches!(self, Transaction::Command(_))
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum Transaction {
    Command(UserCommand),
    FeeTransfer(FeeTransfer),
    Coinbase(Coinbase),
}

impl Transaction {
    pub fn is_zkapp(&self) -> bool {
        matches!(self, Self::Command(UserCommand::ZkAppCommand(_)))
    }

    pub fn fee_excess(&self) -> Result<FeeExcess, String> {
        use Transaction::*;
        use UserCommand::*;

        match self {
            Command(SignedCommand(cmd)) => Ok(cmd.fee_excess()),
            Command(ZkAppCommand(cmd)) => Ok(cmd.fee_excess()),
            FeeTransfer(ft) => ft.fee_excess(),
            Coinbase(cb) => cb.fee_excess(),
        }
    }

    /// <https://github.com/MinaProtocol/mina/blob/436023ba41c43a50458a551b7ef7a9ae61670b25/src/lib/transaction/transaction.ml#L98>
    pub fn public_keys(&self) -> Vec<CompressedPubKey> {
        use Transaction::*;
        use UserCommand::*;

        let to_pks = |ids: Vec<AccountId>| ids.into_iter().map(|id| id.public_key).collect();

        match self {
            Command(SignedCommand(cmd)) => to_pks(cmd.accounts_referenced()),
            Command(ZkAppCommand(cmd)) => to_pks(cmd.accounts_referenced()),
            FeeTransfer(ft) => ft.receiver_pks().cloned().collect(),
            Coinbase(cb) => to_pks(cb.accounts_referenced()),
        }
    }

    /// <https://github.com/MinaProtocol/mina/blob/436023ba41c43a50458a551b7ef7a9ae61670b25/src/lib/transaction/transaction.ml#L112>
    pub fn account_access_statuses(
        &self,
        status: &TransactionStatus,
    ) -> Vec<(AccountId, zkapp_command::AccessedOrNot)> {
        use Transaction::*;
        use UserCommand::*;

        match self {
            Command(SignedCommand(cmd)) => cmd.account_access_statuses(status).to_vec(),
            Command(ZkAppCommand(cmd)) => cmd.account_access_statuses(status),
            FeeTransfer(ft) => ft
                .receivers()
                .map(|account_id| (account_id, AccessedOrNot::Accessed))
                .collect(),
            Coinbase(cb) => cb.account_access_statuses(status),
        }
    }

    /// <https://github.com/MinaProtocol/mina/blob/436023ba41c43a50458a551b7ef7a9ae61670b25/src/lib/transaction/transaction.ml#L125>
    pub fn accounts_referenced(&self) -> Vec<AccountId> {
        self.account_access_statuses(&TransactionStatus::Applied)
            .into_iter()
            .map(|(id, _status)| id)
            .collect()
    }
}

impl From<&Transaction> for MinaTransactionTransactionStableV2 {
    fn from(value: &Transaction) -> Self {
        match value {
            Transaction::Command(v) => Self::Command(Box::new(v.into())),
            Transaction::FeeTransfer(v) => Self::FeeTransfer(v.into()),
            Transaction::Coinbase(v) => Self::Coinbase(v.into()),
        }
    }
}

#[cfg(any(test, feature = "fuzzing"))]
pub mod for_tests {
    use mina_signer::Keypair;
    use rand::Rng;

    use crate::{
        gen_keypair, scan_state::parallel_scan::ceil_log2, AuthRequired, Mask, Permissions,
        VerificationKey, ZkAppAccount, TXN_VERSION_CURRENT,
    };

    use super::*;

    const MIN_INIT_BALANCE: u64 = 8000000000;
    const MAX_INIT_BALANCE: u64 = 8000000000000;
    const NUM_ACCOUNTS: u64 = 10;
    const NUM_TRANSACTIONS: u64 = 10;
    const DEPTH: u64 = ceil_log2(NUM_ACCOUNTS + NUM_TRANSACTIONS);

    /// Use this for tests only
    /// Hashmaps are not deterministic
    #[derive(Debug, PartialEq, Eq)]
    pub struct HashableKeypair(pub Keypair);

    impl std::hash::Hash for HashableKeypair {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            let compressed = self.0.public.into_compressed();
            HashableCompressedPubKey(compressed).hash(state);
        }
    }

    /// Use this for tests only
    /// Hashmaps are not deterministic
    #[derive(Clone, Debug, Eq, derive_more::From)]
    pub struct HashableCompressedPubKey(pub CompressedPubKey);

    impl PartialEq for HashableCompressedPubKey {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl std::hash::Hash for HashableCompressedPubKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.x.hash(state);
            self.0.is_odd.hash(state);
        }
    }

    impl PartialOrd for HashableCompressedPubKey {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self.0.x.partial_cmp(&other.0.x) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            };
            self.0.is_odd.partial_cmp(&other.0.is_odd)
        }
    }

    /// <https://github.com/MinaProtocol/mina/blob/3753a8593cc1577bcf4da16620daf9946d88e8e5/src/lib/transaction_logic/mina_transaction_logic.ml#L2194>
    #[derive(Debug)]
    pub struct InitLedger(pub Vec<(Keypair, u64)>);

    /// <https://github.com/MinaProtocol/mina/blob/3753a8593cc1577bcf4da16620daf9946d88e8e5/src/lib/transaction_logic/mina_transaction_logic.ml#L2230>
    #[derive(Debug)]
    pub struct TransactionSpec {
        pub fee: Fee,
        pub sender: (Keypair, Nonce),
        pub receiver: CompressedPubKey,
        pub amount: Amount,
    }

    /// <https://github.com/MinaProtocol/mina/blob/3753a8593cc1577bcf4da16620daf9946d88e8e5/src/lib/transaction_logic/mina_transaction_logic.ml#L2283>
    #[derive(Debug)]
    pub struct TestSpec {
        pub init_ledger: InitLedger,
        pub specs: Vec<TransactionSpec>,
    }

    impl InitLedger {
        pub fn init(&self, zkapp: Option<bool>, ledger: &mut impl LedgerIntf) {
            let zkapp = zkapp.unwrap_or(true);

            self.0.iter().for_each(|(kp, amount)| {
                let (_tag, mut account, loc) = ledger
                    .get_or_create(&AccountId::new(
                        kp.public.into_compressed(),
                        TokenId::default(),
                    ))
                    .unwrap();

                use AuthRequired::Either;
                let permissions = Permissions {
                    edit_state: Either,
                    access: AuthRequired::None,
                    send: Either,
                    receive: AuthRequired::None,
                    set_delegate: Either,
                    set_permissions: Either,
                    set_verification_key: crate::SetVerificationKey {
                        auth: Either,
                        txn_version: TXN_VERSION_CURRENT,
                    },
                    set_zkapp_uri: Either,
                    edit_action_state: Either,
                    set_token_symbol: Either,
                    increment_nonce: Either,
                    set_voting_for: Either,
                    set_timing: Either,
                };

                let zkapp = if zkapp {
                    let zkapp = ZkAppAccount {
                        verification_key: Some(VerificationKeyWire::new(
                            crate::dummy::trivial_verification_key(),
                        )),
                        ..Default::default()
                    };

                    Some(zkapp.into())
                } else {
                    None
                };

                account.balance = Balance::from_u64(*amount);
                account.permissions = permissions;
                account.zkapp = zkapp;

                ledger.set(&loc, account);
            });
        }

        pub fn gen() -> Self {
            let mut rng = rand::thread_rng();

            let mut tbl = HashSet::with_capacity(256);

            let init = (0..NUM_ACCOUNTS)
                .map(|_| {
                    let kp = loop {
                        let keypair = gen_keypair();
                        let compressed = keypair.public.into_compressed();
                        if !tbl.contains(&HashableCompressedPubKey(compressed)) {
                            break keypair;
                        }
                    };

                    let amount = rng.gen_range(MIN_INIT_BALANCE..MAX_INIT_BALANCE);
                    tbl.insert(HashableCompressedPubKey(kp.public.into_compressed()));
                    (kp, amount)
                })
                .collect();

            Self(init)
        }
    }

    impl TransactionSpec {
        pub fn gen(init_ledger: &InitLedger, nonces: &mut HashMap<HashableKeypair, Nonce>) -> Self {
            let mut rng = rand::thread_rng();

            let pk = |(kp, _): (Keypair, u64)| kp.public.into_compressed();

            let receiver_is_new: bool = rng.gen();

            let mut gen_index = || rng.gen_range(0..init_ledger.0.len().checked_sub(1).unwrap());

            let receiver_index = if receiver_is_new {
                None
            } else {
                Some(gen_index())
            };

            let receiver = match receiver_index {
                None => gen_keypair().public.into_compressed(),
                Some(i) => pk(init_ledger.0[i].clone()),
            };

            let sender = {
                let i = match receiver_index {
                    None => gen_index(),
                    Some(j) => loop {
                        let i = gen_index();
                        if i != j {
                            break i;
                        }
                    },
                };
                init_ledger.0[i].0.clone()
            };

            let nonce = nonces
                .get(&HashableKeypair(sender.clone()))
                .cloned()
                .unwrap();

            let amount = Amount::from_u64(rng.gen_range(1_000_000..100_000_000));
            let fee = Fee::from_u64(rng.gen_range(1_000_000..100_000_000));

            let old = nonces.get_mut(&HashableKeypair(sender.clone())).unwrap();
            *old = old.incr();

            Self {
                fee,
                sender: (sender, nonce),
                receiver,
                amount,
            }
        }
    }

    impl TestSpec {
        fn mk_gen(num_transactions: Option<u64>) -> TestSpec {
            let num_transactions = num_transactions.unwrap_or(NUM_TRANSACTIONS);

            let init_ledger = InitLedger::gen();

            let mut map = init_ledger
                .0
                .iter()
                .map(|(kp, _)| (HashableKeypair(kp.clone()), Nonce::zero()))
                .collect();

            let specs = (0..num_transactions)
                .map(|_| TransactionSpec::gen(&init_ledger, &mut map))
                .collect();

            Self { init_ledger, specs }
        }

        pub fn gen() -> Self {
            Self::mk_gen(Some(NUM_TRANSACTIONS))
        }
    }

    #[derive(Debug)]
    pub struct UpdateStatesSpec {
        pub fee: Fee,
        pub sender: (Keypair, Nonce),
        pub fee_payer: Option<(Keypair, Nonce)>,
        pub receivers: Vec<(CompressedPubKey, Amount)>,
        pub amount: Amount,
        pub zkapp_account_keypairs: Vec<Keypair>,
        pub memo: Memo,
        pub new_zkapp_account: bool,
        pub snapp_update: zkapp_command::Update,
        // Authorization for the update being performed
        pub current_auth: AuthRequired,
        pub actions: Vec<Vec<Fp>>,
        pub events: Vec<Vec<Fp>>,
        pub call_data: Fp,
        pub preconditions: Option<zkapp_command::Preconditions>,
    }

    pub fn trivial_zkapp_account(
        permissions: Option<Permissions<AuthRequired>>,
        vk: VerificationKey,
        pk: CompressedPubKey,
    ) -> Account {
        let id = AccountId::new(pk, TokenId::default());
        let mut account = Account::create_with(id, Balance::from_u64(1_000_000_000_000_000));
        account.permissions = permissions.unwrap_or_else(Permissions::user_default);
        account.zkapp = Some(
            ZkAppAccount {
                verification_key: Some(VerificationKeyWire::new(vk)),
                ..Default::default()
            }
            .into(),
        );
        account
    }

    pub fn create_trivial_zkapp_account(
        permissions: Option<Permissions<AuthRequired>>,
        vk: VerificationKey,
        ledger: &mut Mask,
        pk: CompressedPubKey,
    ) {
        let id = AccountId::new(pk.clone(), TokenId::default());
        let account = trivial_zkapp_account(permissions, vk, pk);
        assert!(BaseLedger::location_of_account(ledger, &id).is_none());
        ledger.get_or_create_account(id, account).unwrap();
    }
}

