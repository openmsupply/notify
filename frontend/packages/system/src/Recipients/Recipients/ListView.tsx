import React from 'react';
import { useTranslation } from '@common/intl';
import {
  AppBarButtonsPortal,
  AppBarContentPortal,
  DataTable,
  LoadingButton,
  NothingHere,
  PlusCircleIcon,
  SearchAndDeleteToolbar,
  TableProvider,
  Typography,
  createTableStore,
  useColumns,
} from '@common/ui';
import {
  RecipientRowFragment,
  useDeleteRecipient,
  useRecipients,
} from '../api';
import { useEditModal, useQueryParamsState } from '@common/hooks';
import { RecipientEditModal } from './RecipientEditModal';

export const ListView = () => {
  const t = useTranslation('system');
  const { filter, queryParams, updatePaginationQuery, updateSortQuery } =
    useQueryParamsState();

  const { isOpen, entity, mode, onClose, onOpen } =
    useEditModal<RecipientRowFragment>();

  const columns = useColumns<RecipientRowFragment>(
    [
      { key: 'name', label: 'label.name' },
      {
        key: 'notificationType',
        label: 'label.type',
        sortable: false,
        Cell: props => (
          <Typography>
            {t(`label.notification-type-${props.rowData.notificationType}`)}
          </Typography>
        ),
      },
      { key: 'toAddress', label: 'label.address' },
      'selection',
    ],
    { sortBy: queryParams.sortBy, onChangeSortBy: updateSortQuery },
    [queryParams.sortBy, updateSortQuery]
  );

  const { data, isError, isLoading } = useRecipients(queryParams);
  const recipients = data?.nodes ?? [];

  const { mutateAsync: deleteRecipient, invalidateQueries } =
    useDeleteRecipient();

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
        <AppBarContentPortal sx={{ paddingBottom: '16px', flex: 1 }}>
          <SearchAndDeleteToolbar
            data={recipients}
            filter={filter}
            deleteItem={deleteRecipient}
            invalidateQueries={invalidateQueries}
          />
        </AppBarContentPortal>

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
