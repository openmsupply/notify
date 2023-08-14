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

export const ListView = () => {
  const t = useTranslation('system');
  const { filter, queryParams, updatePaginationQuery } = useQueryParamsState();

  const columns = useColumns([
    { key: 'name', label: 'label.name' },
    { key: 'notificationType', label: 'label.type' },
    { key: 'toAddress', label: 'label.address' },
  ]);

  const { data: recipients, isError, isLoading } = useRecipients(queryParams);

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
        deleteItem={() => Promise.resolve(undefined)}
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
