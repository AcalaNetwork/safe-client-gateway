use crate::cache::cache_operations::RequestCached;
use crate::cache::manager::ChainCache;
use crate::config::collectibles_request_timeout;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::response::content::RawJson;

pub async fn collectibles(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    trusted: Option<bool>,
    exclude_spam: Option<bool>,
) -> ApiResult<RawJson<String>> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);

    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/collectibles/?trusted={}&exclude_spam={}",
        safe_address,
        trusted.unwrap_or(false),
        exclude_spam.unwrap_or(true)
    )?;

    Ok(RawJson(
        RequestCached::new_from_context(url, &context, ChainCache::from(chain_id))
            .request_timeout(collectibles_request_timeout())
            .execute()
            .await?,
    ))
}
