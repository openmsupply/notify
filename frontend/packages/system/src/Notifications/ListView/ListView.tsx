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
import { NotificationConfigType } from '../Modals/SelectNotificationConfigModal';
import { useEditModal } from '@common/hooks';

// TODO: this will be backend NotificationConfig type?
type NotificationConfig = {
  id: string;
  title: string;
  configType: NotificationConfigType;
};

const notifications: NotificationConfig[] = [
  {
    id: 'notification-1',
    title: 'CC Notification 1',
    configType: NotificationConfigType.ColdChain,
  },
  {
    id: 'notification-2',
    title: 'CC Notification 2',
    configType: NotificationConfigType.ColdChain,
  },
  {
    id: 'notification-3',
    title: 'CC Notification 3',
    configType: NotificationConfigType.ColdChain,
  },
];
export const ListView = () => {
  const t = useTranslation('system');

  const columns = useColumns<NotificationConfig>([
    { key: 'title', label: 'label.title' },
    { key: 'configType', label: 'label.type' },
  ]);

  const { isOpen, onClose, entity, onOpen } =
    useEditModal<NotificationConfig>();

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
