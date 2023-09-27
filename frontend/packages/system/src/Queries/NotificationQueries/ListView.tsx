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
  createTableStore,
  useColumns,
} from '@common/ui';
import { useEditModal, useQueryParamsState } from '@common/hooks';
import React from 'react';
import { useDeleteNotificationQuery, useNotificationQuerys } from '../api';
import { NotificationQueryRowFragment } from '../api/operations.generated';
import { CreateNotificationQueryModal } from './CreateNotificationQueryModal';
import { useNavigate } from 'packages/common/src';

export const ListView = () => {
  const t = useTranslation('system');
  const navigate = useNavigate();

  const { filter, queryParams, updatePaginationQuery, updateSortQuery } =
    useQueryParamsState();

  const columns = useColumns<NotificationQueryRowFragment>(
    [
      { key: 'name', label: 'label.name' },
      {
        key: 'description',
        label: 'label.description',
        maxWidth: 300,
        sortable: false,
      },
      {
        key: 'query',
        label: 'label.query',
        maxWidth: 300,
        sortable: false,
      },
      {
        key: 'requiredParameters',
        label: 'label.parameters',
        maxWidth: 300,
        sortable: false,
      },
      'selection',
    ],
    {
      onChangeSortBy: updateSortQuery,
      sortBy: queryParams.sortBy,
    },
    [updateSortQuery, queryParams.sortBy]
  );

  const { mutateAsync: deleteNotificationQuery, invalidateQueries } =
    useDeleteNotificationQuery();

  const { isOpen, onClose, onOpen } =
    useEditModal<NotificationQueryRowFragment>();

  const { data, isError, isLoading } = useNotificationQuerys(queryParams);
  const recipientLists = data?.nodes ?? [];

  const pagination = {
    page: queryParams.page,
    offset: queryParams.offset,
    first: queryParams.first,
  };

  return (
    <>
      <CreateNotificationQueryModal
        isOpen={isOpen}
        onClose={() => {
          onClose();
        }}
      />
      <AppBarButtonsPortal>
        <LoadingButton
          isLoading={false}
          startIcon={<PlusCircleIcon />}
          onClick={() => onOpen()}
        >
          {t('label.new-query')}
        </LoadingButton>
      </AppBarButtonsPortal>

      <TableProvider createStore={createTableStore}>
        <AppBarContentPortal sx={{ paddingBottom: '16px', flex: 1 }}>
          <SearchAndDeleteToolbar
            data={recipientLists}
            filter={filter}
            deleteItem={deleteNotificationQuery}
            invalidateQueries={invalidateQueries}
          />
        </AppBarContentPortal>
        <DataTable
          pagination={{ ...pagination, total: data?.totalCount }}
          onChangePage={updatePaginationQuery}
          columns={columns}
          data={recipientLists}
          isError={isError}
          isLoading={isLoading}
          onRowClick={list => navigate(list.id)}
          noDataElement={<NothingHere body={t('error.no-recipient-lists')} />}
        />
      </TableProvider>
    </>
  );
};
