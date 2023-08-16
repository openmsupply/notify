import React from 'react';
import { useTranslation } from '@common/intl';
import {
  DataTable,
  NothingHere,
  TableProvider,
  createTableStore,
  useColumns,
} from '@common/ui';

type Notification = {
  id: string;
  name: string;
};

export const ListView = () => {
  const t = useTranslation('system');

  const columns = useColumns<Notification>([
    { key: 'name', label: 'label.name' },
  ]);

  const notifications: Notification[] = [
    { id: 'notification-1', name: 'Notifcation 1' },
    { id: 'notification-2', name: 'Notifcation 2' },
    { id: 'notification-3', name: 'Notifcation 3' },
  ];

  return (
    <>
      <TableProvider createStore={createTableStore}>
        <DataTable
          columns={columns}
          data={notifications}
          isError={false}
          isLoading={false}
          noDataElement={<NothingHere body={t('messages.no-notifications')} />}
        />
      </TableProvider>
    </>
  );
};
