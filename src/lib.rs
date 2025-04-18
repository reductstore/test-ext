use async_trait::async_trait;
use log::info;
use reduct_base::error::ReductError;
use reduct_base::ext::{BoxedReadRecord, ExtSettings, IoExtension, IoExtensionInfo, ProcessStatus};
use reduct_base::io::{ReadChunk, ReadRecord, RecordMeta};
use reduct_base::logger::Logger;
use reduct_base::msg::entry_api::QueryEntry;
use reduct_base::Labels;
use std::time::Duration;

#[no_mangle]
pub fn get_ext(settings: ExtSettings) -> *mut (dyn IoExtension + Send + Sync) {
    // Return a raw pointer to an instance of our plugin
    Logger::init(settings.log_level());
    info!("Init");
    Box::into_raw(Box::new(TestExtension::new()))
}

struct TestExtension {
    info: IoExtensionInfo,
}

impl TestExtension {
    fn new() -> Self {
        Self {
            info: IoExtensionInfo::builder()
                .name("test-ext")
                .version(env!("CARGO_PKG_VERSION"))
                .build(),
        }
    }
}

#[async_trait]
impl IoExtension for TestExtension {
    fn info(&self) -> &IoExtensionInfo {
        &self.info
    }

    fn register_query(
        &mut self,
        query_id: u64,
        bucket_name: &str,
        entry_name: &str,
        query: &QueryEntry,
    ) -> Result<(), ReductError> {
        Ok(())
    }

    fn unregister_query(&mut self, query_id: u64) -> Result<(), ReductError> {
        Ok(())
    }

    async fn next_processed_record(
        &mut self,
        query_id: u64,
        record: BoxedReadRecord,
    ) -> ProcessStatus {
        ProcessStatus::Stop
    }
}

struct Wrapper {
    reader: BoxedReadRecord,
    labels: Labels,
    computed_labels: Labels,
}

impl RecordMeta for Wrapper {
    fn timestamp(&self) -> u64 {
        self.reader.timestamp()
    }

    fn labels(&self) -> &Labels {
        &self.labels
    }
}

#[async_trait]
impl ReadRecord for Wrapper {
    async fn read(&mut self) -> ReadChunk {
        self.reader.read().await
    }

    async fn read_timeout(&mut self, timeout: Duration) -> ReadChunk {
        self.reader.read_timeout(timeout).await
    }

    fn blocking_read(&mut self) -> ReadChunk {
        self.reader.blocking_read()
    }

    fn last(&self) -> bool {
        self.reader.last()
    }
    fn computed_labels(&self) -> &Labels {
        &self.computed_labels
    }

    fn computed_labels_mut(&mut self) -> &mut Labels {
        &mut self.computed_labels
    }

    fn content_length(&self) -> u64 {
        self.reader.content_length()
    }

    fn content_type(&self) -> &str {
        self.reader.content_type()
    }
}
