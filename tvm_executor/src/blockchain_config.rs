// Copyright (C) 2019-2023 EverX. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use tvm_block::ConfigParam18;
use tvm_block::ConfigParams;
use tvm_block::FundamentalSmcAddresses;
use tvm_block::GasLimitsPrices;
use tvm_block::GlobalCapabilities;
use tvm_block::Grams;
use tvm_block::MsgAddressInt;
use tvm_block::MsgForwardPrices;
use tvm_block::StorageInfo;
use tvm_block::StoragePrices;
use tvm_block::StorageUsedShort;
use tvm_types::Cell;
use tvm_types::Result;
use tvm_types::UInt256;

pub const VERSION_BLOCK_REVERT_MESSAGES_WITH_ANYCAST_ADDRESSES: u32 = 8;
pub const VERSION_BLOCK_NEW_CALCULATION_BOUNCED_STORAGE: u32 = 30;

pub(crate) trait TONDefaultConfig {
    /// Get default value for masterchain
    fn default_mc() -> Self;
    /// Get default value for workchains
    fn default_wc() -> Self;
}

impl TONDefaultConfig for MsgForwardPrices {
    fn default_mc() -> Self {
        MsgForwardPrices {
            lump_price: 10000000,
            bit_price: 655360000,
            cell_price: 65536000000,
            ihr_price_factor: 98304,
            first_frac: 21845,
            next_frac: 21845,
        }
    }

    fn default_wc() -> Self {
        MsgForwardPrices {
            lump_price: 1000000,
            bit_price: 65536000,
            cell_price: 6553600000,
            ihr_price_factor: 98304,
            first_frac: 21845,
            next_frac: 21845,
        }
    }
}

pub trait CalcMsgFwdFees {
    fn fwd_fee(&self, msg_cell: &Cell) -> u128;
    fn ihr_fee_checked(&self, fwd_fee: &Grams) -> Result<Grams>;
    fn mine_fee_checked(&self, fwd_fee: &Grams) -> Result<Grams>;
    fn next_fee_checked(&self, fwd_fee: &Grams) -> Result<Grams>;
}

impl CalcMsgFwdFees for MsgForwardPrices {
    /// Calculate message forward fee
    /// Forward fee is calculated according to the following formula:
    /// `fwd_fee = (lump_price + ceil((bit_price * msg.bits + cell_price *
    /// msg.cells)/2^16))`. `msg.bits` and `msg.cells` are calculated from
    /// message represented as tree of cells. Root cell is not counted.
    fn fwd_fee(&self, msg_cell: &Cell) -> u128 {
        let mut storage = StorageUsedShort::default();
        storage.append(msg_cell);
        let mut bits = storage.bits() as u128;
        let mut cells = storage.cells() as u128;
        bits -= msg_cell.bit_length() as u128;
        cells -= 1;

        // All prices except `lump_price` are presented in `0xffff * price` form.
        // It is needed because `ihr_factor`, `first_frac` and `next_frac` are not
        // integer values but calculations are performed in integers, so prices
        // are multiplied to some big number (0xffff) and fee calculation uses
        // such values. At the end result is divided by 0xffff with ceil
        // rounding to obtain nanograms (add 0xffff and then `>> 16`)
        self.lump_price as u128
            + ((cells * self.cell_price as u128 + bits * self.bit_price as u128 + 0xffff) >> 16)
    }

    /// Calculate message IHR fee
    /// IHR fee is calculated as `(msg_forward_fee * ihr_factor) >> 16`
    fn ihr_fee_checked(&self, fwd_fee: &Grams) -> Result<Grams> {
        Grams::new((fwd_fee.as_u128() * self.ihr_price_factor as u128) >> 16)
    }

    /// Calculate mine part of forward fee
    /// Forward fee for internal message is splited to `int_msg_mine_fee` and
    /// `int_msg_remain_fee`: `msg_forward_fee = int_msg_mine_fee +
    /// int_msg_remain_fee` `int_msg_mine_fee` is a part of transaction
    /// `total_fees` and will go validators of account's shard
    /// `int_msg_remain_fee` is placed in header of internal message and will go
    /// to validators of shard to which message destination address is
    /// belong.
    fn mine_fee_checked(&self, fwd_fee: &Grams) -> Result<Grams> {
        Grams::new((fwd_fee.as_u128() * self.first_frac as u128) >> 16)
    }

    fn next_fee_checked(&self, fwd_fee: &Grams) -> Result<Grams> {
        Grams::new((fwd_fee.as_u128() * self.next_frac as u128) >> 16)
    }
}

#[derive(Clone)]
pub struct AccStoragePrices {
    prices: Vec<StoragePrices>,
}

impl Default for AccStoragePrices {
    fn default() -> Self {
        AccStoragePrices {
            prices: vec![StoragePrices {
                utime_since: 0,
                bit_price_ps: 1,
                cell_price_ps: 500,
                mc_bit_price_ps: 1000,
                mc_cell_price_ps: 500000,
            }],
        }
    }
}

impl AccStoragePrices {
    /// Calculate storage fee for provided data
    pub fn calc_storage_fee(
        &self,
        cells: u128,
        bits: u128,
        mut last_paid: u32,
        now: u32,
        is_masterchain: bool,
    ) -> u128 {
        if now <= last_paid
            || last_paid == 0
            || self.prices.is_empty()
            || now <= self.prices[0].utime_since
        {
            return 0;
        }
        let mut fee = 0u128;
        // storage prices config contains prices array for some time intervals
        // to calculate account storage fee we need to sum fees for all intervals since
        // last storage fee pay calculated by formula `(cells * cell_price +
        // bits * bits_price) * interval`
        for i in 0..self.prices.len() {
            let prices = &self.prices[i];
            let end = if i < self.prices.len() - 1 { self.prices[i + 1].utime_since } else { now };

            if end >= last_paid {
                let delta = end - std::cmp::max(prices.utime_since, last_paid);
                fee += if is_masterchain {
                    (cells * prices.mc_cell_price_ps as u128
                        + bits * prices.mc_bit_price_ps as u128)
                        * delta as u128
                } else {
                    (cells * prices.cell_price_ps as u128 + bits * prices.bit_price_ps as u128)
                        * delta as u128
                };
                last_paid = end;
            }
        }

        // stirage fee is calculated in pseudo values (like forward fee and gas fee) -
        // multiplied to 0xffff, so divide by this value with ceil rounding
        (fee + 0xffff) >> 16
    }

    fn with_config(config: &ConfigParam18) -> Result<Self> {
        let mut prices = vec![];
        for i in 0..config.len()? {
            prices.push(config.get(i as u32)?);
        }

        Ok(AccStoragePrices { prices })
    }
}

impl TONDefaultConfig for GasLimitsPrices {
    fn default_mc() -> Self {
        GasLimitsPrices {
            gas_price: 655360000,
            flat_gas_limit: 100,
            flat_gas_price: 1000000,
            gas_limit: 1000000,
            special_gas_limit: 10000000,
            gas_credit: 10000,
            block_gas_limit: 10000000,
            freeze_due_limit: 100000000,
            delete_due_limit: 1000000000,
            max_gas_threshold: 10000000000,
        }
    }

    fn default_wc() -> Self {
        GasLimitsPrices {
            gas_price: 65536000,
            flat_gas_limit: 100,
            flat_gas_price: 100000,
            gas_limit: 1000000,
            special_gas_limit: 1000000,
            gas_credit: 10000,
            block_gas_limit: 10000000,
            freeze_due_limit: 100000000,
            delete_due_limit: 1000000000,
            max_gas_threshold: 1000000000,
        }
    }
}

/// Blockchain configuration parameters
#[derive(Clone)]
pub struct BlockchainConfig {
    gas_prices_mc: GasLimitsPrices,
    gas_prices_wc: GasLimitsPrices,
    fwd_prices_mc: MsgForwardPrices,
    fwd_prices_wc: MsgForwardPrices,
    storage_prices: AccStoragePrices,
    special_contracts: FundamentalSmcAddresses,
    capabilities: u64,
    global_version: u32,
    raw_config: ConfigParams,
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        BlockchainConfig {
            gas_prices_mc: GasLimitsPrices::default_mc(),
            gas_prices_wc: GasLimitsPrices::default_wc(),
            fwd_prices_mc: MsgForwardPrices::default_mc(),
            fwd_prices_wc: MsgForwardPrices::default_wc(),
            storage_prices: AccStoragePrices::default(),
            special_contracts: Self::get_default_special_contracts(),
            raw_config: Self::get_defult_raw_config(),
            global_version: 0,
            capabilities: 0x2e,
        }
    }
}

impl BlockchainConfig {
    fn get_default_special_contracts() -> FundamentalSmcAddresses {
        let mut map = FundamentalSmcAddresses::default();
        map.add_key(&UInt256::with_array([0x33u8; 32])).unwrap();
        map.add_key(&UInt256::with_array([0x66u8; 32])).unwrap();
        map.add_key(
            &"34517C7BDF5187C55AF4F8B61FDC321588C7AB768DEE24B006DF29106458D7CF"
                .parse::<UInt256>()
                .unwrap(),
        )
        .unwrap();
        map
    }

    fn get_defult_raw_config() -> ConfigParams {
        ConfigParams { config_addr: [0x55; 32].into(), ..ConfigParams::default() }
    }

    /// Create `BlockchainConfig` struct with `ConfigParams` taken from
    /// blockchain
    pub fn with_config(config: ConfigParams) -> Result<Self> {
        Ok(BlockchainConfig {
            gas_prices_mc: config.gas_prices(true)?,
            gas_prices_wc: config.gas_prices(false)?,
            fwd_prices_mc: config.fwd_prices(true)?,
            fwd_prices_wc: config.fwd_prices(false)?,
            storage_prices: AccStoragePrices::with_config(&config.storage_prices()?)?,
            special_contracts: config.fundamental_smc_addr()?,
            capabilities: config.capabilities(),
            global_version: config.global_version(),
            raw_config: config,
        })
    }

    /// Get `MsgForwardPrices` for message forward fee calculation
    pub fn get_fwd_prices(&self, is_masterchain: bool) -> &MsgForwardPrices {
        if is_masterchain { &self.fwd_prices_mc } else { &self.fwd_prices_wc }
    }

    /// Calculate gas fee for account
    pub fn calc_gas_fee(&self, gas_used: u64, address: &MsgAddressInt) -> u128 {
        self.get_gas_config(address.is_masterchain()).calc_gas_fee(gas_used)
    }

    /// Get `GasLimitsPrices` for account gas fee calculation
    pub fn get_gas_config(&self, is_masterchain: bool) -> &GasLimitsPrices {
        if is_masterchain { &self.gas_prices_mc } else { &self.gas_prices_wc }
    }

    /// Calculate forward fee
    pub fn calc_fwd_fee(&self, is_masterchain: bool, msg_cell: &Cell) -> Result<Grams> {
        let mut in_fwd_fee = self.get_fwd_prices(is_masterchain).fwd_fee(msg_cell);
        if self.raw_config.has_capability(GlobalCapabilities::CapFeeInGasUnits) {
            in_fwd_fee = self.get_gas_config(is_masterchain).calc_gas_fee(in_fwd_fee.try_into()?)
        }
        Grams::new(in_fwd_fee)
    }

    /// Calculate account storage fee
    pub fn calc_storage_fee(
        &self,
        storage: &StorageInfo,
        is_masterchain: bool,
        now: u32,
    ) -> Result<Grams> {
        let mut storage_fee = self.storage_prices.calc_storage_fee(
            storage.used().cells().into(),
            storage.used().bits().into(),
            storage.last_paid(),
            now,
            is_masterchain,
        );
        if self.raw_config.has_capability(GlobalCapabilities::CapFeeInGasUnits) {
            storage_fee = self.get_gas_config(is_masterchain).calc_gas_fee(storage_fee.try_into()?)
        }
        Grams::new(storage_fee)
    }

    /// Check if account is special TON account
    pub fn is_special_account(&self, address: &MsgAddressInt) -> Result<bool> {
        if address.is_masterchain() {
            let account_id = address.get_address();
            // special account adresses are stored in hashmap
            // config account is special too
            Ok(self.raw_config.config_addr == account_id
                || self.special_contracts.get_raw(account_id)?.is_some())
        } else {
            Ok(false)
        }
    }

    pub fn global_version(&self) -> u32 {
        self.global_version
    }

    pub fn raw_config(&self) -> &ConfigParams {
        &self.raw_config
    }

    pub fn has_capability(&self, capability: GlobalCapabilities) -> bool {
        (self.capabilities & (capability as u64)) != 0
    }

    pub fn capabilites(&self) -> u64 {
        self.capabilities
    }
}
