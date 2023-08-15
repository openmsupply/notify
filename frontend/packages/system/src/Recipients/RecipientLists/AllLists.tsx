import { useTranslation } from '@common/intl';
import {
  DataTable,
  NothingHere,
  SearchAndDeleteToolbar,
  TableProvider,
  createTableStore,
  useColumns,
} from '@common/ui';
import { useNavigate, useQueryParamsState } from 'packages/common/src';
import React from 'react';
import { useRecipientLists } from '../api';
import { RecipientListRowFragment } from '../api/operations.generated';

export const AllLists = () => {
  const t = useTranslation('system');
  const navigate = useNavigate();

  const { filter, queryParams, updatePaginationQuery, updateSortQuery } =
    useQueryParamsState();

  const columns = useColumns<RecipientListRowFragment>(
    [
      { key: 'name', label: 'label.name' },
      {
        key: 'description',
        label: 'label.description',
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

  const deleteRecipientList = async () => null;
  // const { mutateAsync: deleteRecipientList } = useDeleteRecipientList();

  const { data, isError, isLoading } = useRecipientLists(queryParams);
  const recipientLists = data?.nodes ?? [];

  const pagination = {
    page: queryParams.page,
    offset: queryParams.offset,
    first: queryParams.first,
  };

  return (
    <>
      <TableProvider createStore={createTableStore}>
        <SearchAndDeleteToolbar
          data={recipientLists}
          filter={filter}
          deleteItem={deleteRecipientList}
          searchFilterKey="name"
          asStringFilterRule
        />
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
