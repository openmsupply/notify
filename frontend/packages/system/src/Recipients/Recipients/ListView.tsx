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
import {
  RecipientRowFragment,
  useDeleteRecipient,
  useRecipients,
} from '../api';
import { useEditModal, useQueryParamsState } from '@common/hooks';
import { SearchAndDeleteToolbar } from '../../shared/SearchAndDeleteToolbar';
import { RecipientEditModal } from './RecipientEditModal';

export const ListView = () => {
  const t = useTranslation('system');
  const { filter, queryParams, updatePaginationQuery } = useQueryParamsState();

  const { isOpen, entity, mode, onClose, onOpen } =
    useEditModal<RecipientRowFragment>();

  // TODO: sort
  const columns = useColumns<RecipientRowFragment>([
    { key: 'name', label: 'label.name' },
    { key: 'notificationType', label: 'label.type' },
    { key: 'toAddress', label: 'label.address' },
    'selection',
  ]);

  const { data, isError, isLoading } = useRecipients(queryParams);
  const recipients = data?.nodes ?? [];

  const { mutateAsync: deleteRecipient } = useDeleteRecipient();

  const pagination = {
    page: queryParams.page,
    offset: queryParams.offset,
    first: queryParams.first,
  };

  return (
    <>
      {isOpen && (
        <RecipientEditModal
          mode={mode}
          isOpen={isOpen}
          onClose={onClose}
          recipient={entity}
        />
      )}
      <AppBarButtonsPortal>
        <LoadingButton
          isLoading={false}
          startIcon={<PlusCircleIcon />}
          onClick={() => onOpen()}
        >
          {t('label.new-recipient')}
        </LoadingButton>
      </AppBarButtonsPortal>

      <TableProvider createStore={createTableStore}>
        <SearchAndDeleteToolbar
          data={recipients}
          filter={filter}
          deleteItem={deleteRecipient}
        />

        <DataTable
          columns={columns}
          data={recipients}
          isError={isError}
          isLoading={isLoading}
          onRowClick={onOpen}
          noDataElement={<NothingHere body={t('error.no-recipients')} />}
          pagination={{ ...pagination, total: data?.totalCount }}
          onChangePage={updatePaginationQuery}
        />
      </TableProvider>
    </>
  );
};
