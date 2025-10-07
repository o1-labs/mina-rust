use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Display,
};

use ark_ff::Zero;
use itertools::{FoldWhile, Itertools};
use mina_core::constants::ConstraintConstants;
use mina_hasher::Fp;
use mina_macros::SerdeYojsonEnum;
use mina_p2p_messages::{
    bigint::InvalidBigInt,
    binprot,
    v2::{MinaBaseUserCommandStableV2, MinaTransactionTransactionStableV2},
};
use mina_signer::CompressedPubKey;
use poseidon::hash::{
    hash_with_kimchi,
    params::{CODA_RECEIPT_UC, MINA_ZKAPP_MEMO},
    Inputs,
};

use crate::{
    proofs::witness::Witness,
    scan_state::transaction_logic::{
        transaction_applied::{CommandApplied, Varying},
        transaction_partially_applied::FullyApplied,
        zkapp_command::MaybeWithStatus,
    },
    sparse_ledger::{LedgerIntf, SparseLedger},
    zkapps,
    zkapps::non_snark::{LedgerNonSnark, ZkappNonSnark},
    Account, AccountId, AccountIdOrderable, AppendToInputs, BaseLedger, ControlTag,
    ReceiptChainHash, Timing, TokenId, VerificationKeyWire,
};

use self::{
    local_state::{CallStack, LocalStateEnv, StackFrame},
    protocol_state::{GlobalState, ProtocolStateView},
    signed_command::{SignedCommand, SignedCommandPayload},
    transaction_applied::{
        signed_command_applied::{self, SignedCommandApplied},
        TransactionApplied, ZkappCommandApplied,
    },
    transaction_union_payload::TransactionUnionPayload,
    zkapp_command::{AccessedOrNot, AccountUpdate, WithHash, ZkAppCommand},
};

use super::{
    currency::{Amount, Balance, Fee, Index, Length, Magnitude, Nonce, Signed, Slot},
    fee_excess::FeeExcess,
    fee_rate::FeeRate,
    scan_state::transaction_snark::OneOrTwo,
};
use crate::zkapps::zkapp_logic::ZkAppCommandElt;

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

pub mod valid;

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

pub mod signed_command;

pub mod zkapp_command;
pub mod zkapp_statement;

pub mod verifiable;

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

pub mod transaction_applied;
pub mod transaction_witness;
pub mod protocol_state {
    use mina_p2p_messages::v2::{self, MinaStateProtocolStateValueStableV2};

    use crate::proofs::field::FieldWitness;

    use super::*;

    #[derive(Debug, Clone)]
    pub struct EpochLedger<F: FieldWitness> {
        pub hash: F,
        pub total_currency: Amount,
    }

    #[derive(Debug, Clone)]
    pub struct EpochData<F: FieldWitness> {
        pub ledger: EpochLedger<F>,
        pub seed: F,
        pub start_checkpoint: F,
        pub lock_checkpoint: F,
        pub epoch_length: Length,
    }

    #[derive(Debug, Clone)]
    pub struct ProtocolStateView {
        pub snarked_ledger_hash: Fp,
        pub blockchain_length: Length,
        pub min_window_density: Length,
        pub total_currency: Amount,
        pub global_slot_since_genesis: Slot,
        pub staking_epoch_data: EpochData<Fp>,
        pub next_epoch_data: EpochData<Fp>,
    }

    /// <https://github.com/MinaProtocol/mina/blob/bfd1009abdbee78979ff0343cc73a3480e862f58/src/lib/mina_state/protocol_state.ml#L180>
    pub fn protocol_state_view(
        state: &MinaStateProtocolStateValueStableV2,
    ) -> Result<ProtocolStateView, InvalidBigInt> {
        let MinaStateProtocolStateValueStableV2 {
            previous_state_hash: _,
            body,
        } = state;

        protocol_state_body_view(body)
    }

    pub fn protocol_state_body_view(
        body: &v2::MinaStateProtocolStateBodyValueStableV2,
    ) -> Result<ProtocolStateView, InvalidBigInt> {
        let cs = &body.consensus_state;
        let sed = &cs.staking_epoch_data;
        let ned = &cs.next_epoch_data;

        Ok(ProtocolStateView {
            // <https://github.com/MinaProtocol/mina/blob/436023ba41c43a50458a551b7ef7a9ae61670b25/src/lib/mina_state/blockchain_state.ml#L58>
            //
            snarked_ledger_hash: body
                .blockchain_state
                .ledger_proof_statement
                .target
                .first_pass_ledger
                .to_field()?,
            blockchain_length: Length(cs.blockchain_length.as_u32()),
            min_window_density: Length(cs.min_window_density.as_u32()),
            total_currency: Amount(cs.total_currency.as_u64()),
            global_slot_since_genesis: (&cs.global_slot_since_genesis).into(),
            staking_epoch_data: EpochData {
                ledger: EpochLedger {
                    hash: sed.ledger.hash.to_field()?,
                    total_currency: Amount(sed.ledger.total_currency.as_u64()),
                },
                seed: sed.seed.to_field()?,
                start_checkpoint: sed.start_checkpoint.to_field()?,
                lock_checkpoint: sed.lock_checkpoint.to_field()?,
                epoch_length: Length(sed.epoch_length.as_u32()),
            },
            next_epoch_data: EpochData {
                ledger: EpochLedger {
                    hash: ned.ledger.hash.to_field()?,
                    total_currency: Amount(ned.ledger.total_currency.as_u64()),
                },
                seed: ned.seed.to_field()?,
                start_checkpoint: ned.start_checkpoint.to_field()?,
                lock_checkpoint: ned.lock_checkpoint.to_field()?,
                epoch_length: Length(ned.epoch_length.as_u32()),
            },
        })
    }

    pub type GlobalState<L> = GlobalStateSkeleton<L, Signed<Amount>, Slot>;

    #[derive(Debug, Clone)]
    pub struct GlobalStateSkeleton<L, SignedAmount, Slot> {
        pub first_pass_ledger: L,
        pub second_pass_ledger: L,
        pub fee_excess: SignedAmount,
        pub supply_increase: SignedAmount,
        pub protocol_state: ProtocolStateView,
        /// Slot of block when the transaction is applied.
        /// NOTE: This is at least 1 slot after the protocol_state's view,
        /// which is for the *previous* slot.
        pub block_global_slot: Slot,
    }

    impl<L: LedgerIntf + Clone> GlobalState<L> {
        pub fn first_pass_ledger(&self) -> L {
            self.first_pass_ledger.create_masked()
        }

        #[must_use]
        pub fn set_first_pass_ledger(&self, should_update: bool, ledger: L) -> Self {
            let mut this = self.clone();
            if should_update {
                this.first_pass_ledger.apply_mask(ledger);
            }
            this
        }

        pub fn second_pass_ledger(&self) -> L {
            self.second_pass_ledger.create_masked()
        }

        #[must_use]
        pub fn set_second_pass_ledger(&self, should_update: bool, ledger: L) -> Self {
            let mut this = self.clone();
            if should_update {
                this.second_pass_ledger.apply_mask(ledger);
            }
            this
        }

        pub fn fee_excess(&self) -> Signed<Amount> {
            self.fee_excess
        }

        #[must_use]
        pub fn set_fee_excess(&self, fee_excess: Signed<Amount>) -> Self {
            let mut this = self.clone();
            this.fee_excess = fee_excess;
            this
        }

        pub fn supply_increase(&self) -> Signed<Amount> {
            self.supply_increase
        }

        #[must_use]
        pub fn set_supply_increase(&self, supply_increase: Signed<Amount>) -> Self {
            let mut this = self.clone();
            this.supply_increase = supply_increase;
            this
        }

        pub fn block_global_slot(&self) -> Slot {
            self.block_global_slot
        }
    }
}

pub mod local_state {
    use std::{cell::RefCell, rc::Rc};

    use poseidon::hash::params::MINA_ACCOUNT_UPDATE_STACK_FRAME;

    use crate::{
        proofs::{
            field::{field, Boolean, ToBoolean},
            numbers::nat::CheckedNat,
            to_field_elements::ToFieldElements,
        },
        zkapps::interfaces::{
            CallStackInterface, IndexInterface, SignedAmountInterface, StackFrameInterface,
        },
        ToInputs,
    };

    use super::{zkapp_command::CallForest, *};

    #[derive(Debug, Clone)]
    pub struct StackFrame {
        pub caller: TokenId,
        pub caller_caller: TokenId,
        pub calls: CallForest<AccountUpdate>, // TODO
    }

    // <https://github.com/MinaProtocol/mina/blob/78535ae3a73e0e90c5f66155365a934a15535779/src/lib/transaction_snark/transaction_snark.ml#L1081>
    #[derive(Debug, Clone)]
    pub struct StackFrameCheckedFrame {
        pub caller: TokenId,
        pub caller_caller: TokenId,
        pub calls: WithHash<CallForest<AccountUpdate>>,
        /// Hack until we have proper cvar
        pub is_default: bool,
    }

    impl ToFieldElements<Fp> for StackFrameCheckedFrame {
        fn to_field_elements(&self, fields: &mut Vec<Fp>) {
            let Self {
                caller,
                caller_caller,
                calls,
                is_default: _,
            } = self;

            // calls.hash().to_field_elements(fields);
            calls.hash.to_field_elements(fields);
            caller_caller.to_field_elements(fields);
            caller.to_field_elements(fields);
        }
    }

    enum LazyValueInner<T, D> {
        Value(T),
        Fun(Box<dyn FnOnce(&mut D) -> T>),
        None,
    }

    impl<T, D> Default for LazyValueInner<T, D> {
        fn default() -> Self {
            Self::None
        }
    }

    pub struct LazyValue<T, D> {
        value: Rc<RefCell<LazyValueInner<T, D>>>,
    }

    impl<T, D> Clone for LazyValue<T, D> {
        fn clone(&self) -> Self {
            Self {
                value: Rc::clone(&self.value),
            }
        }
    }

    impl<T: std::fmt::Debug, D> std::fmt::Debug for LazyValue<T, D> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let v = self.try_get();
            f.debug_struct("LazyValue").field("value", &v).finish()
        }
    }

    impl<T, D> LazyValue<T, D> {
        pub fn make<F>(fun: F) -> Self
        where
            F: FnOnce(&mut D) -> T + 'static,
        {
            Self {
                value: Rc::new(RefCell::new(LazyValueInner::Fun(Box::new(fun)))),
            }
        }

        fn get_impl(&self) -> std::cell::Ref<'_, T> {
            use std::cell::Ref;

            let inner = self.value.borrow();
            Ref::map(inner, |inner| {
                let LazyValueInner::Value(value) = inner else {
                    panic!("invalid state");
                };
                value
            })
        }

        /// Returns the value when it already has been "computed"
        pub fn try_get(&self) -> Option<std::cell::Ref<'_, T>> {
            let inner = self.value.borrow();

            match &*inner {
                LazyValueInner::Value(_) => {}
                LazyValueInner::Fun(_) => return None,
                LazyValueInner::None => panic!("invalid state"),
            }

            Some(self.get_impl())
        }

        pub fn get(&self, data: &mut D) -> std::cell::Ref<'_, T> {
            let v = self.value.borrow();

            if let LazyValueInner::Fun(_) = &*v {
                std::mem::drop(v);

                let LazyValueInner::Fun(fun) = self.value.take() else {
                    panic!("invalid state");
                };

                let data = fun(data);
                self.value.replace(LazyValueInner::Value(data));
            };

            self.get_impl()
        }
    }

    #[derive(Clone, Debug)]
    pub struct WithLazyHash<T> {
        pub data: T,
        hash: LazyValue<Fp, Witness<Fp>>,
    }

    impl<T> WithLazyHash<T> {
        pub fn new<F>(data: T, fun: F) -> Self
        where
            F: FnOnce(&mut Witness<Fp>) -> Fp + 'static,
        {
            Self {
                data,
                hash: LazyValue::make(fun),
            }
        }

        pub fn hash(&self, w: &mut Witness<Fp>) -> Fp {
            *self.hash.get(w)
        }
    }

    impl<T> std::ops::Deref for WithLazyHash<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl<T> ToFieldElements<Fp> for WithLazyHash<T> {
        fn to_field_elements(&self, fields: &mut Vec<Fp>) {
            let hash = self.hash.try_get().expect("hash hasn't been computed yet");
            hash.to_field_elements(fields)
        }
    }

    // <https://github.com/MinaProtocol/mina/blob/78535ae3a73e0e90c5f66155365a934a15535779/src/lib/transaction_snark/transaction_snark.ml#L1083>
    pub type StackFrameChecked = WithLazyHash<StackFrameCheckedFrame>;

    impl Default for StackFrame {
        fn default() -> Self {
            StackFrame {
                caller: TokenId::default(),
                caller_caller: TokenId::default(),
                calls: CallForest::new(),
            }
        }
    }

    impl StackFrame {
        pub fn empty() -> Self {
            Self {
                caller: TokenId::default(),
                caller_caller: TokenId::default(),
                calls: CallForest(Vec::new()),
            }
        }

        /// TODO: this needs to be tested
        ///
        /// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/stack_frame.ml#L90>
        pub fn hash(&self) -> Fp {
            let mut inputs = Inputs::new();

            inputs.append_field(self.caller.0);
            inputs.append_field(self.caller_caller.0);

            self.calls.ensure_hashed();
            let field = match self.calls.0.first() {
                None => Fp::zero(),
                Some(calls) => calls.stack_hash.get().unwrap(), // Never fail, we called `ensure_hashed`
            };
            inputs.append_field(field);

            hash_with_kimchi(&MINA_ACCOUNT_UPDATE_STACK_FRAME, &inputs.to_fields())
        }

        pub fn digest(&self) -> Fp {
            self.hash()
        }

        pub fn unhash(&self, _h: Fp, w: &mut Witness<Fp>) -> StackFrameChecked {
            let v = self.exists_elt(w);
            v.hash(w);
            v
        }

        pub fn exists_elt(&self, w: &mut Witness<Fp>) -> StackFrameChecked {
            // We decompose this way because of OCaml evaluation order
            let calls = WithHash {
                data: self.calls.clone(),
                hash: w.exists(self.calls.hash()),
            };
            let caller_caller = w.exists(self.caller_caller.clone());
            let caller = w.exists(self.caller.clone());

            let frame = StackFrameCheckedFrame {
                caller,
                caller_caller,
                calls,
                is_default: false,
            };

            StackFrameChecked::of_frame(frame)
        }
    }

    impl StackFrameCheckedFrame {
        pub fn hash(&self, w: &mut Witness<Fp>) -> Fp {
            let mut inputs = Inputs::new();

            inputs.append(&self.caller);
            inputs.append(&self.caller_caller.0);
            inputs.append(&self.calls.hash);

            let fields = inputs.to_fields();

            if self.is_default {
                use crate::proofs::transaction::transaction_snark::checked_hash3;
                checked_hash3(&MINA_ACCOUNT_UPDATE_STACK_FRAME, &fields, w)
            } else {
                use crate::proofs::transaction::transaction_snark::checked_hash;
                checked_hash(&MINA_ACCOUNT_UPDATE_STACK_FRAME, &fields, w)
            }
        }
    }

    impl StackFrameChecked {
        pub fn of_frame(frame: StackFrameCheckedFrame) -> Self {
            // TODO: Don't clone here
            let frame2 = frame.clone();
            let hash = LazyValue::make(move |w: &mut Witness<Fp>| frame2.hash(w));

            Self { data: frame, hash }
        }
    }

    #[derive(Debug, Clone)]
    pub struct CallStack(pub Vec<StackFrame>);

    impl Default for CallStack {
        fn default() -> Self {
            Self::new()
        }
    }

    impl CallStack {
        pub fn new() -> Self {
            CallStack(Vec::new())
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        pub fn iter(&self) -> impl Iterator<Item = &StackFrame> {
            self.0.iter().rev()
        }

        pub fn push(&self, stack_frame: &StackFrame) -> Self {
            let mut ret = self.0.clone();
            ret.push(stack_frame.clone());
            Self(ret)
        }

        pub fn pop(&self) -> Option<(StackFrame, CallStack)> {
            let mut ret = self.0.clone();
            ret.pop().map(|frame| (frame, Self(ret)))
        }

        pub fn pop_exn(&self) -> (StackFrame, CallStack) {
            let mut ret = self.0.clone();
            if let Some(frame) = ret.pop() {
                (frame, Self(ret))
            } else {
                panic!()
            }
        }
    }

    // NOTE: It looks like there are different instances of the polymorphic LocalEnv type
    // One with concrete types for the stack frame, call stack, and ledger. Created from the Env
    // And the other with their hashes. To differentiate them I renamed the first LocalStateEnv
    // Maybe a better solution is to keep the LocalState name and put it under a different module
    // pub type LocalStateEnv<L> = LocalStateSkeleton<
    //     L,                            // ledger
    //     StackFrame,                   // stack_frame
    //     CallStack,                    // call_stack
    //     ReceiptChainHash,             // commitments
    //     Signed<Amount>,               // excess & supply_increase
    //     Vec<Vec<TransactionFailure>>, // failure_status_tbl
    //     bool,                         // success & will_succeed
    //     Index,                        // account_update_index
    // >;

    pub type LocalStateEnv<L> = crate::zkapps::zkapp_logic::LocalState<ZkappNonSnark<L>>;

    // TODO: Dedub this with `crate::zkapps::zkapp_logic::LocalState`
    #[derive(Debug, Clone)]
    pub struct LocalStateSkeleton<
        L: LedgerIntf + Clone,
        StackFrame: StackFrameInterface,
        CallStack: CallStackInterface,
        TC,
        SignedAmount: SignedAmountInterface,
        FailuresTable,
        Bool,
        Index: IndexInterface,
    > {
        pub stack_frame: StackFrame,
        pub call_stack: CallStack,
        pub transaction_commitment: TC,
        pub full_transaction_commitment: TC,
        pub excess: SignedAmount,
        pub supply_increase: SignedAmount,
        pub ledger: L,
        pub success: Bool,
        pub account_update_index: Index,
        // TODO: optimize by reversing the insertion order
        pub failure_status_tbl: FailuresTable,
        pub will_succeed: Bool,
    }

    // impl<L> LocalStateEnv<L>
    // where
    //     L: LedgerNonSnark,
    // {
    //     pub fn add_new_failure_status_bucket(&self) -> Self {
    //         let mut failure_status_tbl = self.failure_status_tbl.clone();
    //         failure_status_tbl.insert(0, Vec::new());
    //         Self {
    //             failure_status_tbl,
    //             ..self.clone()
    //         }
    //     }

    //     pub fn add_check(&self, failure: TransactionFailure, b: bool) -> Self {
    //         let failure_status_tbl = if !b {
    //             let mut failure_status_tbl = self.failure_status_tbl.clone();
    //             failure_status_tbl[0].insert(0, failure);
    //             failure_status_tbl
    //         } else {
    //             self.failure_status_tbl.clone()
    //         };

    //         Self {
    //             failure_status_tbl,
    //             success: self.success && b,
    //             ..self.clone()
    //         }
    //     }
    // }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct LocalState {
        pub stack_frame: Fp,
        pub call_stack: Fp,
        pub transaction_commitment: Fp,
        pub full_transaction_commitment: Fp,
        pub excess: Signed<Amount>,
        pub supply_increase: Signed<Amount>,
        pub ledger: Fp,
        pub success: bool,
        pub account_update_index: Index,
        pub failure_status_tbl: Vec<Vec<TransactionFailure>>,
        pub will_succeed: bool,
    }

    impl ToInputs for LocalState {
        /// <https://github.com/MinaProtocol/mina/blob/4e0b324912017c3ff576704ee397ade3d9bda412/src/lib/mina_state/local_state.ml#L116>
        fn to_inputs(&self, inputs: &mut Inputs) {
            let Self {
                stack_frame,
                call_stack,
                transaction_commitment,
                full_transaction_commitment,
                excess,
                supply_increase,
                ledger,
                success,
                account_update_index,
                failure_status_tbl: _,
                will_succeed,
            } = self;

            inputs.append(stack_frame);
            inputs.append(call_stack);
            inputs.append(transaction_commitment);
            inputs.append(full_transaction_commitment);
            inputs.append(excess);
            inputs.append(supply_increase);
            inputs.append(ledger);
            inputs.append(account_update_index);
            inputs.append(success);
            inputs.append(will_succeed);
        }
    }

    impl LocalState {
        /// <https://github.com/MinaProtocol/mina/blob/436023ba41c43a50458a551b7ef7a9ae61670b25/src/lib/mina_state/local_state.ml#L65>
        pub fn dummy() -> Self {
            Self {
                stack_frame: StackFrame::empty().hash(),
                call_stack: Fp::zero(),
                transaction_commitment: Fp::zero(),
                full_transaction_commitment: Fp::zero(),
                excess: Signed::<Amount>::zero(),
                supply_increase: Signed::<Amount>::zero(),
                ledger: Fp::zero(),
                success: true,
                account_update_index: <Index as Magnitude>::zero(),
                failure_status_tbl: Vec::new(),
                will_succeed: true,
            }
        }

        pub fn empty() -> Self {
            Self::dummy()
        }

        pub fn equal_without_ledger(&self, other: &Self) -> bool {
            let Self {
                stack_frame,
                call_stack,
                transaction_commitment,
                full_transaction_commitment,
                excess,
                supply_increase,
                ledger: _,
                success,
                account_update_index,
                failure_status_tbl,
                will_succeed,
            } = self;

            stack_frame == &other.stack_frame
                && call_stack == &other.call_stack
                && transaction_commitment == &other.transaction_commitment
                && full_transaction_commitment == &other.full_transaction_commitment
                && excess == &other.excess
                && supply_increase == &other.supply_increase
                && success == &other.success
                && account_update_index == &other.account_update_index
                && failure_status_tbl == &other.failure_status_tbl
                && will_succeed == &other.will_succeed
        }

        pub fn checked_equal_prime(&self, other: &Self, w: &mut Witness<Fp>) -> [Boolean; 11] {
            let Self {
                stack_frame,
                call_stack,
                transaction_commitment,
                full_transaction_commitment,
                excess,
                supply_increase,
                ledger,
                success,
                account_update_index,
                failure_status_tbl: _,
                will_succeed,
            } = self;

            // { stack_frame : 'stack_frame
            // ; call_stack : 'call_stack
            // ; transaction_commitment : 'comm
            // ; full_transaction_commitment : 'comm
            // ; excess : 'signed_amount
            // ; supply_increase : 'signed_amount
            // ; ledger : 'ledger
            // ; success : 'bool
            // ; account_update_index : 'length
            // ; failure_status_tbl : 'failure_status_tbl
            // ; will_succeed : 'bool
            // }

            let mut alls = [
                field::equal(*stack_frame, other.stack_frame, w),
                field::equal(*call_stack, other.call_stack, w),
                field::equal(*transaction_commitment, other.transaction_commitment, w),
                field::equal(
                    *full_transaction_commitment,
                    other.full_transaction_commitment,
                    w,
                ),
                excess
                    .to_checked::<Fp>()
                    .equal(&other.excess.to_checked(), w),
                supply_increase
                    .to_checked::<Fp>()
                    .equal(&other.supply_increase.to_checked(), w),
                field::equal(*ledger, other.ledger, w),
                success.to_boolean().equal(&other.success.to_boolean(), w),
                account_update_index
                    .to_checked::<Fp>()
                    .equal(&other.account_update_index.to_checked(), w),
                Boolean::True,
                will_succeed
                    .to_boolean()
                    .equal(&other.will_succeed.to_boolean(), w),
            ];
            alls.reverse();
            alls
        }
    }
}

fn step_all<A, L>(
    _constraint_constants: &ConstraintConstants,
    f: &impl Fn(&mut A, &GlobalState<L>, &LocalStateEnv<L>),
    user_acc: &mut A,
    (g_state, l_state): (&mut GlobalState<L>, &mut LocalStateEnv<L>),
) -> Result<Vec<Vec<TransactionFailure>>, String>
where
    L: LedgerNonSnark,
{
    while !l_state.stack_frame.calls.is_empty() {
        zkapps::non_snark::step(g_state, l_state)?;
        f(user_acc, g_state, l_state);
    }
    Ok(l_state.failure_status_tbl.clone())
}

/// apply zkapp command fee payer's while stubbing out the second pass ledger
/// CAUTION: If you use the intermediate local states, you MUST update the
/// [`LocalStateEnv::will_succeed`] field to `false` if the `status` is [`TransactionStatus::Failed`].*)
pub fn apply_zkapp_command_first_pass_aux<A, F, L>(
    constraint_constants: &ConstraintConstants,
    global_slot: Slot,
    state_view: &ProtocolStateView,
    init: &mut A,
    f: F,
    fee_excess: Option<Signed<Amount>>,
    supply_increase: Option<Signed<Amount>>,
    ledger: &mut L,
    command: &ZkAppCommand,
) -> Result<ZkappCommandPartiallyApplied<L>, String>
where
    L: LedgerNonSnark,
    F: Fn(&mut A, &GlobalState<L>, &LocalStateEnv<L>),
{
    let fee_excess = fee_excess.unwrap_or_else(Signed::zero);
    let supply_increase = supply_increase.unwrap_or_else(Signed::zero);

    let previous_hash = ledger.merkle_root();
    let original_first_pass_account_states = {
        let id = command.fee_payer();
        let location = {
            let loc = ledger.location_of_account(&id);
            let account = loc.as_ref().and_then(|loc| ledger.get(loc));
            loc.zip(account)
        };

        vec![(id, location)]
    };
    // let perform = |eff: Eff<L>| Env::perform(eff);

    let (mut global_state, mut local_state) = (
        GlobalState {
            protocol_state: state_view.clone(),
            first_pass_ledger: ledger.clone(),
            second_pass_ledger: {
                // We stub out the second_pass_ledger initially, and then poke the
                // correct value in place after the first pass is finished.
                <L as LedgerIntf>::empty(0)
            },
            fee_excess,
            supply_increase,
            block_global_slot: global_slot,
        },
        LocalStateEnv {
            stack_frame: StackFrame::default(),
            call_stack: CallStack::new(),
            transaction_commitment: Fp::zero(),
            full_transaction_commitment: Fp::zero(),
            excess: Signed::<Amount>::zero(),
            supply_increase,
            ledger: <L as LedgerIntf>::empty(0),
            success: true,
            account_update_index: Index::zero(),
            failure_status_tbl: Vec::new(),
            will_succeed: true,
        },
    );

    f(init, &global_state, &local_state);
    let account_updates = command.all_account_updates();

    zkapps::non_snark::start(
        &mut global_state,
        &mut local_state,
        zkapps::non_snark::StartData {
            account_updates,
            memo_hash: command.memo.hash(),
            // It's always valid to set this value to true, and it will
            // have no effect outside of the snark.
            will_succeed: true,
        },
    )?;

    let command = command.clone();
    let constraint_constants = constraint_constants.clone();
    let state_view = state_view.clone();

    let res = ZkappCommandPartiallyApplied {
        command,
        previous_hash,
        original_first_pass_account_states,
        constraint_constants,
        state_view,
        global_state,
        local_state,
    };

    Ok(res)
}

fn apply_zkapp_command_first_pass<L>(
    constraint_constants: &ConstraintConstants,
    global_slot: Slot,
    state_view: &ProtocolStateView,
    fee_excess: Option<Signed<Amount>>,
    supply_increase: Option<Signed<Amount>>,
    ledger: &mut L,
    command: &ZkAppCommand,
) -> Result<ZkappCommandPartiallyApplied<L>, String>
where
    L: LedgerNonSnark,
{
    let mut acc = ();
    let partial_stmt = apply_zkapp_command_first_pass_aux(
        constraint_constants,
        global_slot,
        state_view,
        &mut acc,
        |_acc, _g, _l| {},
        fee_excess,
        supply_increase,
        ledger,
        command,
    )?;

    Ok(partial_stmt)
}

pub fn apply_zkapp_command_second_pass_aux<A, F, L>(
    constraint_constants: &ConstraintConstants,
    init: &mut A,
    f: F,
    ledger: &mut L,
    c: ZkappCommandPartiallyApplied<L>,
) -> Result<ZkappCommandApplied, String>
where
    L: LedgerNonSnark,
    F: Fn(&mut A, &GlobalState<L>, &LocalStateEnv<L>),
{
    // let perform = |eff: Eff<L>| Env::perform(eff);

    let original_account_states: Vec<(AccountId, Option<_>)> = {
        // get the original states of all the accounts in each pass.
        // If an account updated in the first pass is referenced in account
        // updates, then retain the value before first pass application*)

        let accounts_referenced = c.command.accounts_referenced();

        let mut account_states = BTreeMap::<AccountIdOrderable, Option<_>>::new();

        let referenced = accounts_referenced.into_iter().map(|id| {
            let location = {
                let loc = ledger.location_of_account(&id);
                let account = loc.as_ref().and_then(|loc| ledger.get(loc));
                loc.zip(account)
            };
            (id, location)
        });

        c.original_first_pass_account_states
            .into_iter()
            .chain(referenced)
            .for_each(|(id, acc_opt)| {
                use std::collections::btree_map::Entry::Vacant;

                let id_with_order: AccountIdOrderable = id.into();
                if let Vacant(entry) = account_states.entry(id_with_order) {
                    entry.insert(acc_opt);
                };
            });

        account_states
            .into_iter()
            // Convert back the `AccountIdOrder` into `AccountId`, now that they are sorted
            .map(|(id, account): (AccountIdOrderable, Option<_>)| (id.into(), account))
            .collect()
    };

    let mut account_states_after_fee_payer = {
        // To check if the accounts remain unchanged in the event the transaction
        // fails. First pass updates will remain even if the transaction fails to
        // apply zkapp account updates*)

        c.command.accounts_referenced().into_iter().map(|id| {
            let loc = ledger.location_of_account(&id);
            let a = loc.as_ref().and_then(|loc| ledger.get(loc));

            match a {
                Some(a) => (id, Some((loc.unwrap(), a))),
                None => (id, None),
            }
        })
    };

    let accounts = || {
        original_account_states
            .iter()
            .map(|(id, account)| (id.clone(), account.as_ref().map(|(_loc, acc)| acc.clone())))
            .collect::<Vec<_>>()
    };

    // Warning(OCaml): This is an abstraction leak / hack.
    // Here, we update global second pass ledger to be the input ledger, and
    // then update the local ledger to be the input ledger *IF AND ONLY IF*
    // there are more transaction segments to be processed in this pass.

    // TODO(OCaml): Remove this, and uplift the logic into the call in staged ledger.

    let mut global_state = GlobalState {
        second_pass_ledger: ledger.clone(),
        ..c.global_state
    };

    let mut local_state = {
        if c.local_state.stack_frame.calls.is_empty() {
            // Don't mess with the local state; we've already finished the
            // transaction after the fee payer.
            c.local_state
        } else {
            // Install the ledger that should already be in the local state, but
            // may not be in some situations depending on who the caller is.
            LocalStateEnv {
                ledger: global_state.second_pass_ledger(),
                ..c.local_state
            }
        }
    };

    f(init, &global_state, &local_state);
    let start = (&mut global_state, &mut local_state);

    let reversed_failure_status_tbl = step_all(constraint_constants, &f, init, start)?;

    let failure_status_tbl = reversed_failure_status_tbl
        .into_iter()
        .rev()
        .collect::<Vec<_>>();

    let account_ids_originally_not_in_ledger =
        original_account_states
            .iter()
            .filter_map(|(acct_id, loc_and_acct)| {
                if loc_and_acct.is_none() {
                    Some(acct_id)
                } else {
                    None
                }
            });

    let successfully_applied = failure_status_tbl.concat().is_empty();

    // if the zkapp command fails in at least 1 account update,
    // then all the account updates would be cancelled except
    // the fee payer one
    let failure_status_tbl = if successfully_applied {
        failure_status_tbl
    } else {
        failure_status_tbl
            .into_iter()
            .enumerate()
            .map(|(idx, fs)| {
                if idx > 0 && fs.is_empty() {
                    vec![TransactionFailure::Cancelled]
                } else {
                    fs
                }
            })
            .collect()
    };

    // accounts not originally in ledger, now present in ledger
    let new_accounts = account_ids_originally_not_in_ledger
        .filter(|acct_id| ledger.location_of_account(acct_id).is_some())
        .cloned()
        .collect::<Vec<_>>();

    let new_accounts_is_empty = new_accounts.is_empty();

    let valid_result = Ok(ZkappCommandApplied {
        accounts: accounts(),
        command: WithStatus {
            data: c.command,
            status: if successfully_applied {
                TransactionStatus::Applied
            } else {
                TransactionStatus::Failed(failure_status_tbl)
            },
        },
        new_accounts,
    });

    if successfully_applied {
        valid_result
    } else {
        let other_account_update_accounts_unchanged = account_states_after_fee_payer
            .fold_while(true, |acc, (_, loc_opt)| match loc_opt {
                Some((loc, a)) => match ledger.get(&loc) {
                    Some(a_) if !(a == a_) => FoldWhile::Done(false),
                    _ => FoldWhile::Continue(acc),
                },
                _ => FoldWhile::Continue(acc),
            })
            .into_inner();

        // Other zkapp_command failed, therefore, updates in those should not get applied
        if new_accounts_is_empty && other_account_update_accounts_unchanged {
            valid_result
        } else {
            Err("Zkapp_command application failed but new accounts created or some of the other account_update updates applied".to_string())
        }
    }
}

fn apply_zkapp_command_second_pass<L>(
    constraint_constants: &ConstraintConstants,
    ledger: &mut L,
    c: ZkappCommandPartiallyApplied<L>,
) -> Result<ZkappCommandApplied, String>
where
    L: LedgerNonSnark,
{
    let x = apply_zkapp_command_second_pass_aux(
        constraint_constants,
        &mut (),
        |_, _, _| {},
        ledger,
        c,
    )?;
    Ok(x)
}

fn apply_zkapp_command_unchecked_aux<A, F, L>(
    constraint_constants: &ConstraintConstants,
    global_slot: Slot,
    state_view: &ProtocolStateView,
    init: &mut A,
    f: F,
    fee_excess: Option<Signed<Amount>>,
    supply_increase: Option<Signed<Amount>>,
    ledger: &mut L,
    command: &ZkAppCommand,
) -> Result<ZkappCommandApplied, String>
where
    L: LedgerNonSnark,
    F: Fn(&mut A, &GlobalState<L>, &LocalStateEnv<L>),
{
    let partial_stmt = apply_zkapp_command_first_pass_aux(
        constraint_constants,
        global_slot,
        state_view,
        init,
        &f,
        fee_excess,
        supply_increase,
        ledger,
        command,
    )?;

    apply_zkapp_command_second_pass_aux(constraint_constants, init, &f, ledger, partial_stmt)
}

fn apply_zkapp_command_unchecked<L>(
    constraint_constants: &ConstraintConstants,
    global_slot: Slot,
    state_view: &ProtocolStateView,
    ledger: &mut L,
    command: &ZkAppCommand,
) -> Result<(ZkappCommandApplied, (LocalStateEnv<L>, Signed<Amount>)), String>
where
    L: LedgerNonSnark,
{
    let zkapp_partially_applied: ZkappCommandPartiallyApplied<L> = apply_zkapp_command_first_pass(
        constraint_constants,
        global_slot,
        state_view,
        None,
        None,
        ledger,
        command,
    )?;

    let mut state_res = None;
    let account_update_applied = apply_zkapp_command_second_pass_aux(
        constraint_constants,
        &mut state_res,
        |acc, global_state, local_state| {
            *acc = Some((local_state.clone(), global_state.fee_excess))
        },
        ledger,
        zkapp_partially_applied,
    )?;
    let (state, amount) = state_res.unwrap();

    Ok((account_update_applied, (state.clone(), amount)))
}

pub mod transaction_partially_applied {
    use super::{
        transaction_applied::{CoinbaseApplied, FeeTransferApplied},
        *,
    };

    #[derive(Clone, Debug)]
    pub struct ZkappCommandPartiallyApplied<L: LedgerNonSnark> {
        pub command: ZkAppCommand,
        pub previous_hash: Fp,
        pub original_first_pass_account_states:
            Vec<(AccountId, Option<(L::Location, Box<Account>)>)>,
        pub constraint_constants: ConstraintConstants,
        pub state_view: ProtocolStateView,
        pub global_state: GlobalState<L>,
        pub local_state: LocalStateEnv<L>,
    }

    #[derive(Clone, Debug)]
    pub struct FullyApplied<T> {
        pub previous_hash: Fp,
        pub applied: T,
    }

    #[derive(Clone, Debug)]
    pub enum TransactionPartiallyApplied<L: LedgerNonSnark> {
        SignedCommand(FullyApplied<SignedCommandApplied>),
        ZkappCommand(Box<ZkappCommandPartiallyApplied<L>>),
        FeeTransfer(FullyApplied<FeeTransferApplied>),
        Coinbase(FullyApplied<CoinbaseApplied>),
    }

    impl<L> TransactionPartiallyApplied<L>
    where
        L: LedgerNonSnark,
    {
        pub fn command(self) -> Transaction {
            use Transaction as T;

            match self {
                Self::SignedCommand(s) => T::Command(UserCommand::SignedCommand(Box::new(
                    s.applied.common.user_command.data,
                ))),
                Self::ZkappCommand(z) => T::Command(UserCommand::ZkAppCommand(Box::new(z.command))),
                Self::FeeTransfer(ft) => T::FeeTransfer(ft.applied.fee_transfer.data),
                Self::Coinbase(cb) => T::Coinbase(cb.applied.coinbase.data),
            }
        }
    }
}

use transaction_partially_applied::{TransactionPartiallyApplied, ZkappCommandPartiallyApplied};

pub fn apply_transaction_first_pass<L>(
    constraint_constants: &ConstraintConstants,
    global_slot: Slot,
    txn_state_view: &ProtocolStateView,
    ledger: &mut L,
    transaction: &Transaction,
) -> Result<TransactionPartiallyApplied<L>, String>
where
    L: LedgerNonSnark,
{
    use Transaction::*;
    use UserCommand::*;

    let previous_hash = ledger.merkle_root();
    let txn_global_slot = &global_slot;

    match transaction {
        Command(SignedCommand(cmd)) => apply_user_command(
            constraint_constants,
            txn_state_view,
            txn_global_slot,
            ledger,
            cmd,
        )
        .map(|applied| {
            TransactionPartiallyApplied::SignedCommand(FullyApplied {
                previous_hash,
                applied,
            })
        }),
        Command(ZkAppCommand(txn)) => apply_zkapp_command_first_pass(
            constraint_constants,
            global_slot,
            txn_state_view,
            None,
            None,
            ledger,
            txn,
        )
        .map(Box::new)
        .map(TransactionPartiallyApplied::ZkappCommand),
        FeeTransfer(fee_transfer) => {
            apply_fee_transfer(constraint_constants, txn_global_slot, ledger, fee_transfer).map(
                |applied| {
                    TransactionPartiallyApplied::FeeTransfer(FullyApplied {
                        previous_hash,
                        applied,
                    })
                },
            )
        }
        Coinbase(coinbase) => {
            apply_coinbase(constraint_constants, txn_global_slot, ledger, coinbase).map(|applied| {
                TransactionPartiallyApplied::Coinbase(FullyApplied {
                    previous_hash,
                    applied,
                })
            })
        }
    }
}

pub fn apply_transaction_second_pass<L>(
    constraint_constants: &ConstraintConstants,
    ledger: &mut L,
    partial_transaction: TransactionPartiallyApplied<L>,
) -> Result<TransactionApplied, String>
where
    L: LedgerNonSnark,
{
    use TransactionPartiallyApplied as P;

    match partial_transaction {
        P::SignedCommand(FullyApplied {
            previous_hash,
            applied,
        }) => Ok(TransactionApplied {
            previous_hash,
            varying: Varying::Command(CommandApplied::SignedCommand(Box::new(applied))),
        }),
        P::ZkappCommand(partially_applied) => {
            // TODO(OCaml): either here or in second phase of apply, need to update the
            // prior global state statement for the fee payer segment to add the
            // second phase ledger at the end

            let previous_hash = partially_applied.previous_hash;
            let applied =
                apply_zkapp_command_second_pass(constraint_constants, ledger, *partially_applied)?;

            Ok(TransactionApplied {
                previous_hash,
                varying: Varying::Command(CommandApplied::ZkappCommand(Box::new(applied))),
            })
        }
        P::FeeTransfer(FullyApplied {
            previous_hash,
            applied,
        }) => Ok(TransactionApplied {
            previous_hash,
            varying: Varying::FeeTransfer(applied),
        }),
        P::Coinbase(FullyApplied {
            previous_hash,
            applied,
        }) => Ok(TransactionApplied {
            previous_hash,
            varying: Varying::Coinbase(applied),
        }),
    }
}

pub fn apply_transactions<L>(
    constraint_constants: &ConstraintConstants,
    global_slot: Slot,
    txn_state_view: &ProtocolStateView,
    ledger: &mut L,
    txns: &[Transaction],
) -> Result<Vec<TransactionApplied>, String>
where
    L: LedgerNonSnark,
{
    let first_pass: Vec<_> = txns
        .iter()
        .map(|txn| {
            apply_transaction_first_pass(
                constraint_constants,
                global_slot,
                txn_state_view,
                ledger,
                txn,
            )
        })
        .collect::<Result<Vec<TransactionPartiallyApplied<_>>, _>>()?;

    first_pass
        .into_iter()
        .map(|partial_transaction| {
            apply_transaction_second_pass(constraint_constants, ledger, partial_transaction)
        })
        .collect()
}

struct FailureCollection {
    inner: Vec<Vec<TransactionFailure>>,
}

/// <https://github.com/MinaProtocol/mina/blob/bfd1009abdbee78979ff0343cc73a3480e862f58/src/lib/transaction_logic/mina_transaction_logic.ml#L2197C1-L2210C53>
impl FailureCollection {
    fn empty() -> Self {
        Self {
            inner: Vec::default(),
        }
    }

    fn no_failure() -> Vec<TransactionFailure> {
        vec![]
    }

    /// <https://github.com/MinaProtocol/mina/blob/bfd1009abdbee78979ff0343cc73a3480e862f58/src/lib/transaction_logic/mina_transaction_logic.ml#L2204>
    fn single_failure() -> Self {
        Self {
            inner: vec![vec![TransactionFailure::UpdateNotPermittedBalance]],
        }
    }

    fn update_failed() -> Vec<TransactionFailure> {
        vec![TransactionFailure::UpdateNotPermittedBalance]
    }

    /// <https://github.com/MinaProtocol/mina/blob/bfd1009abdbee78979ff0343cc73a3480e862f58/src/lib/transaction_logic/mina_transaction_logic.ml#L2208>
    fn append_entry(list: Vec<TransactionFailure>, mut s: Self) -> Self {
        if s.inner.is_empty() {
            Self { inner: vec![list] }
        } else {
            s.inner.insert(1, list);
            s
        }
    }

    fn is_empty(&self) -> bool {
        self.inner.iter().all(Vec::is_empty)
    }

    fn take(self) -> Vec<Vec<TransactionFailure>> {
        self.inner
    }
}

/// Structure of the failure status:
///  I. No fee transfer and coinbase transfer fails: `[[failure]]`
///  II. With fee transfer-
///   Both fee transfer and coinbase fails:
///     `[[failure-of-fee-transfer]; [failure-of-coinbase]]`
///   Fee transfer succeeds and coinbase fails:
///     `[[];[failure-of-coinbase]]`
///   Fee transfer fails and coinbase succeeds:
///     `[[failure-of-fee-transfer];[]]`
///
/// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/transaction_logic/mina_transaction_logic.ml#L2022>
fn apply_coinbase<L>(
    constraint_constants: &ConstraintConstants,
    txn_global_slot: &Slot,
    ledger: &mut L,
    coinbase: &Coinbase,
) -> Result<transaction_applied::CoinbaseApplied, String>
where
    L: LedgerIntf,
{
    let Coinbase {
        receiver,
        amount: coinbase_amount,
        fee_transfer,
    } = &coinbase;

    let (
        receiver_reward,
        new_accounts1,
        transferee_update,
        transferee_timing_prev,
        failures1,
        burned_tokens1,
    ) = match fee_transfer {
        None => (
            *coinbase_amount,
            None,
            None,
            None,
            FailureCollection::empty(),
            Amount::zero(),
        ),
        Some(
            ft @ CoinbaseFeeTransfer {
                receiver_pk: transferee,
                fee,
            },
        ) => {
            assert_ne!(transferee, receiver);

            let transferee_id = ft.receiver();
            let fee = Amount::of_fee(fee);

            let receiver_reward = coinbase_amount
                .checked_sub(&fee)
                .ok_or_else(|| "Coinbase fee transfer too large".to_string())?;

            let (transferee_account, action, can_receive) =
                has_permission_to_receive(ledger, &transferee_id);
            let new_accounts = get_new_accounts(action, transferee_id.clone());

            let timing = update_timing_when_no_deduction(txn_global_slot, &transferee_account)?;

            let balance = {
                let amount = sub_account_creation_fee(constraint_constants, action, fee)?;
                add_amount(transferee_account.balance, amount)?
            };

            if can_receive.0 {
                let (_, mut transferee_account, transferee_location) =
                    ledger.get_or_create(&transferee_id)?;

                transferee_account.balance = balance;
                transferee_account.timing = timing;

                let timing = transferee_account.timing.clone();

                (
                    receiver_reward,
                    new_accounts,
                    Some((transferee_location, transferee_account)),
                    Some(timing),
                    FailureCollection::append_entry(
                        FailureCollection::no_failure(),
                        FailureCollection::empty(),
                    ),
                    Amount::zero(),
                )
            } else {
                (
                    receiver_reward,
                    None,
                    None,
                    None,
                    FailureCollection::single_failure(),
                    fee,
                )
            }
        }
    };

    let receiver_id = AccountId::new(receiver.clone(), TokenId::default());
    let (receiver_account, action2, can_receive) = has_permission_to_receive(ledger, &receiver_id);
    let new_accounts2 = get_new_accounts(action2, receiver_id.clone());

    // Note: Updating coinbase receiver timing only if there is no fee transfer.
    // This is so as to not add any extra constraints in transaction snark for checking
    // "receiver" timings. This is OK because timing rules will not be violated when
    // balance increases and will be checked whenever an amount is deducted from the
    // account (#5973)

    let coinbase_receiver_timing = match transferee_timing_prev {
        None => update_timing_when_no_deduction(txn_global_slot, &receiver_account)?,
        Some(_) => receiver_account.timing.clone(),
    };

    let receiver_balance = {
        let amount = sub_account_creation_fee(constraint_constants, action2, receiver_reward)?;
        add_amount(receiver_account.balance, amount)?
    };

    let (failures, burned_tokens2) = if can_receive.0 {
        let (_action2, mut receiver_account, receiver_location) =
            ledger.get_or_create(&receiver_id)?;

        receiver_account.balance = receiver_balance;
        receiver_account.timing = coinbase_receiver_timing;

        ledger.set(&receiver_location, receiver_account);

        (
            FailureCollection::append_entry(FailureCollection::no_failure(), failures1),
            Amount::zero(),
        )
    } else {
        (
            FailureCollection::append_entry(FailureCollection::update_failed(), failures1),
            receiver_reward,
        )
    };

    if let Some((addr, account)) = transferee_update {
        ledger.set(&addr, account);
    };

    let burned_tokens = burned_tokens1
        .checked_add(&burned_tokens2)
        .ok_or_else(|| "burned tokens overflow".to_string())?;

    let status = if failures.is_empty() {
        TransactionStatus::Applied
    } else {
        TransactionStatus::Failed(failures.take())
    };

    let new_accounts: Vec<_> = [new_accounts1, new_accounts2]
        .into_iter()
        .flatten()
        .collect();

    Ok(transaction_applied::CoinbaseApplied {
        coinbase: WithStatus {
            data: coinbase.clone(),
            status,
        },
        new_accounts,
        burned_tokens,
    })
}

/// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/transaction_logic/mina_transaction_logic.ml#L1991>
fn apply_fee_transfer<L>(
    constraint_constants: &ConstraintConstants,
    txn_global_slot: &Slot,
    ledger: &mut L,
    fee_transfer: &FeeTransfer,
) -> Result<transaction_applied::FeeTransferApplied, String>
where
    L: LedgerIntf,
{
    let (new_accounts, failures, burned_tokens) = process_fee_transfer(
        ledger,
        fee_transfer,
        |action, _, balance, fee| {
            let amount = {
                let amount = Amount::of_fee(fee);
                sub_account_creation_fee(constraint_constants, action, amount)?
            };
            add_amount(balance, amount)
        },
        |account| update_timing_when_no_deduction(txn_global_slot, account),
    )?;

    let status = if failures.is_empty() {
        TransactionStatus::Applied
    } else {
        TransactionStatus::Failed(failures.take())
    };

    Ok(transaction_applied::FeeTransferApplied {
        fee_transfer: WithStatus {
            data: fee_transfer.clone(),
            status,
        },
        new_accounts,
        burned_tokens,
    })
}

/// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/transaction_logic/mina_transaction_logic.ml#L607>
fn sub_account_creation_fee(
    constraint_constants: &ConstraintConstants,
    action: AccountState,
    amount: Amount,
) -> Result<Amount, String> {
    let account_creation_fee = Amount::from_u64(constraint_constants.account_creation_fee);

    match action {
        AccountState::Added => {
            if let Some(amount) = amount.checked_sub(&account_creation_fee) {
                return Ok(amount);
            }
            Err(format!(
                "Error subtracting account creation fee {:?}; transaction amount {:?} insufficient",
                account_creation_fee, amount
            ))
        }
        AccountState::Existed => Ok(amount),
    }
}

fn update_timing_when_no_deduction(
    txn_global_slot: &Slot,
    account: &Account,
) -> Result<Timing, String> {
    validate_timing(account, Amount::zero(), txn_global_slot)
}

// /// TODO: Move this to the ledger
// /// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_ledger/ledger.ml#L311>
// fn get_or_create<L>(
//     ledger: &mut L,
//     account_id: &AccountId,
// ) -> Result<(AccountState, Account, Address), String>
// where
//     L: LedgerIntf,
// {
//     let location = ledger
//         .get_or_create_account(account_id.clone(), Account::initialize(account_id))
//         .map_err(|e| format!("{:?}", e))?;

//     let action = match location {
//         GetOrCreated::Added(_) => AccountState::Added,
//         GetOrCreated::Existed(_) => AccountState::Existed,
//     };

//     let addr = location.addr();

//     let account = ledger
//         .get(addr.clone())
//         .expect("get_or_create: Account was not found in the ledger after creation");

//     Ok((action, account, addr))
// }

fn get_new_accounts<T>(action: AccountState, data: T) -> Option<T> {
    match action {
        AccountState::Added => Some(data),
        AccountState::Existed => None,
    }
}

/// Structure of the failure status:
///  I. Only one fee transfer in the transaction (`One) and it fails:
///     [[failure]]
///  II. Two fee transfers in the transaction (`Two)-
///   Both fee transfers fail:
///     [[failure-of-first-fee-transfer]; [failure-of-second-fee-transfer]]
///   First succeeds and second one fails:
///     [[];[failure-of-second-fee-transfer]]
///   First fails and second succeeds:
///     [[failure-of-first-fee-transfer];[]]
fn process_fee_transfer<L, FunBalance, FunTiming>(
    ledger: &mut L,
    fee_transfer: &FeeTransfer,
    modify_balance: FunBalance,
    modify_timing: FunTiming,
) -> Result<(Vec<AccountId>, FailureCollection, Amount), String>
where
    L: LedgerIntf,
    FunTiming: Fn(&Account) -> Result<Timing, String>,
    FunBalance: Fn(AccountState, &AccountId, Balance, &Fee) -> Result<Balance, String>,
{
    if !fee_transfer.fee_tokens().all(TokenId::is_default) {
        return Err("Cannot pay fees in non-default tokens.".to_string());
    }

    match &**fee_transfer {
        OneOrTwo::One(fee_transfer) => {
            let account_id = fee_transfer.receiver();
            let (a, action, can_receive) = has_permission_to_receive(ledger, &account_id);

            let timing = modify_timing(&a)?;
            let balance = modify_balance(action, &account_id, a.balance, &fee_transfer.fee)?;

            if can_receive.0 {
                let (_, mut account, loc) = ledger.get_or_create(&account_id)?;
                let new_accounts = get_new_accounts(action, account_id.clone());

                account.balance = balance;
                account.timing = timing;

                ledger.set(&loc, account);

                let new_accounts: Vec<_> = new_accounts.into_iter().collect();
                Ok((new_accounts, FailureCollection::empty(), Amount::zero()))
            } else {
                Ok((
                    vec![],
                    FailureCollection::single_failure(),
                    Amount::of_fee(&fee_transfer.fee),
                ))
            }
        }
        OneOrTwo::Two((fee_transfer1, fee_transfer2)) => {
            let account_id1 = fee_transfer1.receiver();
            let (a1, action1, can_receive1) = has_permission_to_receive(ledger, &account_id1);

            let account_id2 = fee_transfer2.receiver();

            if account_id1 == account_id2 {
                let fee = fee_transfer1
                    .fee
                    .checked_add(&fee_transfer2.fee)
                    .ok_or_else(|| "Overflow".to_string())?;

                let timing = modify_timing(&a1)?;
                let balance = modify_balance(action1, &account_id1, a1.balance, &fee)?;

                if can_receive1.0 {
                    let (_, mut a1, l1) = ledger.get_or_create(&account_id1)?;
                    let new_accounts1 = get_new_accounts(action1, account_id1);

                    a1.balance = balance;
                    a1.timing = timing;

                    ledger.set(&l1, a1);

                    let new_accounts: Vec<_> = new_accounts1.into_iter().collect();
                    Ok((new_accounts, FailureCollection::empty(), Amount::zero()))
                } else {
                    // failure for each fee transfer single

                    Ok((
                        vec![],
                        FailureCollection::append_entry(
                            FailureCollection::update_failed(),
                            FailureCollection::single_failure(),
                        ),
                        Amount::of_fee(&fee),
                    ))
                }
            } else {
                let (a2, action2, can_receive2) = has_permission_to_receive(ledger, &account_id2);

                let balance1 =
                    modify_balance(action1, &account_id1, a1.balance, &fee_transfer1.fee)?;

                // Note: Not updating the timing field of a1 to avoid additional check
                // in transactions snark (check_timing for "receiver"). This is OK
                // because timing rules will not be violated when balance increases
                // and will be checked whenever an amount is deducted from the account. (#5973)*)

                let timing2 = modify_timing(&a2)?;
                let balance2 =
                    modify_balance(action2, &account_id2, a2.balance, &fee_transfer2.fee)?;

                let (new_accounts1, failures, burned_tokens1) = if can_receive1.0 {
                    let (_, mut a1, l1) = ledger.get_or_create(&account_id1)?;
                    let new_accounts1 = get_new_accounts(action1, account_id1);

                    a1.balance = balance1;
                    ledger.set(&l1, a1);

                    (
                        new_accounts1,
                        FailureCollection::append_entry(
                            FailureCollection::no_failure(),
                            FailureCollection::empty(),
                        ),
                        Amount::zero(),
                    )
                } else {
                    (
                        None,
                        FailureCollection::single_failure(),
                        Amount::of_fee(&fee_transfer1.fee),
                    )
                };

                let (new_accounts2, failures, burned_tokens2) = if can_receive2.0 {
                    let (_, mut a2, l2) = ledger.get_or_create(&account_id2)?;
                    let new_accounts2 = get_new_accounts(action2, account_id2);

                    a2.balance = balance2;
                    a2.timing = timing2;

                    ledger.set(&l2, a2);

                    (
                        new_accounts2,
                        FailureCollection::append_entry(FailureCollection::no_failure(), failures),
                        Amount::zero(),
                    )
                } else {
                    (
                        None,
                        FailureCollection::append_entry(
                            FailureCollection::update_failed(),
                            failures,
                        ),
                        Amount::of_fee(&fee_transfer2.fee),
                    )
                };

                let burned_tokens = burned_tokens1
                    .checked_add(&burned_tokens2)
                    .ok_or_else(|| "burned tokens overflow".to_string())?;

                let new_accounts: Vec<_> = [new_accounts1, new_accounts2]
                    .into_iter()
                    .flatten()
                    .collect();

                Ok((new_accounts, failures, burned_tokens))
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum AccountState {
    Added,
    Existed,
}

#[derive(Debug)]
struct HasPermissionToReceive(bool);

/// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/transaction_logic/mina_transaction_logic.ml#L1852>
fn has_permission_to_receive<L>(
    ledger: &mut L,
    receiver_account_id: &AccountId,
) -> (Box<Account>, AccountState, HasPermissionToReceive)
where
    L: LedgerIntf,
{
    use crate::PermissionTo::*;
    use AccountState::*;

    let init_account = Account::initialize(receiver_account_id);

    match ledger.location_of_account(receiver_account_id) {
        None => {
            // new account, check that default permissions allow receiving
            let perm = init_account.has_permission_to(ControlTag::NoneGiven, Receive);
            (Box::new(init_account), Added, HasPermissionToReceive(perm))
        }
        Some(location) => match ledger.get(&location) {
            None => panic!("Ledger location with no account"),
            Some(receiver_account) => {
                let perm = receiver_account.has_permission_to(ControlTag::NoneGiven, Receive);
                (receiver_account, Existed, HasPermissionToReceive(perm))
            }
        },
    }
}

pub fn validate_time(valid_until: &Slot, current_global_slot: &Slot) -> Result<(), String> {
    if current_global_slot <= valid_until {
        return Ok(());
    }

    Err(format!(
        "Current global slot {:?} greater than transaction expiry slot {:?}",
        current_global_slot, valid_until
    ))
}

pub fn is_timed(a: &Account) -> bool {
    matches!(&a.timing, Timing::Timed { .. })
}

pub fn set_with_location<L>(
    ledger: &mut L,
    location: &ExistingOrNew<L::Location>,
    account: Box<Account>,
) -> Result<(), String>
where
    L: LedgerIntf,
{
    match location {
        ExistingOrNew::Existing(location) => {
            ledger.set(location, account);
            Ok(())
        }
        ExistingOrNew::New => ledger
            .create_new_account(account.id(), *account)
            .map_err(|_| "set_with_location".to_string()),
    }
}

pub struct Updates<Location> {
    pub located_accounts: Vec<(ExistingOrNew<Location>, Box<Account>)>,
    pub applied_body: signed_command_applied::Body,
}

pub fn compute_updates<L>(
    constraint_constants: &ConstraintConstants,
    receiver: AccountId,
    ledger: &mut L,
    current_global_slot: &Slot,
    user_command: &SignedCommand,
    fee_payer: &AccountId,
    fee_payer_account: &Account,
    fee_payer_location: &ExistingOrNew<L::Location>,
    reject_command: &mut bool,
) -> Result<Updates<L::Location>, TransactionFailure>
where
    L: LedgerIntf,
{
    match &user_command.payload.body {
        signed_command::Body::StakeDelegation(_) => {
            let (receiver_location, _) = get_with_location(ledger, &receiver).unwrap();

            if let ExistingOrNew::New = receiver_location {
                return Err(TransactionFailure::ReceiverNotPresent);
            }
            if !fee_payer_account.has_permission_to_set_delegate() {
                return Err(TransactionFailure::UpdateNotPermittedDelegate);
            }

            let previous_delegate = fee_payer_account.delegate.clone();

            // Timing is always valid, but we need to record any switch from
            // timed to untimed here to stay in sync with the snark.
            let fee_payer_account = {
                let timing = timing_error_to_user_command_status(validate_timing(
                    fee_payer_account,
                    Amount::zero(),
                    current_global_slot,
                ))?;

                Box::new(Account {
                    delegate: Some(receiver.public_key.clone()),
                    timing,
                    ..fee_payer_account.clone()
                })
            };

            Ok(Updates {
                located_accounts: vec![(fee_payer_location.clone(), fee_payer_account)],
                applied_body: signed_command_applied::Body::StakeDelegation { previous_delegate },
            })
        }
        signed_command::Body::Payment(payment) => {
            let get_fee_payer_account = || {
                let balance = fee_payer_account
                    .balance
                    .sub_amount(payment.amount)
                    .ok_or(TransactionFailure::SourceInsufficientBalance)?;

                let timing = timing_error_to_user_command_status(validate_timing(
                    fee_payer_account,
                    payment.amount,
                    current_global_slot,
                ))?;

                Ok(Box::new(Account {
                    balance,
                    timing,
                    ..fee_payer_account.clone()
                }))
            };

            let fee_payer_account = match get_fee_payer_account() {
                Ok(fee_payer_account) => fee_payer_account,
                Err(e) => {
                    // OCaml throw an exception when an error occurs here
                    // Here in Rust we set `reject_command` to differentiate the 3 cases (Ok, Err, exception)
                    //
                    // <https://github.com/MinaProtocol/mina/blob/bfd1009abdbee78979ff0343cc73a3480e862f58/src/lib/transaction_logic/mina_transaction_logic.ml#L962>

                    // Don't accept transactions with insufficient balance from the fee-payer.
                    // TODO(OCaml): eliminate this condition and accept transaction with failed status
                    *reject_command = true;
                    return Err(e);
                }
            };

            let (receiver_location, mut receiver_account) = if fee_payer == &receiver {
                (fee_payer_location.clone(), fee_payer_account.clone())
            } else {
                get_with_location(ledger, &receiver).unwrap()
            };

            if !fee_payer_account.has_permission_to_send() {
                return Err(TransactionFailure::UpdateNotPermittedBalance);
            }

            if !receiver_account.has_permission_to_receive() {
                return Err(TransactionFailure::UpdateNotPermittedBalance);
            }

            let receiver_amount = match &receiver_location {
                ExistingOrNew::Existing(_) => payment.amount,
                ExistingOrNew::New => {
                    match payment
                        .amount
                        .checked_sub(&Amount::from_u64(constraint_constants.account_creation_fee))
                    {
                        Some(amount) => amount,
                        None => return Err(TransactionFailure::AmountInsufficientToCreateAccount),
                    }
                }
            };

            let balance = match receiver_account.balance.add_amount(receiver_amount) {
                Some(balance) => balance,
                None => return Err(TransactionFailure::Overflow),
            };

            let new_accounts = match receiver_location {
                ExistingOrNew::New => vec![receiver.clone()],
                ExistingOrNew::Existing(_) => vec![],
            };

            receiver_account.balance = balance;

            let updated_accounts = if fee_payer == &receiver {
                // [receiver_account] at this point has all the updates
                vec![(receiver_location, receiver_account)]
            } else {
                vec![
                    (receiver_location, receiver_account),
                    (fee_payer_location.clone(), fee_payer_account),
                ]
            };

            Ok(Updates {
                located_accounts: updated_accounts,
                applied_body: signed_command_applied::Body::Payments { new_accounts },
            })
        }
    }
}

pub fn apply_user_command_unchecked<L>(
    constraint_constants: &ConstraintConstants,
    _txn_state_view: &ProtocolStateView,
    txn_global_slot: &Slot,
    ledger: &mut L,
    user_command: &SignedCommand,
) -> Result<SignedCommandApplied, String>
where
    L: LedgerIntf,
{
    let SignedCommand {
        payload: _,
        signer: signer_pk,
        signature: _,
    } = &user_command;
    let current_global_slot = txn_global_slot;

    let valid_until = user_command.valid_until();
    validate_time(&valid_until, current_global_slot)?;

    // Fee-payer information
    let fee_payer = user_command.fee_payer();
    let (fee_payer_location, fee_payer_account) =
        pay_fee(user_command, signer_pk, ledger, current_global_slot)?;

    if !fee_payer_account.has_permission_to_send() {
        return Err(TransactionFailure::UpdateNotPermittedBalance.to_string());
    }
    if !fee_payer_account.has_permission_to_increment_nonce() {
        return Err(TransactionFailure::UpdateNotPermittedNonce.to_string());
    }

    // Charge the fee. This must happen, whether or not the command itself
    // succeeds, to ensure that the network is compensated for processing this
    // command.
    set_with_location(ledger, &fee_payer_location, fee_payer_account.clone())?;

    let receiver = user_command.receiver();

    let mut reject_command = false;

    match compute_updates(
        constraint_constants,
        receiver,
        ledger,
        current_global_slot,
        user_command,
        &fee_payer,
        &fee_payer_account,
        &fee_payer_location,
        &mut reject_command,
    ) {
        Ok(Updates {
            located_accounts,
            applied_body,
        }) => {
            for (location, account) in located_accounts {
                set_with_location(ledger, &location, account)?;
            }

            Ok(SignedCommandApplied {
                common: signed_command_applied::Common {
                    user_command: WithStatus::<SignedCommand> {
                        data: user_command.clone(),
                        status: TransactionStatus::Applied,
                    },
                },
                body: applied_body,
            })
        }
        Err(failure) if !reject_command => Ok(SignedCommandApplied {
            common: signed_command_applied::Common {
                user_command: WithStatus::<SignedCommand> {
                    data: user_command.clone(),
                    status: TransactionStatus::Failed(vec![vec![failure]]),
                },
            },
            body: signed_command_applied::Body::Failed,
        }),
        Err(failure) => {
            // This case occurs when an exception is throwned in OCaml
            // <https://github.com/MinaProtocol/mina/blob/3753a8593cc1577bcf4da16620daf9946d88e8e5/src/lib/transaction_logic/mina_transaction_logic.ml#L964>
            assert!(reject_command);
            Err(failure.to_string())
        }
    }
}

pub fn apply_user_command<L>(
    constraint_constants: &ConstraintConstants,
    txn_state_view: &ProtocolStateView,
    txn_global_slot: &Slot,
    ledger: &mut L,
    user_command: &SignedCommand,
) -> Result<SignedCommandApplied, String>
where
    L: LedgerIntf,
{
    apply_user_command_unchecked(
        constraint_constants,
        txn_state_view,
        txn_global_slot,
        ledger,
        user_command,
    )
}

pub fn pay_fee<L, Loc>(
    user_command: &SignedCommand,
    signer_pk: &CompressedPubKey,
    ledger: &mut L,
    current_global_slot: &Slot,
) -> Result<(ExistingOrNew<Loc>, Box<Account>), String>
where
    L: LedgerIntf<Location = Loc>,
{
    let nonce = user_command.nonce();
    let fee_payer = user_command.fee_payer();
    let fee_token = user_command.fee_token();

    if &fee_payer.public_key != signer_pk {
        return Err("Cannot pay fees from a public key that did not sign the transaction".into());
    }

    if fee_token != TokenId::default() {
        return Err("Cannot create transactions with fee_token different from the default".into());
    }

    pay_fee_impl(
        &user_command.payload,
        nonce,
        fee_payer,
        user_command.fee(),
        ledger,
        current_global_slot,
    )
}

fn pay_fee_impl<L>(
    command: &SignedCommandPayload,
    nonce: Nonce,
    fee_payer: AccountId,
    fee: Fee,
    ledger: &mut L,
    current_global_slot: &Slot,
) -> Result<(ExistingOrNew<L::Location>, Box<Account>), String>
where
    L: LedgerIntf,
{
    // Fee-payer information
    let (location, mut account) = get_with_location(ledger, &fee_payer)?;

    if let ExistingOrNew::New = location {
        return Err("The fee-payer account does not exist".to_string());
    };

    let fee = Amount::of_fee(&fee);
    let balance = sub_amount(account.balance, fee)?;

    validate_nonces(nonce, account.nonce)?;
    let timing = validate_timing(&account, fee, current_global_slot)?;

    account.balance = balance;
    account.nonce = account.nonce.incr(); // TODO: Not sure if OCaml wraps
    account.receipt_chain_hash = cons_signed_command_payload(command, account.receipt_chain_hash);
    account.timing = timing;

    Ok((location, account))

    // in
    // ( location
    // , { account with
    //     balance
    //   ; nonce = Account.Nonce.succ account.nonce
    //   ; receipt_chain_hash =
    //       Receipt.Chain_hash.cons_signed_command_payload command
    //         account.receipt_chain_hash
    //   ; timing
    //   } )
}

pub mod transaction_union_payload {
    use ark_ff::PrimeField;
    use mina_hasher::{Hashable, ROInput as LegacyInput};
    use mina_signer::{NetworkId, PubKey, Signature};

    use crate::{
        decompress_pk,
        proofs::field::Boolean,
        scan_state::transaction_logic::signed_command::{PaymentPayload, StakeDelegationPayload},
    };

    use super::*;

    #[derive(Clone)]
    pub struct Common {
        pub fee: Fee,
        pub fee_token: TokenId,
        pub fee_payer_pk: CompressedPubKey,
        pub nonce: Nonce,
        pub valid_until: Slot,
        pub memo: Memo,
    }

    #[derive(Clone, Debug)]
    pub enum Tag {
        Payment = 0,
        StakeDelegation = 1,
        FeeTransfer = 2,
        Coinbase = 3,
    }

    impl Tag {
        pub fn is_user_command(&self) -> Boolean {
            match self {
                Tag::Payment | Tag::StakeDelegation => Boolean::True,
                Tag::FeeTransfer | Tag::Coinbase => Boolean::False,
            }
        }

        pub fn is_payment(&self) -> Boolean {
            match self {
                Tag::Payment => Boolean::True,
                Tag::FeeTransfer | Tag::Coinbase | Tag::StakeDelegation => Boolean::False,
            }
        }

        pub fn is_stake_delegation(&self) -> Boolean {
            match self {
                Tag::StakeDelegation => Boolean::True,
                Tag::FeeTransfer | Tag::Coinbase | Tag::Payment => Boolean::False,
            }
        }

        pub fn is_fee_transfer(&self) -> Boolean {
            match self {
                Tag::FeeTransfer => Boolean::True,
                Tag::StakeDelegation | Tag::Coinbase | Tag::Payment => Boolean::False,
            }
        }

        pub fn is_coinbase(&self) -> Boolean {
            match self {
                Tag::Coinbase => Boolean::True,
                Tag::StakeDelegation | Tag::FeeTransfer | Tag::Payment => Boolean::False,
            }
        }

        pub fn to_bits(&self) -> [bool; 3] {
            let tag = self.clone() as u8;
            let mut bits = [false; 3];
            for (index, bit) in [4, 2, 1].iter().enumerate() {
                bits[index] = tag & bit != 0;
            }
            bits
        }

        pub fn to_untagged_bits(&self) -> [bool; 5] {
            let mut is_payment = false;
            let mut is_stake_delegation = false;
            let mut is_fee_transfer = false;
            let mut is_coinbase = false;
            let mut is_user_command = false;

            match self {
                Tag::Payment => {
                    is_payment = true;
                    is_user_command = true;
                }
                Tag::StakeDelegation => {
                    is_stake_delegation = true;
                    is_user_command = true;
                }
                Tag::FeeTransfer => is_fee_transfer = true,
                Tag::Coinbase => is_coinbase = true,
            }

            [
                is_payment,
                is_stake_delegation,
                is_fee_transfer,
                is_coinbase,
                is_user_command,
            ]
        }
    }

    #[derive(Clone)]
    pub struct Body {
        pub tag: Tag,
        pub source_pk: CompressedPubKey,
        pub receiver_pk: CompressedPubKey,
        pub token_id: TokenId,
        pub amount: Amount,
    }

    #[derive(Clone)]
    pub struct TransactionUnionPayload {
        pub common: Common,
        pub body: Body,
    }

    impl Hashable for TransactionUnionPayload {
        type D = NetworkId;

        fn to_roinput(&self) -> LegacyInput {
            /*
                Payment transactions only use the default token-id value 1.
                The old transaction format encoded the token-id as an u64,
                however zkApps encode the token-id as a Fp.

                For testing/fuzzing purposes we want the ability to encode
                arbitrary values different from the default token-id, for this
                we will extract the LS u64 of the token-id.
            */
            let fee_token_id = self.common.fee_token.0.into_bigint().0[0];
            let token_id = self.body.token_id.0.into_bigint().0[0];

            let mut roi = LegacyInput::new()
                .append_field(self.common.fee_payer_pk.x)
                .append_field(self.body.source_pk.x)
                .append_field(self.body.receiver_pk.x)
                .append_u64(self.common.fee.as_u64())
                .append_u64(fee_token_id)
                .append_bool(self.common.fee_payer_pk.is_odd)
                .append_u32(self.common.nonce.as_u32())
                .append_u32(self.common.valid_until.as_u32())
                .append_bytes(&self.common.memo.0);

            let tag = self.body.tag.clone() as u8;
            for bit in [4, 2, 1] {
                roi = roi.append_bool(tag & bit != 0);
            }

            roi.append_bool(self.body.source_pk.is_odd)
                .append_bool(self.body.receiver_pk.is_odd)
                .append_u64(token_id)
                .append_u64(self.body.amount.as_u64())
                .append_bool(false) // Used to be `self.body.token_locked`
        }

        // TODO: this is unused, is it needed?
        fn domain_string(network_id: NetworkId) -> Option<String> {
            // Domain strings must have length <= 20
            match network_id {
                NetworkId::MAINNET => mina_core::network::mainnet::SIGNATURE_PREFIX,
                NetworkId::TESTNET => mina_core::network::devnet::SIGNATURE_PREFIX,
            }
            .to_string()
            .into()
        }
    }

    impl TransactionUnionPayload {
        pub fn of_user_command_payload(payload: &SignedCommandPayload) -> Self {
            use signed_command::Body::{Payment, StakeDelegation};

            Self {
                common: Common {
                    fee: payload.common.fee,
                    fee_token: TokenId::default(),
                    fee_payer_pk: payload.common.fee_payer_pk.clone(),
                    nonce: payload.common.nonce,
                    valid_until: payload.common.valid_until,
                    memo: payload.common.memo.clone(),
                },
                body: match &payload.body {
                    Payment(PaymentPayload {
                        receiver_pk,
                        amount,
                    }) => Body {
                        tag: Tag::Payment,
                        source_pk: payload.common.fee_payer_pk.clone(),
                        receiver_pk: receiver_pk.clone(),
                        token_id: TokenId::default(),
                        amount: *amount,
                    },
                    StakeDelegation(StakeDelegationPayload::SetDelegate { new_delegate }) => Body {
                        tag: Tag::StakeDelegation,
                        source_pk: payload.common.fee_payer_pk.clone(),
                        receiver_pk: new_delegate.clone(),
                        token_id: TokenId::default(),
                        amount: Amount::zero(),
                    },
                },
            }
        }

        /// <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/transaction_union_payload.ml#L309>
        pub fn to_input_legacy(&self) -> ::poseidon::hash::legacy::Inputs<Fp> {
            let mut roi = ::poseidon::hash::legacy::Inputs::new();

            // Self.common
            {
                roi.append_u64(self.common.fee.0);

                // TokenId.default
                // <https://github.com/MinaProtocol/mina/blob/2ee6e004ba8c6a0541056076aab22ea162f7eb3a/src/lib/mina_base/signed_command_payload.ml#L19>
                roi.append_bool(true);
                for _ in 0..63 {
                    roi.append_bool(false);
                }

                // fee_payer_pk
                roi.append_field(self.common.fee_payer_pk.x);
                roi.append_bool(self.common.fee_payer_pk.is_odd);

                // nonce
                roi.append_u32(self.common.nonce.0);

                // valid_until
                roi.append_u32(self.common.valid_until.0);

                // memo
                roi.append_bytes(&self.common.memo.0);
            }

            // Self.body
            {
                // tag
                let tag = self.body.tag.clone() as u8;
                for bit in [4, 2, 1] {
                    roi.append_bool(tag & bit != 0);
                }

                // source_pk
                roi.append_field(self.body.source_pk.x);
                roi.append_bool(self.body.source_pk.is_odd);

                // receiver_pk
                roi.append_field(self.body.receiver_pk.x);
                roi.append_bool(self.body.receiver_pk.is_odd);

                // default token_id
                roi.append_u64(1);

                // amount
                roi.append_u64(self.body.amount.0);

                // token_locked
                roi.append_bool(false);
            }

            roi
        }
    }

    pub struct TransactionUnion {
        pub payload: TransactionUnionPayload,
        pub signer: PubKey,
        pub signature: Signature,
    }

    impl TransactionUnion {
        /// For SNARK purposes, we inject [Transaction.t]s into a single-variant 'tagged-union' record capable of
        /// representing all the variants. We interpret the fields of this union in different ways depending on
        /// the value of the [payload.body.tag] field, which represents which variant of [Transaction.t] the value
        /// corresponds to.
        ///
        /// Sometimes we interpret fields in surprising ways in different cases to save as much space in the SNARK as possible (e.g.,
        /// [payload.body.public_key] is interpreted as the recipient of a payment, the new delegate of a stake
        /// delegation command, and a fee transfer recipient for both coinbases and fee-transfers.
        pub fn of_transaction(tx: &Transaction) -> Self {
            match tx {
                Transaction::Command(cmd) => {
                    let UserCommand::SignedCommand(cmd) = cmd else {
                        unreachable!();
                    };

                    let SignedCommand {
                        payload,
                        signer,
                        signature,
                    } = cmd.as_ref();

                    TransactionUnion {
                        payload: TransactionUnionPayload::of_user_command_payload(payload),
                        signer: decompress_pk(signer).unwrap(),
                        signature: signature.clone(),
                    }
                }
                Transaction::Coinbase(Coinbase {
                    receiver,
                    amount,
                    fee_transfer,
                }) => {
                    let CoinbaseFeeTransfer {
                        receiver_pk: other_pk,
                        fee: other_amount,
                    } = fee_transfer.clone().unwrap_or_else(|| {
                        CoinbaseFeeTransfer::create(receiver.clone(), Fee::zero())
                    });

                    let signer = decompress_pk(&other_pk).unwrap();
                    let payload = TransactionUnionPayload {
                        common: Common {
                            fee: other_amount,
                            fee_token: TokenId::default(),
                            fee_payer_pk: other_pk.clone(),
                            nonce: Nonce::zero(),
                            valid_until: Slot::max(),
                            memo: Memo::empty(),
                        },
                        body: Body {
                            source_pk: other_pk,
                            receiver_pk: receiver.clone(),
                            token_id: TokenId::default(),
                            amount: *amount,
                            tag: Tag::Coinbase,
                        },
                    };

                    TransactionUnion {
                        payload,
                        signer,
                        signature: Signature::dummy(),
                    }
                }
                Transaction::FeeTransfer(tr) => {
                    let two = |SingleFeeTransfer {
                                   receiver_pk: pk1,
                                   fee: fee1,
                                   fee_token,
                               },
                               SingleFeeTransfer {
                                   receiver_pk: pk2,
                                   fee: fee2,
                                   fee_token: token_id,
                               }| {
                        let signer = decompress_pk(&pk2).unwrap();
                        let payload = TransactionUnionPayload {
                            common: Common {
                                fee: fee2,
                                fee_token,
                                fee_payer_pk: pk2.clone(),
                                nonce: Nonce::zero(),
                                valid_until: Slot::max(),
                                memo: Memo::empty(),
                            },
                            body: Body {
                                source_pk: pk2,
                                receiver_pk: pk1,
                                token_id,
                                amount: Amount::of_fee(&fee1),
                                tag: Tag::FeeTransfer,
                            },
                        };

                        TransactionUnion {
                            payload,
                            signer,
                            signature: Signature::dummy(),
                        }
                    };

                    match tr.0.clone() {
                        OneOrTwo::One(t) => {
                            let other = SingleFeeTransfer::create(
                                t.receiver_pk.clone(),
                                Fee::zero(),
                                t.fee_token.clone(),
                            );
                            two(t, other)
                        }
                        OneOrTwo::Two((t1, t2)) => two(t1, t2),
                    }
                }
            }
        }
    }
}

/// Returns the new `receipt_chain_hash`
pub fn cons_signed_command_payload(
    command_payload: &SignedCommandPayload,
    last_receipt_chain_hash: ReceiptChainHash,
) -> ReceiptChainHash {
    // Note: Not sure why they use the legacy way of hashing here

    use poseidon::hash::legacy;

    let ReceiptChainHash(last_receipt_chain_hash) = last_receipt_chain_hash;
    let union = TransactionUnionPayload::of_user_command_payload(command_payload);

    let mut inputs = union.to_input_legacy();
    inputs.append_field(last_receipt_chain_hash);
    let hash = legacy::hash_with_kimchi(&legacy::params::CODA_RECEIPT_UC, &inputs.to_fields());

    ReceiptChainHash(hash)
}

/// Returns the new `receipt_chain_hash`
pub fn checked_cons_signed_command_payload(
    payload: &TransactionUnionPayload,
    last_receipt_chain_hash: ReceiptChainHash,
    w: &mut Witness<Fp>,
) -> ReceiptChainHash {
    use crate::proofs::transaction::{
        legacy_input::CheckedLegacyInput, transaction_snark::checked_legacy_hash,
    };
    use poseidon::hash::legacy;

    let mut inputs = payload.to_checked_legacy_input_owned(w);
    inputs.append_field(last_receipt_chain_hash.0);

    let receipt_chain_hash = checked_legacy_hash(&legacy::params::CODA_RECEIPT_UC, inputs, w);

    ReceiptChainHash(receipt_chain_hash)
}

/// prepend account_update index computed by Zkapp_command_logic.apply
///
/// <https://github.com/MinaProtocol/mina/blob/3753a8593cc1577bcf4da16620daf9946d88e8e5/src/lib/mina_base/receipt.ml#L66>
pub fn cons_zkapp_command_commitment(
    index: Index,
    e: ZkAppCommandElt,
    receipt_hash: &ReceiptChainHash,
) -> ReceiptChainHash {
    let ZkAppCommandElt::ZkAppCommandCommitment(x) = e;

    let mut inputs = Inputs::new();

    inputs.append(&index);
    inputs.append_field(x.0);
    inputs.append(receipt_hash);

    ReceiptChainHash(hash_with_kimchi(&CODA_RECEIPT_UC, &inputs.to_fields()))
}

fn validate_nonces(txn_nonce: Nonce, account_nonce: Nonce) -> Result<(), String> {
    if account_nonce == txn_nonce {
        return Ok(());
    }

    Err(format!(
        "Nonce in account {:?} different from nonce in transaction {:?}",
        account_nonce, txn_nonce,
    ))
}

pub fn validate_timing(
    account: &Account,
    txn_amount: Amount,
    txn_global_slot: &Slot,
) -> Result<Timing, String> {
    let (timing, _) = validate_timing_with_min_balance(account, txn_amount, txn_global_slot)?;

    Ok(timing)
}

pub fn account_check_timing(
    txn_global_slot: &Slot,
    account: &Account,
) -> (TimingValidation<bool>, Timing) {
    let (invalid_timing, timing, _) =
        validate_timing_with_min_balance_impl(account, Amount::from_u64(0), txn_global_slot);
    // TODO: In OCaml the returned Timing is actually converted to None/Some(fields of Timing structure)
    (invalid_timing, timing)
}

fn validate_timing_with_min_balance(
    account: &Account,
    txn_amount: Amount,
    txn_global_slot: &Slot,
) -> Result<(Timing, MinBalance), String> {
    use TimingValidation::*;

    let (possibly_error, timing, min_balance) =
        validate_timing_with_min_balance_impl(account, txn_amount, txn_global_slot);

    match possibly_error {
        InsufficientBalance(true) => Err(format!(
            "For timed account, the requested transaction for amount {:?} \
             at global slot {:?}, the balance {:?} \
             is insufficient",
            txn_amount, txn_global_slot, account.balance
        )),
        InvalidTiming(true) => Err(format!(
            "For timed account {}, the requested transaction for amount {:?} \
             at global slot {:?}, applying the transaction would put the \
             balance below the calculated minimum balance of {:?}",
            account.public_key.into_address(),
            txn_amount,
            txn_global_slot,
            min_balance.0
        )),
        InsufficientBalance(false) => {
            panic!("Broken invariant in validate_timing_with_min_balance'")
        }
        InvalidTiming(false) => Ok((timing, min_balance)),
    }
}

pub fn timing_error_to_user_command_status(
    timing_result: Result<Timing, String>,
) -> Result<Timing, TransactionFailure> {
    match timing_result {
        Ok(timing) => Ok(timing),
        Err(err_str) => {
            /*
                HACK: we are matching over the full error string instead
                of including an extra tag string to the Err variant
            */
            if err_str.contains("minimum balance") {
                return Err(TransactionFailure::SourceMinimumBalanceViolation);
            }

            if err_str.contains("is insufficient") {
                return Err(TransactionFailure::SourceInsufficientBalance);
            }

            panic!("Unexpected timed account validation error")
        }
    }
}

pub enum TimingValidation<B> {
    InsufficientBalance(B),
    InvalidTiming(B),
}

#[derive(Debug)]
struct MinBalance(Balance);

fn validate_timing_with_min_balance_impl(
    account: &Account,
    txn_amount: Amount,
    txn_global_slot: &Slot,
) -> (TimingValidation<bool>, Timing, MinBalance) {
    use crate::Timing::*;
    use TimingValidation::*;

    match &account.timing {
        Untimed => {
            // no time restrictions
            match account.balance.sub_amount(txn_amount) {
                None => (
                    InsufficientBalance(true),
                    Untimed,
                    MinBalance(Balance::zero()),
                ),
                Some(_) => (InvalidTiming(false), Untimed, MinBalance(Balance::zero())),
            }
        }
        Timed {
            initial_minimum_balance,
            ..
        } => {
            let account_balance = account.balance;

            let (invalid_balance, invalid_timing, curr_min_balance) =
                match account_balance.sub_amount(txn_amount) {
                    None => {
                        // NB: The [initial_minimum_balance] here is the incorrect value,
                        // but:
                        // * we don't use it anywhere in this error case; and
                        // * we don't want to waste time computing it if it will be unused.
                        (true, false, *initial_minimum_balance)
                    }
                    Some(proposed_new_balance) => {
                        let curr_min_balance = account.min_balance_at_slot(*txn_global_slot);

                        if proposed_new_balance < curr_min_balance {
                            (false, true, curr_min_balance)
                        } else {
                            (false, false, curr_min_balance)
                        }
                    }
                };

            // once the calculated minimum balance becomes zero, the account becomes untimed
            let possibly_error = if invalid_balance {
                InsufficientBalance(invalid_balance)
            } else {
                InvalidTiming(invalid_timing)
            };

            if curr_min_balance > Balance::zero() {
                (
                    possibly_error,
                    account.timing.clone(),
                    MinBalance(curr_min_balance),
                )
            } else {
                (possibly_error, Untimed, MinBalance(Balance::zero()))
            }
        }
    }
}

fn sub_amount(balance: Balance, amount: Amount) -> Result<Balance, String> {
    balance
        .sub_amount(amount)
        .ok_or_else(|| "insufficient funds".to_string())
}

fn add_amount(balance: Balance, amount: Amount) -> Result<Balance, String> {
    balance
        .add_amount(amount)
        .ok_or_else(|| "overflow".to_string())
}

#[derive(Clone, Debug)]
pub enum ExistingOrNew<Loc> {
    Existing(Loc),
    New,
}

fn get_with_location<L>(
    ledger: &mut L,
    account_id: &AccountId,
) -> Result<(ExistingOrNew<L::Location>, Box<Account>), String>
where
    L: LedgerIntf,
{
    match ledger.location_of_account(account_id) {
        Some(location) => match ledger.get(&location) {
            Some(account) => Ok((ExistingOrNew::Existing(location), account)),
            None => panic!("Ledger location with no account"),
        },
        None => Ok((
            ExistingOrNew::New,
            Box::new(Account::create_with(account_id.clone(), Balance::zero())),
        )),
    }
}

pub fn get_account<L>(
    ledger: &mut L,
    account_id: AccountId,
) -> (Box<Account>, ExistingOrNew<L::Location>)
where
    L: LedgerIntf,
{
    let (loc, account) = get_with_location(ledger, &account_id).unwrap();
    (account, loc)
}

pub fn set_account<'a, L>(
    l: &'a mut L,
    (a, loc): (Box<Account>, &ExistingOrNew<L::Location>),
) -> &'a mut L
where
    L: LedgerIntf,
{
    set_with_location(l, loc, a).unwrap();
    l
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use o1_utils::FieldHelpers;

    #[cfg(target_family = "wasm")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use super::{
        signed_command::{Body, Common, PaymentPayload},
        *,
    };

    fn pub_key(address: &str) -> CompressedPubKey {
        mina_signer::PubKey::from_address(address)
            .unwrap()
            .into_compressed()
    }

    #[test]
    fn test_hash_empty_event() {
        // Same value than OCaml
        const EXPECTED: &str =
            "6963060754718463299978089777716994949151371320681588566338620419071140958308";

        let event = zkapp_command::Event::empty();
        assert_eq!(event.hash(), Fp::from_str(EXPECTED).unwrap());
    }

    /// Test using same values as here:
    /// <https://github.com/MinaProtocol/mina/blob/3a78f0e0c1343d14e2729c8b00205baa2ec70c93/src/lib/mina_base/receipt.ml#L136>
    #[test]
    fn test_cons_receipt_hash_ocaml() {
        let from = pub_key("B62qr71UxuyKpkSKYceCPsjw14nuaeLwWKZdMqaBMPber5AAF6nkowS");
        let to = pub_key("B62qnvGVnU7FXdy8GdkxL7yciZ8KattyCdq5J6mzo5NCxjgQPjL7BTH");

        let common = Common {
            fee: Fee::from_u64(9758327274353182341),
            fee_payer_pk: from,
            nonce: Nonce::from_u32(1609569868),
            valid_until: Slot::from_u32(2127252111),
            memo: Memo([
                1, 32, 101, 26, 225, 104, 115, 118, 55, 102, 76, 118, 108, 78, 114, 50, 0, 115,
                110, 108, 53, 75, 109, 112, 50, 110, 88, 97, 76, 66, 76, 81, 235, 79,
            ]),
        };

        let body = Body::Payment(PaymentPayload {
            receiver_pk: to,
            amount: Amount::from_u64(1155659205107036493),
        });

        let tx = SignedCommandPayload { common, body };

        let prev = "4918218371695029984164006552208340844155171097348169027410983585063546229555";
        let prev_receipt_chain_hash = ReceiptChainHash(Fp::from_str(prev).unwrap());

        let next = "19078048535981853335308913493724081578728104896524544653528728307378106007337";
        let next_receipt_chain_hash = ReceiptChainHash(Fp::from_str(next).unwrap());

        let result = cons_signed_command_payload(&tx, prev_receipt_chain_hash);
        assert_eq!(result, next_receipt_chain_hash);
    }

    #[test]
    fn test_receipt_hash_update() {
        let from = pub_key("B62qmnY6m4c6bdgSPnQGZriSaj9vuSjsfh6qkveGTsFX3yGA5ywRaja");
        let to = pub_key("B62qjVQLxt9nYMWGn45mkgwYfcz8e8jvjNCBo11VKJb7vxDNwv5QLPS");

        let common = Common {
            fee: Fee::from_u64(14500000),
            fee_payer_pk: from,
            nonce: Nonce::from_u32(15),
            valid_until: Slot::from_u32(-1i32 as u32),
            memo: Memo([
                1, 7, 84, 104, 101, 32, 49, 48, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
        };

        let body = Body::Payment(PaymentPayload {
            receiver_pk: to,
            amount: Amount::from_u64(2354000000),
        });

        let tx = SignedCommandPayload { common, body };

        let mut prev =
            hex::decode("09ac04c9965b885acfc9c54141dbecfc63b2394a4532ea2c598d086b894bfb14")
                .unwrap();
        prev.reverse();
        let prev_receipt_chain_hash = ReceiptChainHash(Fp::from_bytes(&prev).unwrap());

        let mut next =
            hex::decode("3ecaa73739df77549a2f92f7decf822562d0593373cff1e480bb24b4c87dc8f0")
                .unwrap();
        next.reverse();
        let next_receipt_chain_hash = ReceiptChainHash(Fp::from_bytes(&next).unwrap());

        let result = cons_signed_command_payload(&tx, prev_receipt_chain_hash);
        assert_eq!(result, next_receipt_chain_hash);
    }
}
