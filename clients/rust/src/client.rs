use jup_ag_api_generated::apis::configuration::Configuration;

use crate::config::{create_configuration, JupiterClientOptions, JupiterClientService};

/// Typed access to Jupiter APIs, organized after the local OpenAPI source tree.
#[derive(Clone, Debug)]
pub struct JupiterClient {
    pub lend: LendApis,
    pub portfolio: Portfolio,
    pub prediction: Prediction,
    pub price: PriceApis,
    pub recurring: Recurring,
    pub send: SendApi,
    pub studio: Studio,
    pub swap: SwapApis,
    pub tokens: TokensApis,
    pub transaction: Transaction,
    pub trigger: TriggerApis,
    pub ultra: Ultra,
}

#[derive(Clone, Debug)]
pub struct LendApis {
    pub borrow: LendBorrow,
    pub lend: Lend,
}

#[derive(Clone, Debug)]
pub struct PriceApis {
    pub v2: PriceV2,
    pub v3: PriceV3,
}

#[derive(Clone, Debug)]
pub struct SwapApis {
    pub v1: SwapV1,
    pub v2: SwapV2,
}

#[derive(Clone, Debug)]
pub struct TokensApis {
    pub v1: TokensV1,
    pub v2: TokensV2,
}

#[derive(Clone, Debug)]
pub struct TriggerApis {
    pub v1: TriggerV1,
    pub v2: TriggerV2,
}

macro_rules! api_client {
    ($name:ident) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            pub(crate) config: Configuration,
        }
    };
}

api_client!(Lend);
api_client!(LendBorrow);
api_client!(Portfolio);
api_client!(Prediction);
api_client!(PriceV2);
api_client!(PriceV3);
api_client!(Recurring);
api_client!(SendApi);
api_client!(Studio);
api_client!(SwapV1);
api_client!(SwapV2);
api_client!(TokensV1);
api_client!(TokensV2Verification);
api_client!(Transaction);
api_client!(TriggerV1);
api_client!(TriggerV2);
api_client!(Ultra);

#[derive(Clone, Debug)]
pub struct TokensV2 {
    pub(crate) config: Configuration,
    pub verification: TokensV2Verification,
}

impl JupiterClient {
    pub fn new(options: JupiterClientOptions) -> Self {
        Self {
            lend: LendApis {
                borrow: LendBorrow {
                    config: create_configuration(&options, JupiterClientService::LendBorrow),
                },
                lend: Lend {
                    config: create_configuration(&options, JupiterClientService::Lend),
                },
            },
            portfolio: Portfolio {
                config: create_configuration(&options, JupiterClientService::Portfolio),
            },
            prediction: Prediction {
                config: create_configuration(&options, JupiterClientService::Prediction),
            },
            price: PriceApis {
                v2: PriceV2 {
                    config: create_configuration(&options, JupiterClientService::PriceV2),
                },
                v3: PriceV3 {
                    config: create_configuration(&options, JupiterClientService::PriceV3),
                },
            },
            recurring: Recurring {
                config: create_configuration(&options, JupiterClientService::Recurring),
            },
            send: SendApi {
                config: create_configuration(&options, JupiterClientService::Send),
            },
            studio: Studio {
                config: create_configuration(&options, JupiterClientService::Studio),
            },
            swap: SwapApis {
                v1: SwapV1 {
                    config: create_configuration(&options, JupiterClientService::SwapV1),
                },
                v2: SwapV2 {
                    config: create_configuration(&options, JupiterClientService::SwapV2),
                },
            },
            tokens: TokensApis {
                v1: TokensV1 {
                    config: create_configuration(&options, JupiterClientService::TokensV1),
                },
                v2: TokensV2 {
                    config: create_configuration(&options, JupiterClientService::TokensV2),
                    verification: TokensV2Verification {
                        config: create_configuration(
                            &options,
                            JupiterClientService::TokensV2Verification,
                        ),
                    },
                },
            },
            transaction: Transaction {
                config: create_configuration(&options, JupiterClientService::Transaction),
            },
            trigger: TriggerApis {
                v1: TriggerV1 {
                    config: create_configuration(&options, JupiterClientService::TriggerV1),
                },
                v2: TriggerV2 {
                    config: create_configuration(&options, JupiterClientService::TriggerV2),
                },
            },
            ultra: Ultra {
                config: create_configuration(&options, JupiterClientService::Ultra),
            },
        }
    }
}

impl Default for JupiterClient {
    fn default() -> Self {
        Self::new(JupiterClientOptions::default())
    }
}
