use std::ops::Index;

mod user_account;
pub use user_account::*;
mod user_permissions;
pub use user_permissions::*;
mod notification_configs;
pub use notification_configs::*;
mod recipients;
pub use recipients::*;
mod recipient_lists;
pub use recipient_lists::*;
mod recipient_list_members;
pub use recipient_list_members::*;
mod sql_recipient_list;
pub use sql_recipient_list::*;

use crate::{
    NotificationConfigRow, NotificationConfigRowRepository, RecipientListMemberRow,
    RecipientListMemberRowRepository, RecipientListRow, RecipientListRowRepository, RecipientRow,
    RecipientRowRepository, SqlRecipientListRow, SqlRecipientListRowRepository, UserAccountRow,
    UserAccountRowRepository, UserPermissionRow, UserPermissionRowRepository,
};

use super::StorageConnection;

#[derive(Default, Clone)]
pub struct MockData {
    pub user_accounts: Vec<UserAccountRow>,
    pub permissions: Vec<UserPermissionRow>,
    pub recipients: Vec<RecipientRow>,
    pub recipient_lists: Vec<RecipientListRow>,
    pub recipient_list_members: Vec<RecipientListMemberRow>,
    pub sql_recipient_lists: Vec<SqlRecipientListRow>,
    pub notification_configs: Vec<NotificationConfigRow>,
}

#[derive(Default)]
pub struct MockDataInserts {
    pub user_accounts: bool,
    pub permissions: bool,
    pub recipients: bool,
    pub recipient_lists: bool,
    pub recipient_list_members: bool,
    pub sql_recipient_lists: bool,
    pub notification_configs: bool,
}

impl MockDataInserts {
    pub fn all() -> Self {
        MockDataInserts {
            user_accounts: true,
            permissions: true,
            recipients: true,
            recipient_lists: true,
            recipient_list_members: true,
            sql_recipient_lists: true,
            notification_configs: true,
        }
    }

    pub fn none() -> Self {
        MockDataInserts::default()
    }

    pub fn user_accounts(mut self) -> Self {
        self.user_accounts = true;
        self
    }

    pub fn permissions(mut self) -> Self {
        self.user_accounts = true; // Permissions require user accounts
        self.permissions = true;
        self
    }

    pub fn recipients(mut self) -> Self {
        self.recipients = true;
        self
    }

    pub fn recipient_lists(mut self) -> Self {
        self.recipient_lists = true;
        self
    }

    pub fn recipient_list_members(mut self) -> Self {
        self.recipients = true;
        self.recipient_lists = true;
        self.recipient_list_members = true;
        self
    }

    pub fn sql_recipient_lists(mut self) -> Self {
        self.sql_recipient_lists = true;
        self
    }

    pub fn notification_configs(mut self) -> Self {
        self.notification_configs = true;
        self
    }
}

#[derive(Default)]
pub struct MockDataCollection {
    // Note: can't use a HashMap since mock data should be inserted in order
    pub data: Vec<(String, MockData)>,
}

impl MockDataCollection {
    pub fn insert(&mut self, name: &str, data: MockData) {
        self.data.push((name.to_string(), data));
    }

    pub fn get_mut(&mut self, name: &str) -> &mut MockData {
        for (n, data) in &mut self.data {
            if n != name {
                continue;
            }
            return data;
        }
        unreachable!("Missing mock data");
    }
}

impl Index<&str> for MockDataCollection {
    type Output = MockData;

    fn index(&self, name: &str) -> &Self::Output {
        &self.data.iter().find(|entry| entry.0 == name).unwrap().1
    }
}

fn all_mock_data() -> MockDataCollection {
    let mut data: MockDataCollection = Default::default();
    data.insert(
        "base",
        MockData {
            user_accounts: mock_user_accounts(),
            permissions: mock_permissions(),
            recipients: mock_recipients(),
            recipient_lists: mock_recipient_lists(),
            recipient_list_members: mock_recipient_list_members(),
            sql_recipient_lists: mock_sql_recipient_lists(),
            notification_configs: mock_notification_configs(),
        },
    );
    data
}

pub async fn insert_all_mock_data(
    connection: &StorageConnection,
    inserts: MockDataInserts,
) -> MockDataCollection {
    insert_mock_data(connection, inserts, all_mock_data()).await
}

pub async fn insert_mock_data(
    connection: &StorageConnection,
    inserts: MockDataInserts,
    mock_data: MockDataCollection,
) -> MockDataCollection {
    for (_, mock_data) in &mock_data.data {
        if inserts.user_accounts {
            let repo = UserAccountRowRepository::new(connection);
            for row in &mock_data.user_accounts {
                repo.insert_one(row).unwrap();
            }
        }
        if inserts.permissions {
            let repo = UserPermissionRowRepository::new(connection);
            for row in &mock_data.permissions {
                repo.insert_one(row).unwrap();
            }
        }
        if inserts.recipients {
            let repo = RecipientRowRepository::new(connection);
            for row in &mock_data.recipients {
                repo.insert_one(row).unwrap();
            }
        }
        if inserts.recipient_lists {
            let repo = RecipientListRowRepository::new(connection);
            for row in &mock_data.recipient_lists {
                repo.insert_one(row).unwrap();
            }
        }
        if inserts.recipient_list_members {
            let repo = RecipientListMemberRowRepository::new(connection);
            for row in &mock_data.recipient_list_members {
                repo.insert_one(row).unwrap();
            }
        }
        if inserts.sql_recipient_lists {
            let repo = SqlRecipientListRowRepository::new(connection);
            for row in &mock_data.sql_recipient_lists {
                repo.insert_one(row).unwrap();
            }
        }
        if inserts.notification_configs {
            let repo = NotificationConfigRowRepository::new(connection);
            for row in &mock_data.notification_configs {
                repo.insert_one(row).unwrap();
            }
        }
    }

    mock_data
}

impl MockData {
    pub fn join(mut self, other: MockData) -> MockData {
        let MockData {
            mut user_accounts,
            mut permissions,
            mut recipients,
            mut recipient_lists,
            mut recipient_list_members,
            mut sql_recipient_lists,
            mut notification_configs,
        } = other;

        self.user_accounts.append(&mut user_accounts);
        self.permissions.append(&mut permissions);
        self.recipients.append(&mut recipients);
        self.recipient_lists.append(&mut recipient_lists);
        self.recipient_list_members
            .append(&mut recipient_list_members);
        self.sql_recipient_lists.append(&mut sql_recipient_lists);
        self.notification_configs.append(&mut notification_configs);

        self
    }
}
