use axum::middleware::Next;
use axum::http::{Request,Response};
use axum::body::Body;
use axum::Extension;
use tracing::info;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub async fn request_counter_middleware(
    Extension(counter): Extension<Arc<AtomicUsize>>,
    Extension(is_shutting_down): Extension<Arc<AtomicBool>>,
    req: Request<Body>,
    next: Next<>,
) -> Response<Body> {
    // 如果服务器正在关闭，则返回 503
    if is_shutting_down.load(Ordering::SeqCst) {
        info!("Server is shutting down, returning 503");
        return Response::builder()
            .status(503)
            .body(Body::empty())
            .unwrap();
    }
    // 这里假设计数器已经通过 Extension 传递进来
    // let Extension(counter) = req.extensions().get::<Extension<Arc<AtomicUsize>>>().cloned().unwrap();
    counter.fetch_add(1, Ordering::SeqCst);
    info!("Request count: {}", counter.load(Ordering::SeqCst));
    let response = next.run(req).await;
    counter.fetch_sub(1, Ordering::SeqCst);
    response
}

