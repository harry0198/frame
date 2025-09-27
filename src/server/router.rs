use std::{sync::Arc, time::Duration};

use crate::server::{activity_monitor::ActivityMonitor, handlers::{get_images, upload_image}};
use axum::{middleware::{self, Next}, response::Response, extract::{Request, State}, routing::get, Router};

pub fn create_router() -> Router {
    let activity_monitor = Arc::new(ActivityMonitor::new(Duration::from_secs(30)));
    
    let router = Router::new()
        .route("/api/images",
            get(get_images)
            .post(upload_image))
        .layer(middleware::from_fn_with_state(
            activity_monitor.clone(),
            activity_monitor_middleware
        ));

    // // Start the watcher
    // tokio::spawn({
    //     let activity_monitor = activity_monitor.clone();
    //     async move { activity_monitor.watch_inactivity().await }
    // });

    router
}

async fn activity_monitor_middleware(
    State(activity_handle): State<Arc<ActivityMonitor>>,
    request: Request, 
    next: Next
) -> Response {
    // Update the activity timestamp
    activity_handle.feed().await;
    next.run(request).await
}
