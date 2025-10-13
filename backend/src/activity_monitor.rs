use std::{sync::Arc, time::Duration};

use tokio::{sync::Mutex, time::{sleep_until, Instant}};


// Simple watchdog implementation
pub struct ActivityMonitor {
    last_activity: Arc<Mutex<Instant>>,
    pub timeout_after: Duration,
}

impl ActivityMonitor {
    pub fn new(timeout_after: Duration) -> Self {
        ActivityMonitor {
            last_activity: Arc::new(Mutex::new(Instant::now())),
            timeout_after,
        }
    }

    pub fn get_last_activity(&self) -> Arc<Mutex<Instant>> {
        Arc::clone(&self.last_activity)
    }

    pub async fn feed(&self) {
        *self.last_activity.lock().await = Instant::now();
        println!("Watchdog fed");
    }

    pub async fn watch_inactivity(&self) {
        loop {
            let deadline = Instant::now() + self.timeout_after;

            sleep_until(deadline).await;

            if self.last_activity.lock().await.elapsed() >= self.timeout_after {
                // No activity, shutdown.
                println!("Shutdown signal should be sent");
                break;
            }
        }
    }
}

