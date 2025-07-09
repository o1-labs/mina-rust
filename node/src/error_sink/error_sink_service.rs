pub trait ErrorPayload {
    fn to_payload(&self) -> Vec<u8>;
}

impl ErrorPayload for String {
    fn to_payload(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

pub trait ErrorSinkService {
    fn submit_error_report_payload(&mut self, category: &str, payload: Vec<u8>);

    fn submit_error_report<E>(&mut self, category: &str, error: E)
    where
        E: ErrorPayload,
    {
        let payload = error.to_payload();
        self.submit_error_report_payload(category, payload);
    }
}
