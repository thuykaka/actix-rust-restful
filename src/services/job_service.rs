use std::sync::Arc;

use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub struct JobService {
    scheduler: Arc<Mutex<JobScheduler>>,
}

impl JobService {
    pub async fn new() -> Result<Self, JobSchedulerError> {
        let scheduler = JobScheduler::new().await?;
        let scheduler = Arc::new(Mutex::new(scheduler));
        Ok(Self { scheduler })
    }

    pub async fn start(&self) -> Result<(), JobSchedulerError> {
        log::info!("starting job scheduler...");

        self.setup_jobs().await?;

        {
            let scheduler = self.scheduler.lock().await;
            scheduler.start().await?;
        }

        log::info!("job scheduler started successfully");
        Ok(())
    }

    async fn setup_jobs(&self) -> Result<(), JobSchedulerError> {
        let scheduler = self.scheduler.lock().await;

        self.add_test_job_1(&scheduler).await?;
        self.add_test_job_2(&scheduler).await?;

        log::info!("jobs setup completed");
        Ok(())
    }

    async fn add_test_job_1(&self, scheduler: &JobScheduler) -> Result<(), JobSchedulerError> {
        scheduler
            .add(Job::new_async("1/10 * * * * *", |_, _| {
                Box::pin(async move {
                    log::info!("test job 1 executed at {}", chrono::Utc::now());
                })
            })?)
            .await?;
        Ok(())
    }

    async fn add_test_job_2(&self, scheduler: &JobScheduler) -> Result<(), JobSchedulerError> {
        scheduler
            .add(Job::new_async("0/30 * * * * *", |_, _| {
                Box::pin(async move {
                    log::info!("test job 2 executed at {}", chrono::Utc::now());
                })
            })?)
            .await?;
        Ok(())
    }
}
