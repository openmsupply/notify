use chrono::NaiveDateTime;
use repository::{
    EqualFilter, NotificationConfigFilter, NotificationConfigKind, NotificationConfigRepository,
    NotificationConfigRow, NotificationConfigRowRepository, NotificationConfigSort,
    NotificationConfigStatus, PaginationOption,
};
use util::i64_to_u32;

use crate::{
    get_default_pagination, service_provider::ServiceContext, ListError, ListResult,
    SingleRecordError,
};

pub const MAX_LIMIT: u32 = 1000;
pub const MIN_LIMIT: u32 = 1;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct NotificationConfig {
    pub id: String,
    pub title: String,
    pub kind: NotificationConfigKind,
    pub configuration_data: String,
    pub status: NotificationConfigStatus,
    pub parameters: String,
    pub parameter_query_id: Option<String>,
    pub recipient_ids: Vec<String>,
    pub recipient_list_ids: Vec<String>,
    pub sql_recipient_list_ids: Vec<String>,
    pub last_run_datetime: Option<NaiveDateTime>,
    pub next_due_datetime: Option<NaiveDateTime>,
}

impl From<NotificationConfigRow> for NotificationConfig {
    fn from(
        NotificationConfigRow {
            id,
            title,
            kind,
            configuration_data,
            status,
            parameters,
            parameter_query_id,
            recipient_ids,
            recipient_list_ids,
            sql_recipient_list_ids,
            last_run_datetime,
            next_due_datetime,
        }: NotificationConfigRow,
    ) -> Self {
        NotificationConfig {
            id,
            title,
            kind,
            configuration_data,
            status,
            parameters,
            parameter_query_id,
            recipient_ids: serde_json::from_str(&recipient_ids).unwrap_or_default(),
            recipient_list_ids: serde_json::from_str(&recipient_list_ids).unwrap_or_default(),
            sql_recipient_list_ids: serde_json::from_str(&sql_recipient_list_ids)
                .unwrap_or_default(),
            last_run_datetime,
            next_due_datetime,
        }
    }
}

pub fn get_notification_configs(
    ctx: &ServiceContext,
    pagination: Option<PaginationOption>,
    filter: Option<NotificationConfigFilter>,
    sort: Option<NotificationConfigSort>,
) -> Result<ListResult<NotificationConfig>, ListError> {
    let pagination = get_default_pagination(pagination, MAX_LIMIT, MIN_LIMIT)?;
    let repository = NotificationConfigRepository::new(&ctx.connection);

    let rows = repository.query(pagination, filter.clone(), sort)?;

    Ok(ListResult {
        rows: rows.into_iter().map(|row| row.into()).collect(),
        count: i64_to_u32(repository.count(filter)?),
    })
}

pub fn get_notification_config(
    ctx: &ServiceContext,
    id: String,
) -> Result<NotificationConfig, SingleRecordError> {
    let repository = NotificationConfigRepository::new(&ctx.connection);

    let mut result = repository
        .query_by_filter(NotificationConfigFilter::new().id(EqualFilter::equal_to(&id)))?;

    if let Some(record) = result.pop() {
        Ok(record.into())
    } else {
        Err(SingleRecordError::NotFound(id))
    }
}

pub fn find_all_due_by_kind(
    ctx: &ServiceContext,
    kind: NotificationConfigKind,
    datetime: NaiveDateTime,
) -> Result<Vec<NotificationConfig>, ListError> {
    let repository = NotificationConfigRowRepository::new(&ctx.connection);
    let result = repository.find_all_due_by_kind(kind, datetime)?;

    Ok(result.into_iter().map(|row| row.into()).collect())
}
