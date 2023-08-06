import React, { useEffect } from 'react';
import { useTranslation } from '@common/intl';
import { useBreadcrumbs } from '@common/hooks';
import {
  DataTable,
  NothingHere,
  TableProvider,
  createTableStore,
  useColumns,
} from '@common/ui';

const groupDummyData = { id: 'friends-id', name: 'Friends' };

const contactsDummyData = [
  {
    id: 'lache-id',
    name: 'Laché',
    type: 'email',
    address: 'lache@msupply.foundation',
  },
  {
    id: 'james-id',
    name: 'James',
    type: 'email',
    address: 'james@msupply.foundation',
  },
  {
    id: 'mai-id',
    name: 'Mai',
    type: 'email',
    address: 'mai@msupply.foundation',
  },
  {
    id: 'telegram-id',
    name: 'CC Notifications',
    type: 'telegram',
    address: 'abc123',
  },
];

export const GroupDetails: React.FC = () => {
  const t = useTranslation('system');
  const x = useBreadcrumbs();

  useEffect(() => {
    if (!x.suffix) {
      x.setSuffix(groupDummyData.name);
    }
  }, [x.suffix]);

  const columns = useColumns([
    { key: 'name', label: 'label.name' },
    { key: 'type', label: 'label.type' },
    { key: 'address', label: 'label.address' },
  ]);

  return (
    <TableProvider createStore={createTableStore}>
      <DataTable
        columns={columns}
        data={contactsDummyData}
        isError={false}
        isLoading={false}
        noDataElement={
          <NothingHere body={t('error.no-notification-group-members')} />
        }
      />
    </TableProvider>
  );
};
