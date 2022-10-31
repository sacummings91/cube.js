use crate::metastore::QueueItem;
use crate::metastore::{IdRow, MetaStore};
use crate::queryplanner::InfoSchemaTableDef;
use crate::CubeError;
use arrow::array::{ArrayRef, StringArray, TimestampNanosecondArray};
use arrow::datatypes::{DataType, Field, TimeUnit};
use async_trait::async_trait;
use std::sync::Arc;

pub struct InfoSchemaQueueDef;

#[async_trait]
impl InfoSchemaTableDef for InfoSchemaQueueDef {
    type T = IdRow<QueueItem>;

    async fn rows(&self, meta_store: Arc<dyn MetaStore>) -> Result<Arc<Vec<Self::T>>, CubeError> {
        Ok(Arc::new(meta_store.all_queue().await?))
    }

    fn columns(&self) -> Vec<(Field, Box<dyn Fn(Arc<Vec<Self::T>>) -> ArrayRef>)> {
        vec![
            (
                Field::new("id", DataType::Utf8, false),
                Box::new(|items| {
                    Arc::new(StringArray::from_iter(
                        items.iter().map(|row| Some(row.get_row().get_key())),
                    ))
                }),
            ),
            (
                Field::new(
                    "created",
                    DataType::Timestamp(TimeUnit::Nanosecond, None),
                    false,
                ),
                Box::new(|items| {
                    Arc::new(TimestampNanosecondArray::from(
                        items
                            .iter()
                            .map(|row| row.get_row().get_created().timestamp_nanos())
                            .collect::<Vec<_>>(),
                    ))
                }),
            ),
            // (
            //     Field::new("value", DataType::Utf8, false),
            //     Box::new(|items| {
            //         Arc::new(StringArray::from_iter(
            //             items.iter().map(|row| Some(row.get_row().get_value())),
            //         ))
            //     }),
            // ),
        ]
    }
}

crate::base_info_schema_table_def!(InfoSchemaQueueDef);
