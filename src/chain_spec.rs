use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use runtime::{
    opaque::SessionKeys, AccountId, BabeConfig, BalancesConfig, FinancialCouncilMembershipConfig,
    GeneralCouncilMembershipConfig, GenesisConfig, GrandpaConfig, IndicesConfig,
    OperatorMembershipConfig, SessionConfig, Signature, StakerStatus, StakingConfig, SudoConfig,
    SystemConfig, WASM_BINARY,
};
// use sc_service;
use sc_telemetry::TelemetryEndpoints;
use serde_json::map::Map;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::Perbill;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
    /// Whatever the current runtime is, with just Alice as an auth.
    Development,
    /// Whatever the current runtime is, with simple Alice/Bob auths.
    LocalTestnet,
    SunshineTestnet,
    SunshineTestnetLatest,
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, GrandpaId, BabeId) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
    )
}

impl Alternative {
    /// Get an actual chain config from one of the alternatives.
    pub(crate) fn load(self) -> Result<ChainSpec, String> {
        let mut properties = Map::new();
        properties.insert("tokenSymbol".into(), "SUNI".into());
        properties.insert("tokenDecimals".into(), 18.into());

        Ok(match self {
            Alternative::Development => ChainSpec::from_genesis(
                "Development",
                "dev",
                || {
                    dev_genesis(
                        vec![get_authority_keys_from_seed("Alice")],
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        vec![
                            get_account_id_from_seed::<sr25519::Public>("Alice"),
                            get_account_id_from_seed::<sr25519::Public>("Bob"),
                            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                        ],
                        true,
                    )
                },
                vec![],
                None,
                None,
                Some(properties),
                Default::default(),
            ),
            Alternative::LocalTestnet => ChainSpec::from_genesis(
                "Local Testnet",
                "local_testnet",
                || {
                    dev_genesis(
                        vec![
                            get_authority_keys_from_seed("Alice"),
                            get_authority_keys_from_seed("Bob"),
                        ],
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        vec![
                            get_account_id_from_seed::<sr25519::Public>("Alice"),
                            get_account_id_from_seed::<sr25519::Public>("Bob"),
                            get_account_id_from_seed::<sr25519::Public>("Charlie"),
                            get_account_id_from_seed::<sr25519::Public>("Dave"),
                            get_account_id_from_seed::<sr25519::Public>("Eve"),
                            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                        ],
                        true,
                    )
                },
                vec![],
                None,
                None,
                Some(properties),
                Default::default(),
            ),
            Alternative::SunshineTestnet => {
                ChainSpec::from_json_bytes(&include_bytes!("../resources/testnet-dist.json")[..])?
            }
            Alternative::SunshineTestnetLatest => {
                ChainSpec::from_genesis(
					"Sunshine Testnet",
					"sunshine-testnet",
					|| {
						// TODO: regenerate alphabet according to babe-grandpa consensus
						// SECRET="..."
						// ./target/debug/subkey --sr25519 inspect "$SECRET//sunshine//aura"
						// ./target/debug/subkey --ed25519 inspect "$SECRET//sunshine//grandpa"
						// ./target/debug/subkey inspect "$SECRET//sunshine//root"
						// ./target/debug/subkey inspect "$SECRET//sunshine//oracle"
						testnet_genesis(
							vec![(
								// TODO: regenerate alphanet according to babe-grandpa consensus
								// 5HGU1TsEkXDgpGdhwpYdzdgxfMAyRUYK3FuiaE5CYR9s78y5
								hex!["e6257e9066e63b860259ee5c7cb752ac37a9ddf9f8bf889d6a3b95cf89ccab5a"]
									.into(),
								// 5HGU1TsEkXDgpGdhwpYdzdgxfMAyRUYK3FuiaE5CYR9s78y5
								hex!["e6257e9066e63b860259ee5c7cb752ac37a9ddf9f8bf889d6a3b95cf89ccab5a"]
									.into(),
								// 5HGU1TsEkXDgpGdhwpYdzdgxfMAyRUYK3FuiaE5CYR9s78y5
								hex!["e6257e9066e63b860259ee5c7cb752ac37a9ddf9f8bf889d6a3b95cf89ccab5a"]
									.unchecked_into(),
								// 5H5NcTUZRmV4nwZAjaJgiSyfYBafAcrkU2dBAJ9bSArqZi4E
								hex!["ddafa0cdbaab3c9662b535c544a01b0ba5d09e850dd15c61525e626821695926"]
									.unchecked_into(),
							)],
							// 5FeowPepSWZ1rP11pKRLmhBxtxLVnHvayxHxJBk6SD6THKZF
							hex!["9eb78419050eff5d5d95d889b125ca69af78f399bf4641aac2cb39d7c18edb79"].into(),
							vec![
								// 5FeowPepSWZ1rP11pKRLmhBxtxLVnHvayxHxJBk6SD6THKZF
								hex!["9eb78419050eff5d5d95d889b125ca69af78f399bf4641aac2cb39d7c18edb79"].into(),
								// 5EZC7fb3W1F5548fakGVb19tDaM1zKHxBpg7UvzpkpmuyYki
								hex!["6e32770eef925d3e31a575b1fdc1c67d387eaac589daecfc77a2661c97711036"].into(),
							],
						)
					},
					vec![
						"/dns4/testnet-bootnode-1.sunshine-chain.sunshine.one/tcp/30333/p2p/QmQUpeDzQk4jszwMsb9zUKMfGMZT4fkC1iTiPyCnGVGY8H".into(),
					],
					Some(TelemetryEndpoints::new(vec![(
						"wss://telemetry.polkadot.io/submit/".into(),
						0,
					)])),
					Some("suni-test"),
					Some(properties),
					None,
				)
            }
        })
    }

    pub(crate) fn from(s: &str) -> Option<Self> {
        match s {
            "dev" => Some(Alternative::Development),
            "local" => Some(Alternative::LocalTestnet),
            "" | "testnet" => Some(Alternative::SunshineTestnet),
            "testnet-latest" => Some(Alternative::SunshineTestnetLatest),
            _ => None,
        }
    }
}

fn session_keys(grandpa: GrandpaId, babe: BabeId) -> SessionKeys {
    SessionKeys { grandpa, babe }
}

const INITIAL_BALANCE: u128 = 1_000_000_000_000_000_000_000_u128; // $1M
const INITIAL_STAKING: u128 = 1_000_000_000_000_000_000_u128;

fn dev_genesis(
    initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        system: Some(SystemConfig {
            code: WASM_BINARY.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_indices: Some(IndicesConfig {
            ids: endowed_accounts.clone(),
        }),
        pallet_balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, INITIAL_BALANCE))
                .collect(),
        }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), session_keys(x.2.clone(), x.3.clone())))
                .collect::<Vec<_>>(),
        }),
        pallet_staking: Some(StakingConfig {
            current_era: 0,
            validator_count: initial_authorities.len() as u32 * 2,
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.1.clone(),
                        INITIAL_STAKING,
                        StakerStatus::Validator,
                    )
                })
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_sudo: Some(SudoConfig {
            key: root_key.clone(),
        }),
        pallet_babe: Some(BabeConfig {
            authorities: vec![],
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        pallet_collective_Instance1: Some(Default::default()),
        pallet_membership_Instance1: Some(GeneralCouncilMembershipConfig {
            members: vec![root_key.clone()],
            phantom: Default::default(),
        }),
        pallet_collective_Instance2: Some(Default::default()),
        pallet_membership_Instance2: Some(FinancialCouncilMembershipConfig {
            members: vec![root_key.clone()],
            phantom: Default::default(),
        }),
        pallet_collective_Instance3: Some(Default::default()),
        pallet_membership_Instance3: Some(OperatorMembershipConfig {
            members: vec![root_key],
            phantom: Default::default(),
        }),
        pallet_treasury: Some(Default::default()),
    }
}

fn testnet_genesis(
    initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
    GenesisConfig {
        system: Some(SystemConfig {
            code: WASM_BINARY.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_indices: Some(IndicesConfig {
            ids: endowed_accounts.clone(),
        }),
        pallet_balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, INITIAL_BALANCE))
                .collect(),
        }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), session_keys(x.2.clone(), x.3.clone())))
                .collect::<Vec<_>>(),
        }),
        pallet_staking: Some(StakingConfig {
            current_era: 0,
            validator_count: initial_authorities.len() as u32 * 2,
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.1.clone(),
                        INITIAL_STAKING,
                        StakerStatus::Validator,
                    )
                })
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_sudo: Some(SudoConfig {
            key: root_key.clone(),
        }),
        pallet_babe: Some(BabeConfig {
            authorities: vec![],
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        pallet_collective_Instance1: Some(Default::default()),
        pallet_membership_Instance1: Some(GeneralCouncilMembershipConfig {
            members: vec![root_key.clone()],
            phantom: Default::default(),
        }),
        pallet_collective_Instance2: Some(Default::default()),
        pallet_membership_Instance2: Some(FinancialCouncilMembershipConfig {
            members: vec![root_key.clone()],
            phantom: Default::default(),
        }),
        pallet_collective_Instance3: Some(Default::default()),
        pallet_membership_Instance3: Some(OperatorMembershipConfig {
            members: vec![root_key],
            phantom: Default::default(),
        }),
        pallet_treasury: Some(Default::default()),
    }
}

pub fn load_spec(id: &str) -> Result<Option<ChainSpec>, String> {
    Ok(match Alternative::from(id) {
        Some(spec) => Some(spec.load()?),
        None => None,
    })
}