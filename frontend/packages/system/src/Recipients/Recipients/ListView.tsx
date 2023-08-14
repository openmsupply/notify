import React from 'react';
import { useTranslation } from '@common/intl';
import {
  DataTable,
  NothingHere,
  TableProvider,
  createTableStore,
  useColumns,
} from '@common/ui';
import { useRecipients } from '../api';
import { useQueryParamsState } from '@common/hooks';
import { SearchAndDeleteToolbar } from '../../shared/SearchAndDeleteToolbar';
import { useDeleteRecipient } from '../api/hooks/useDeleteRecipient';

export const ListView = () => {
  const t = useTranslation('system');
  const { filter, queryParams, updatePaginationQuery } = useQueryParamsState();

  // TODO: sort
  const columns = useColumns([
    { key: 'name', label: 'label.name' },
    { key: 'notificationType', label: 'label.type' },
    { key: 'toAddress', label: 'label.address' },
    'selection',
  ]);

  const { data: recipients, isError, isLoading } = useRecipients(queryParams);

  const { mutateAsync: deleteRecipient } = useDeleteRecipient();

  const pagination = {
    page: queryParams.page,
    offset: queryParams.offset,
    first: queryParams.first,
  };

  return (
    <TableProvider createStore={createTableStore}>
      <SearchAndDeleteToolbar
        data={recipients?.nodes ?? []}
        filter={filter}
        deleteItem={deleteRecipient}
      />
      <DataTable
        columns={columns}
        data={recipients?.nodes ?? []}
        isError={isError}
        isLoading={isLoading}
        noDataElement={<NothingHere body={t('error.no-recipients')} />}
        pagination={{ ...pagination, total: recipients?.totalCount }}
        onChangePage={updatePaginationQuery}
      />
    </TableProvider>
  );
};
