pub mod dispatcher;
pub mod inbound_server;
pub mod scheduler;

pub use dispatcher::{dispatch_to_platform, N8nOutboundPayload, PublishResult};
pub use inbound_server::{start_inbound_listener_if_needed, N8nCallbackPayload};
pub use scheduler::{
    calculate_drip_offset_seconds, cancel_schedule, create_drip_batch, create_schedule,
    get_schedule, list_publishing_channels, list_schedules, poll_and_dispatch_due_jobs,
    save_publishing_channel, start_cron_worker_if_needed, trigger_instant_dispatch,
    ContentSchedule, CreateDripBatchInput, CreateScheduleInput, PublishingChannel,
};

/// Initializes both the background Cron worker and the Axum inbound HTTP server
pub fn init_automation_subsystem() {
    inbound_server::start_inbound_listener_if_needed();
    scheduler::start_cron_worker_if_needed();
}
