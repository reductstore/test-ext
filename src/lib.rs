use async_stream::stream;
use async_trait::async_trait;
use log::info;
use reduct_base::error::ReductError;
use reduct_base::ext::{BoxedCommiter, BoxedProcessor, BoxedReadRecord, BoxedRecordStream, Commiter, ExtSettings, IoExtension, IoExtensionInfo, Processor};
use reduct_base::logger::Logger;
use reduct_base::msg::entry_api::QueryEntry;

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

    fn query(
        &mut self,
        _bucket_name: &str,
        _entry_name: &str,
        _query: &QueryEntry,
    ) -> Result<(BoxedProcessor, BoxedCommiter), ReductError> {
        struct DummyProcessor;

        #[async_trait]
        impl Processor for DummyProcessor {
            async fn process_record(
                &mut self,
                record: BoxedReadRecord,
            ) -> Result<BoxedRecordStream, ReductError> {
                let stream = stream! {
                    yield Ok(record);
                };

                Ok(Box::new(stream))
            }
        }


        struct DummyCommiter;

        #[async_trait]
        impl Commiter for DummyCommiter {
            async fn commit_record(&mut self, record: BoxedReadRecord) -> Option<Result<BoxedReadRecord, ReductError>> {
                Some(Ok(record))
            }

            async fn flush(&mut self) -> Option<Result<BoxedReadRecord, ReductError>> {
                None
            }
        }
        
        
        Ok((
            Box::new(DummyProcessor) as BoxedProcessor,
            Box::new(DummyCommiter) as BoxedCommiter,
        ))
    }
}
