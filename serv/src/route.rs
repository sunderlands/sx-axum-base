use std::{str::FromStr, time::Duration, vec};

use anyhow::Context;
use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{HeaderName, HeaderValue, Method},
};
use log::error_with_context;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer, trace::TraceLayer,
};

pub async fn route() -> anyhow::Result<Router> {
    let mut route = Router::new().nest("/api", Router::new());
    route = layers(route).with_context(error_with_context!("路由配置layer时发生异常"))?;
    tracing::info!("路由创建成功");

    Ok(route)
}

fn layers(router: Router) -> anyhow::Result<Router> {
    let router = cors(router).with_context(error_with_context!("路由配置CORS时发生异常"))?;

    Ok(router
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(32 * 1024 * 1024))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(CompressionLayer::new()))
}

fn cors(mut router: Router) -> anyhow::Result<Router> {
    if let Some(cors) = &config::instance().server.cors {
        fn read_cors_config<T>(strings: &Vec<String>, error_info: &str) -> anyhow::Result<Vec<T>>
        where
            T: FromStr,
            T::Err: Into<anyhow::Error>,
        {
            let mut vec: Vec<T> = vec![];
            for str in strings {
                let v = str.parse::<T>();
                match v {
                    Ok(a) => vec.push(a),
                    Err(e) => {
                        return Err(e.into()).with_context(error_with_context!(
                            "配置{error_info}时发生异常,错误输入为:[{}]",
                            str
                        ));
                    }
                }
            }
            Ok(vec)
        }

        let mut cors_layer = CorsLayer::new();

        if let Some(origins) = &cors.origin {
            let vec: Vec<HeaderValue> = read_cors_config(origins, "CORS-Origin")?;
            cors_layer = cors_layer.allow_origin(vec);
        }

        if let Some(methods) = &cors.methods {
            let vec: Vec<Method> = read_cors_config(methods, "CORS-Method")?;
            cors_layer = cors_layer.allow_methods(vec);
        }

        if let Some(headers) = &cors.headers {
            let vec: Vec<HeaderName> = read_cors_config(headers, "CORS-Header")?;
            cors_layer = cors_layer.allow_headers(vec);
        }

        router = router.layer(cors_layer);
    }
    Ok(router)
}
