import React from 'react';
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
import { useEditModal } from '@common/hooks';
import { CCNotificationEditModal } from '../ColdChain/CCNotificationEditModal';

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

  const columns = useColumns<Notification>([
    { key: 'name', label: 'label.name' },
  ]);

  const { isOpen, mode, onClose, onOpen } = useEditModal<Notification>();

  return (
    <>
      {isOpen && (
        <CCNotificationEditModal
          mode={mode}
          isOpen={isOpen}
          onClose={onClose}
        />
      )}
      <AppBarButtonsPortal>
        <LoadingButton
          isLoading={false}
          startIcon={<PlusCircleIcon />}
          onClick={() => onOpen()}
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
