/// A macro to call a function every...
#[macro_export]
macro_rules! spawn_cron {
    ($fn_to_exec:expr, $interval:expr) => {
        tokio::spawn(async move {
            loop {
                $fn_to_exec().await;
                tokio::time::sleep($interval).await;
            }
        })
    };
}

pub async fn launch_crons() {
    // Example usage:
    // spawn_cron!(example_cron, 5.minutes());
}
