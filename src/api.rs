use bytecheck::CheckBytes;
#[cfg(not(feature = "ssr"))]
use js_sys::Uint8Array;
use leptos::{Scope, Serializable};
use rkyv::{
    de::deserializers::SharedDeserializeMap, from_bytes, ser::serializers::AllocSerializer,
    to_bytes, validation::validators::DefaultValidator, Archive, Deserialize, Serialize,
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
    use reqwest::{Certificate, Client};

    let start = std::time::Instant::now();
    let client = CLIENT.get_or_init(|| {
        let dev =
            DEV_MODE.get_or_init(|| std::env::var("DEV").unwrap_or("false".to_string()) == "true");
        let mut client = Client::builder()
            .pool_max_idle_per_host(2)
            .http2_prior_knowledge();
        if dev.clone() {
            let cert = std::fs::read("../platform/cert.pem").expect("failed to read cert");
            client = client
                .danger_accept_invalid_certs(dev.clone())
                .add_root_certificate(Certificate::from_pem(&cert).expect("failed to parse cert"));
        }
        client.build().expect("failed to build client")
    });
    let bytes = client
        .post(path)
        .body(to_bytes(&body).ok()?.to_vec())
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .bytes()
        .await
        .ok()?;
    log::info!("fetch took {:?}", start.elapsed());
    from_bytes::<T>(&bytes).ok()
}
