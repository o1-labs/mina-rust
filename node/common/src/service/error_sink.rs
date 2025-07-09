use node::core::{channels::mpsc, thread};

pub struct ErrorSinkService {
    error_report_sender: mpsc::TrackedUnboundedSender<(String, Vec<u8>)>,
}

impl ErrorSinkService {
    pub fn new(error_report_sender: mpsc::TrackedUnboundedSender<(String, Vec<u8>)>) -> Self {
        Self {
            error_report_sender,
        }
    }

    pub fn start(_url: String) -> Self {
        let (error_report_sender, error_report_receiver) = mpsc::unbounded_channel();

        thread::Builder::new()
            .name("openmina_error_sink".to_owned())
            .spawn(move || {
                error_sink_loop(error_report_receiver);
            })
            .unwrap();

        ErrorSinkService::new(error_report_sender)
    }

    pub fn pending_reports(&self) -> usize {
        self.error_report_sender.len()
    }

    pub fn submit_error_report(
        &self,
        category: &str,
        payload: Vec<u8>,
    ) -> Result<(), mpsc::SendError<(String, Vec<u8>)>> {
        self.error_report_sender
            .tracked_send((category.to_string(), payload))
    }
}

fn error_sink_loop(mut rx: mpsc::TrackedUnboundedReceiver<(String, Vec<u8>)>) {
    while let Some(msg) = rx.blocking_recv() {
        let (category, payload) = msg.0;
        openmina_core::debug!(
            message = "Processing error report",
            category = category,
            data_size = payload.len()
        );

        let submission_url = std::env::var("OPENMINA_ERROR_SINK_SERVICE_URL").ok();

        if let Some(url) = submission_url {
            if let Err(err) = submit_error_report(&category, &payload, &url) {
                openmina_core::error!(
                    message = "Failed to submit error report",
                    category = category,
                    error = format!("{}", err)
                );
            } else {
                openmina_core::debug!(
                    message = "Successfully submitted error report",
                    category = category
                );
            }
        } else {
            openmina_core::warn!(
                message = "No error sink URL configured, skipping report submission",
                category = category
            );
        }
    }
}

fn submit_error_report(category: &str, payload: &[u8], url: &str) -> anyhow::Result<()> {
    // TODO: Implement the actual submission logic to the external service
    // This would likely use reqwest or a similar HTTP client to send the data

    todo!("Implement error report submission to external service");

    // Example implementation might look like:
    // let client = reqwest::blocking::Client::new();
    // let response = client
    //     .post(url)
    //     .header("Content-Type", "application/octet-stream")
    //     .header("X-Error-Category", category)
    //     .body(data.to_vec())
    //     .send()?;
    //
    // if response.status().is_success() {
    //     Ok(())
    // } else {
    //     Err(anyhow::anyhow!("Failed with status: {}", response.status()))
    // }
}
