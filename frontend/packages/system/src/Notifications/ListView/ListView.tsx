import React from 'react';
import { useTranslation } from '@common/intl';
import {
  AppBarButtonsPortal,
  DataTable,
  LoadingButton,
  NothingHere,
  PlusCircleIcon,
  TableProvider,
  Typography,
  createTableStore,
  useColumns,
} from '@common/ui';
import { NotificationsModal } from '../Modals/NotificationsModal';
import { useEditModal } from '@common/hooks';
import { NotificationConfigRowFragment, useNotificationConfigs } from '../api';

export const ListView = () => {
  const t = useTranslation('system');

  const columns = useColumns<NotificationConfigRowFragment>([
    { key: 'title', label: 'label.title' },
    {
      key: 'kind',
      label: 'label.kind',
      Cell: props => (
        <Typography>{t(`config-kind.${props.rowData.kind}`)}</Typography>
      ),
    },
  ]);

  const {
    data: notificationConfigs,
    isError,
    isLoading,
  } = useNotificationConfigs();

  const { isOpen, onClose, entity, onOpen } =
    useEditModal<NotificationConfigRowFragment>();

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
          data={notificationConfigs?.nodes ?? []}
          isError={isError}
          isLoading={isLoading}
          onRowClick={onOpen}
          noDataElement={<NothingHere body={t('messages.no-notifications')} />}
        />
      </TableProvider>
    </>
  );
};
