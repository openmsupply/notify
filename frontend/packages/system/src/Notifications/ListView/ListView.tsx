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
import { NotificationsModal } from '../Modals/NotificationsModal';
import { useEditModal } from '@common/hooks';
import { BaseNotificationConfig, NotificationConfigType } from '../types';

const notifications: BaseNotificationConfig[] = [
  {
    id: 'notification-1',
    title: 'CC Notification 1',
    configType: NotificationConfigType.ColdChain,
    recipientIds: [],
    recipientListIds: [],
  },
  {
    id: 'notification-2',
    title: 'CC Notification 2',
    configType: NotificationConfigType.ColdChain,
    recipientIds: [],
    recipientListIds: [],
  },
  {
    id: 'notification-3',
    title: 'CC Notification 3',
    configType: NotificationConfigType.ColdChain,
    recipientIds: [],
    recipientListIds: [],
  },
];
export const ListView = () => {
  const t = useTranslation('system');

  const columns = useColumns<BaseNotificationConfig>([
    { key: 'title', label: 'label.title' },
    { key: 'configType', label: 'label.type' },
  ]);

  const { isOpen, onClose, entity, onOpen } =
    useEditModal<BaseNotificationConfig>();

  return (
    <>
      <NotificationsModal isOpen={isOpen} onClose={onClose} entity={entity} />
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
          onRowClick={onOpen}
          noDataElement={<NothingHere body={t('messages.no-notifications')} />}
        />
      </TableProvider>
    </>
  );
};
