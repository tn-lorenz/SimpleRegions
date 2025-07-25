use crate::TOKIO_RUNTIME;
use std::sync::Arc;
use tokio::time::{Duration, sleep};

#[async_trait::async_trait]
pub trait TaskHandler: Send + Sync {
    async fn run(&self);
}

// TODO: As soon as the `on_player_death` event is available, kill this task and remove the player from `ACTIVE_UUIDS` (need to implement more concise thread handling for that)
pub fn start_loop<H>(delay: Duration, handler: Arc<H>)
where
    H: TaskHandler + 'static,
{
    TOKIO_RUNTIME.spawn(run_task_timer(delay, handler));
}

async fn run_task_timer<H>(delay: Duration, handler: Arc<H>)
where
    H: TaskHandler + 'static,
{
    loop {
        sleep(delay).await;
        handler.run().await;
    }
}

#[macro_export]
macro_rules! run_task_timer {
    ($delay:expr, $body:block) => {{
        use std::sync::Arc;
        use $crate::task::{start_loop, TaskHandler};

        struct InlineHandler;

        #[async_trait::async_trait]
        impl TaskHandler for InlineHandler {
            async fn run(&self) $body
        }

        let handler = Arc::new(InlineHandler);
        start_loop($delay, handler);
    }};
}
