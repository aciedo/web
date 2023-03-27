#[cfg(not(feature = "ssr"))]
use js_sys::Uint8Array;
use leptos::{Scope, Serializable};
use rkyv::{
    de::deserializers::SharedDeserializeMap, from_bytes, ser::serializers::AllocSerializer,
    to_bytes, validation::validators::DefaultValidator, Archive, CheckBytes, Deserialize,
    Serialize,
};

#[cfg(not(feature = "ssr"))]
pub async fn fetch<T, K, const N: usize>(cx: Scope, path: &str, body: K) -> Option<T>
where
    T: Serialize<AllocSerializer<1024>>,
    T: Serializable + Archive,
    T::Archived: for<'b> CheckBytes<DefaultValidator<'b>> + Deserialize<T, SharedDeserializeMap>,
    K: Serialize<AllocSerializer<N>>,
{
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    let bytes = gloo_net::http::Request::post(path)
        .abort_signal(abort_signal.as_ref())
        .body(Uint8Array::from(to_bytes(&body).ok()?.as_slice()))
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .binary()
        .await
        .ok()?;

    // abort in-flight requests if the Scope is disposed
    // i.e., if we've navigated away from this page
    leptos::on_cleanup(cx, move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });
    from_bytes::<T>(&bytes).ok()
}

#[cfg(feature = "ssr")]
pub async fn fetch<T, K, const N: usize>(_cx: Scope, path: &str, body: K) -> Option<T>
where
    T: Serialize<AllocSerializer<1024>>,
    T: Serializable + Archive,
    T::Archived: for<'b> CheckBytes<DefaultValidator<'b>> + Deserialize<T, SharedDeserializeMap>,
    K: Serialize<AllocSerializer<N>>,
{
    use crate::{CLIENT, DEV_MODE};

    let host = if DEV_MODE.clone() {
        std::env::var("API_HOST").unwrap_or("localhost:8080".to_string())
    } else {
        "api.valera.co".to_string()
    };

    let bytes = CLIENT
        .post(format!("https://{host}{path}"))
        .body(to_bytes(&body).ok()?.to_vec())
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .bytes()
        .await
        .ok()?;
    from_bytes::<T>(&bytes).ok()
}
