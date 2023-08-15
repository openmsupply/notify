import { useTranslation } from '@common/intl';
import {
  DataTable,
  NothingHere,
  TableProvider,
  createTableStore,
  useColumns,
} from '@common/ui';
import { useNavigate } from 'packages/common/src';
import React from 'react';

const dummyData = [
  { id: 'friends-id', name: 'Friends', description: 'My good friends' },
  { id: 'foes-id', name: 'Foes', description: 'Keep your enemies closer' },
  { id: 'fries-id', name: 'Fries', description: 'With ketchup please' },
  {
    id: 'kids-id',
    name: 'Kids',
    description: 'This is a description about the list',
  },
  {
    id: 'mates-id',
    name: 'Mates',
    description: 'This is a description about the list',
  },
  {
    id: 'lads-id',
    name: 'Lads',
    description: 'This is a description about the list',
  },
  {
    id: 'homies-id',
    name: 'Homies',
    description: 'This is a description about the list',
  },
  {
    id: 'cuties-id',
    name: 'Cuties',
    description: 'This is a description about the list',
  },
  {
    id: 'bros-id',
    name: 'Bros',
    description: 'This is a description about the list',
  },
];

export const AllLists = () => {
  const t = useTranslation('system');
  const navigate = useNavigate();

  // const columns = useColumns<RecipientListRowFragment>(
  const columns = useColumns(
    [
      { key: 'name', label: 'label.name' },
      {
        key: 'description',
        label: 'label.description',
        sortable: false,
      },
      // 'selection',
    ]
    // {
    //   onChangeSortBy: updateSortQuery,
    //   sortBy,
    // },
    // [updateSortQuery, sortBy]
  );

  return (
    <>
      <TableProvider createStore={createTableStore}>
        <DataTable
          // pagination={{ ...pagination, total: data?.totalCount }}
          // onChangePage={updatePaginationQuery}
          columns={columns}
          data={dummyData}
          // isError={isError}
          // isLoading={isLoading}
          onRowClick={list => navigate(list.id)}
          noDataElement={<NothingHere body={t('error.no-recipient-lists')} />}
        />
      </TableProvider>
    </>
  );
};
