use core::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RpcMethod {
    DeregisterNode,
    GetAccountInfo,
    GetBalance,
    GetBlock,
    GetBlockHeight,
    GetBlockProduction,
    GetBlocks,
    GetBlocksWithLimit,
    GetBlockTime,
    GetClusterNodes,
    GetEpochInfo,
    GetEpochSchedule,
    GetFeeCalculatorForBlockhash,
    GetFeeRateGovernor,
    GetFees,
    GetFirstAvailableBlock,
    GetGenesisHash,
    GetHealth,
    GetIdentity,
    GetInflationGovernor,
    GetInflationRate,
    GetInflationReward,
    GetLargestAccounts,
    GetLeaderSchedule,
    GetMaxRetransmitSlot,
    GetMaxShredInsertSlot,
    GetMinimumBalanceForRentExemption,
    GetMultipleAccounts,
    GetProgramAccounts,
    GetRecentBlockhash,
    GetRecentPerformanceSamples,
    GetSnapshotSlot,
    GetSignaturesForAddress,
    GetSignatureStatuses,
    GetSlot,
    GetSlotLeader,
    GetSlotLeaders,
    GetStorageTurn,
    GetStorageTurnRate,
    GetSlotsPerSegment,
    GetStakeActivation,
    GetStoragePubkeysForSlot,
    GetSupply,
    GetTokenAccountBalance,
    GetTokenAccountsByDelegate,
    GetTokenAccountsByOwner,
    GetTokenSupply,
    GetTransaction,
    GetTransactionCount,
    GetVersion,
    GetVoteAccounts,
    MinimumLedgerSlot,
    RegisterNode,
    RequestAirdrop,
    SendTransaction,
    SimulateTransaction,
    SignVote,
    Custom { method: &'static str },
}

impl RpcMethod {
    pub fn to_camel_case(self) -> &'static str {
        match self {
            RpcMethod::DeregisterNode => "deregisterNode",
            RpcMethod::GetAccountInfo => "getAccountInfo",
            RpcMethod::GetBalance => "getBalance",
            RpcMethod::GetBlock => "getBlock",
            RpcMethod::GetBlockHeight => "getBlockHeight",
            RpcMethod::GetBlockProduction => "getBlockProduction",
            RpcMethod::GetBlocks => "getBlocks",
            RpcMethod::GetBlocksWithLimit => "getBlocksWithLimit",
            RpcMethod::GetBlockTime => "getBlockTime",
            RpcMethod::GetClusterNodes => "getClusterNodes",
            RpcMethod::GetEpochInfo => "getEpochInfo",
            RpcMethod::GetEpochSchedule => "getEpochSchedule",
            RpcMethod::GetFeeCalculatorForBlockhash => "getFeeCalculatorForBlockhash",
            RpcMethod::GetFeeRateGovernor => "getFeeRateGovernor",
            RpcMethod::GetFees => "getFees",
            RpcMethod::GetFirstAvailableBlock => "getFirstAvailableBlock",
            RpcMethod::GetGenesisHash => "getGenesisHash",
            RpcMethod::GetHealth => "getHealth",
            RpcMethod::GetIdentity => "getIdentity",
            RpcMethod::GetInflationGovernor => "getInflationGovernor",
            RpcMethod::GetInflationRate => "getInflationRate",
            RpcMethod::GetInflationReward => "getInflationReward",
            RpcMethod::GetLargestAccounts => "getLargestAccounts",
            RpcMethod::GetLeaderSchedule => "getLeaderSchedule",
            RpcMethod::GetMaxRetransmitSlot => "getMaxRetransmitSlot",
            RpcMethod::GetMaxShredInsertSlot => "getMaxShredInsertSlot",
            RpcMethod::GetMinimumBalanceForRentExemption => "getMinimumBalanceForRentExemption",
            RpcMethod::GetMultipleAccounts => "getMultipleAccounts",
            RpcMethod::GetProgramAccounts => "getProgramAccounts",
            RpcMethod::GetRecentBlockhash => "getRecentBlockhash",
            RpcMethod::GetRecentPerformanceSamples => "getRecentPerformanceSamples",
            RpcMethod::GetSnapshotSlot => "getSnapshotSlot",
            RpcMethod::GetSignaturesForAddress => "getSignaturesForAddress",
            RpcMethod::GetSignatureStatuses => "getSignatureStatuses",
            RpcMethod::GetSlot => "getSlot",
            RpcMethod::GetSlotLeader => "getSlotLeader",
            RpcMethod::GetSlotLeaders => "getSlotLeaders",
            RpcMethod::GetStakeActivation => "getStakeActivation",
            RpcMethod::GetStorageTurn => "getStorageTurn",
            RpcMethod::GetStorageTurnRate => "getStorageTurnRate",
            RpcMethod::GetSlotsPerSegment => "getSlotsPerSegment",
            RpcMethod::GetStoragePubkeysForSlot => "getStoragePubkeysForSlot",
            RpcMethod::GetSupply => "getSupply",
            RpcMethod::GetTokenAccountBalance => "getTokenAccountBalance",
            RpcMethod::GetTokenAccountsByDelegate => "getTokenAccountsByDelegate",
            RpcMethod::GetTokenAccountsByOwner => "getTokenAccountsByOwner",
            RpcMethod::GetTokenSupply => "getTokenSupply",
            RpcMethod::GetTransaction => "getTransaction",
            RpcMethod::GetTransactionCount => "getTransactionCount",
            RpcMethod::GetVersion => "getVersion",
            RpcMethod::GetVoteAccounts => "getVoteAccounts",
            RpcMethod::MinimumLedgerSlot => "minimumLedgerSlot",
            RpcMethod::RegisterNode => "registerNode",
            RpcMethod::RequestAirdrop => "requestAirdrop",
            RpcMethod::SendTransaction => "sendTransaction",
            RpcMethod::SimulateTransaction => "simulateTransaction",
            RpcMethod::SignVote => "signVote",
            RpcMethod::Custom { method } => method,
        }
    }
}

impl fmt::Display for RpcMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method = match self {
            RpcMethod::DeregisterNode => "deregisterNode",
            RpcMethod::GetAccountInfo => "getAccountInfo",
            RpcMethod::GetBalance => "getBalance",
            RpcMethod::GetBlock => "getBlock",
            RpcMethod::GetBlockHeight => "getBlockHeight",
            RpcMethod::GetBlockProduction => "getBlockProduction",
            RpcMethod::GetBlocks => "getBlocks",
            RpcMethod::GetBlocksWithLimit => "getBlocksWithLimit",
            RpcMethod::GetBlockTime => "getBlockTime",
            RpcMethod::GetClusterNodes => "getClusterNodes",
            RpcMethod::GetEpochInfo => "getEpochInfo",
            RpcMethod::GetEpochSchedule => "getEpochSchedule",
            RpcMethod::GetFeeCalculatorForBlockhash => "getFeeCalculatorForBlockhash",
            RpcMethod::GetFeeRateGovernor => "getFeeRateGovernor",
            RpcMethod::GetFees => "getFees",
            RpcMethod::GetFirstAvailableBlock => "getFirstAvailableBlock",
            RpcMethod::GetGenesisHash => "getGenesisHash",
            RpcMethod::GetHealth => "getHealth",
            RpcMethod::GetIdentity => "getIdentity",
            RpcMethod::GetInflationGovernor => "getInflationGovernor",
            RpcMethod::GetInflationRate => "getInflationRate",
            RpcMethod::GetInflationReward => "getInflationReward",
            RpcMethod::GetLargestAccounts => "getLargestAccounts",
            RpcMethod::GetLeaderSchedule => "getLeaderSchedule",
            RpcMethod::GetMaxRetransmitSlot => "getMaxRetransmitSlot",
            RpcMethod::GetMaxShredInsertSlot => "getMaxShredInsertSlot",
            RpcMethod::GetMinimumBalanceForRentExemption => "getMinimumBalanceForRentExemption",
            RpcMethod::GetMultipleAccounts => "getMultipleAccounts",
            RpcMethod::GetProgramAccounts => "getProgramAccounts",
            RpcMethod::GetRecentBlockhash => "getRecentBlockhash",
            RpcMethod::GetRecentPerformanceSamples => "getRecentPerformanceSamples",
            RpcMethod::GetSnapshotSlot => "getSnapshotSlot",
            RpcMethod::GetSignaturesForAddress => "getSignaturesForAddress",
            RpcMethod::GetSignatureStatuses => "getSignatureStatuses",
            RpcMethod::GetSlot => "getSlot",
            RpcMethod::GetSlotLeader => "getSlotLeader",
            RpcMethod::GetSlotLeaders => "getSlotLeaders",
            RpcMethod::GetStakeActivation => "getStakeActivation",
            RpcMethod::GetStorageTurn => "getStorageTurn",
            RpcMethod::GetStorageTurnRate => "getStorageTurnRate",
            RpcMethod::GetSlotsPerSegment => "getSlotsPerSegment",
            RpcMethod::GetStoragePubkeysForSlot => "getStoragePubkeysForSlot",
            RpcMethod::GetSupply => "getSupply",
            RpcMethod::GetTokenAccountBalance => "getTokenAccountBalance",
            RpcMethod::GetTokenAccountsByDelegate => "getTokenAccountsByDelegate",
            RpcMethod::GetTokenAccountsByOwner => "getTokenAccountsByOwner",
            RpcMethod::GetTokenSupply => "getTokenSupply",
            RpcMethod::GetTransaction => "getTransaction",
            RpcMethod::GetTransactionCount => "getTransactionCount",
            RpcMethod::GetVersion => "getVersion",
            RpcMethod::GetVoteAccounts => "getVoteAccounts",
            RpcMethod::MinimumLedgerSlot => "minimumLedgerSlot",
            RpcMethod::RegisterNode => "registerNode",
            RpcMethod::RequestAirdrop => "requestAirdrop",
            RpcMethod::SendTransaction => "sendTransaction",
            RpcMethod::SimulateTransaction => "simulateTransaction",
            RpcMethod::SignVote => "signVote",
            RpcMethod::Custom { method } => method,
        };

        write!(f, "{}", method)
    }
}
