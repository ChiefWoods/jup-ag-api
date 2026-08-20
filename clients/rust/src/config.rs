use jup_ag_api_generated::apis::configuration::{ApiKey, Configuration};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum JupiterClientService {
    Lend,
    LendBorrow,
    Portfolio,
    Prediction,
    PriceV2,
    PriceV3,
    Recurring,
    Send,
    Studio,
    SwapV1,
    SwapV2,
    TokensV1,
    TokensV2,
    TokensV2Verification,
    Transaction,
    TriggerV1,
    TriggerV2,
    Ultra,
}

#[derive(Clone, Debug, Default)]
pub struct JupiterClientBaseUrls {
    pub lend: Option<String>,
    pub lend_borrow: Option<String>,
    pub portfolio: Option<String>,
    pub prediction: Option<String>,
    pub price_v2: Option<String>,
    pub price_v3: Option<String>,
    pub recurring: Option<String>,
    pub send: Option<String>,
    pub studio: Option<String>,
    pub swap_v1: Option<String>,
    pub swap_v2: Option<String>,
    pub tokens_v1: Option<String>,
    pub tokens_v2: Option<String>,
    pub tokens_v2_verification: Option<String>,
    pub transaction: Option<String>,
    pub trigger_v1: Option<String>,
    pub trigger_v2: Option<String>,
    pub ultra: Option<String>,
}

impl JupiterClientBaseUrls {
    fn for_service(&self, service: JupiterClientService) -> Option<&str> {
        match service {
            JupiterClientService::Lend => self.lend.as_deref(),
            JupiterClientService::LendBorrow => self.lend_borrow.as_deref(),
            JupiterClientService::Portfolio => self.portfolio.as_deref(),
            JupiterClientService::Prediction => self.prediction.as_deref(),
            JupiterClientService::PriceV2 => self.price_v2.as_deref(),
            JupiterClientService::PriceV3 => self.price_v3.as_deref(),
            JupiterClientService::Recurring => self.recurring.as_deref(),
            JupiterClientService::Send => self.send.as_deref(),
            JupiterClientService::Studio => self.studio.as_deref(),
            JupiterClientService::SwapV1 => self.swap_v1.as_deref(),
            JupiterClientService::SwapV2 => self.swap_v2.as_deref(),
            JupiterClientService::TokensV1 => self.tokens_v1.as_deref(),
            JupiterClientService::TokensV2 => self.tokens_v2.as_deref(),
            JupiterClientService::TokensV2Verification => self.tokens_v2_verification.as_deref(),
            JupiterClientService::Transaction => self.transaction.as_deref(),
            JupiterClientService::TriggerV1 => self.trigger_v1.as_deref(),
            JupiterClientService::TriggerV2 => self.trigger_v2.as_deref(),
            JupiterClientService::Ultra => self.ultra.as_deref(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct JupiterClientOptions {
    /// API key sent as the `x-api-key` header.
    pub api_key: Option<String>,
    /// reqwest client used by every generated API call.
    pub client: Option<reqwest::Client>,
    /// Base URL overrides for individual source specifications.
    pub base_urls: JupiterClientBaseUrls,
}

const DEFAULT_BASE_URL: &str = "https://api.jup.ag";
const DEFAULT_TRANSACTION_BASE_URL: &str = "https://tx.jup.ag";

fn default_base_url(service: JupiterClientService) -> &'static str {
    match service {
        JupiterClientService::Transaction => DEFAULT_TRANSACTION_BASE_URL,
        _ => DEFAULT_BASE_URL,
    }
}

fn service_key(service: JupiterClientService) -> &'static str {
    match service {
        JupiterClientService::Lend => "lend",
        JupiterClientService::LendBorrow => "lend_borrow",
        JupiterClientService::Portfolio => "portfolio",
        JupiterClientService::Prediction => "prediction",
        JupiterClientService::PriceV2 => "price_v2",
        JupiterClientService::PriceV3 => "price_v3",
        JupiterClientService::Recurring => "recurring",
        JupiterClientService::Send => "send",
        JupiterClientService::Studio => "studio",
        JupiterClientService::SwapV1 => "swap_v1",
        JupiterClientService::SwapV2 => "swap_v2",
        JupiterClientService::TokensV1 => "tokens_v1",
        JupiterClientService::TokensV2 => "tokens_v2",
        JupiterClientService::TokensV2Verification => "tokens_v2_verification",
        JupiterClientService::Transaction => "transaction",
        JupiterClientService::TriggerV1 => "trigger_v1",
        JupiterClientService::TriggerV2 => "trigger_v2",
        JupiterClientService::Ultra => "ultra",
    }
}

fn reject_blank(value: &str, name: &str) {
    if value.trim().is_empty() {
        panic!("{name} must not be blank");
    }
}

pub(crate) fn create_configuration(
    options: &JupiterClientOptions,
    service: JupiterClientService,
) -> Configuration {
    if let Some(api_key) = &options.api_key {
        reject_blank(api_key, "api_key");
    }

    let base_path = options
        .base_urls
        .for_service(service)
        .unwrap_or_else(|| default_base_url(service))
        .to_owned();
    reject_blank(&base_path, &format!("base_urls.{}", service_key(service)));

    let mut configuration = Configuration::new();
    configuration.base_path = base_path;

    if let Some(api_key) = &options.api_key {
        configuration.api_key = Some(ApiKey {
            prefix: None,
            key: api_key.clone(),
        });
    }

    if let Some(client) = &options.client {
        configuration.client = client.clone();
    }

    configuration
}
