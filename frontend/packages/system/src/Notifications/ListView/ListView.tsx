import React, { useState } from 'react';
import { useTranslation } from '@common/intl';
import {
  AppBarButtonsPortal,
  DataTable,
  LoadingButton,
  NothingHere,
  PlusCircleIcon,
  TableProvider,
  createTableStore,
  useColumns,
} from '@common/ui';
import { NotificationsModal } from '../NotificationsModal';

type Notification = {
  id: string;
  name: string;
};

const notifications: Notification[] = [
  { id: 'notification-1', name: 'CC Notifcation 1' },
  { id: 'notification-2', name: 'CC Notifcation 2' },
  { id: 'notification-3', name: 'CC Notifcation 3' },
];
export const ListView = () => {
  const t = useTranslation('system');
  const [open, setOpen] = useState(false);

  const columns = useColumns<Notification>([
    { key: 'name', label: 'label.name' },
  ]);

  return (
    <>
      <NotificationsModal isOpen={open} onClose={() => setOpen(false)} />
      <AppBarButtonsPortal>
        <LoadingButton
          isLoading={false}
          startIcon={<PlusCircleIcon />}
          onClick={() => setOpen(true)}
        >
          {t('label.new-notification')}
        </LoadingButton>
      </AppBarButtonsPortal>
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
