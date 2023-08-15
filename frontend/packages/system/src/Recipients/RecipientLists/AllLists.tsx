import { useTranslation } from '@common/intl';
import {
  AppBarButtonsPortal,
  DataTable,
  LoadingButton,
  NothingHere,
  PlusCircleIcon,
  SearchAndDeleteToolbar,
  TableProvider,
  createTableStore,
  useColumns,
} from '@common/ui';
import {
  ModalMode,
  useEditModal,
  useNavigate,
  useQueryParamsState,
} from 'packages/common/src';
import React from 'react';
import { useDeleteRecipientList, useRecipientLists } from '../api';
import { RecipientListRowFragment } from '../api/operations.generated';
import { RecipientListEditModal } from './RecipientListEditModal';

export const AllLists = () => {
  const t = useTranslation('system');
  const navigate = useNavigate();

  const { filter, queryParams, updatePaginationQuery, updateSortQuery } =
    useQueryParamsState();

  const { isOpen, onClose, onOpen } = useEditModal<RecipientListRowFragment>();

  const columns = useColumns<RecipientListRowFragment>(
    [
      { key: 'name', label: 'label.name' },
      {
        key: 'description',
        label: 'label.description',
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

  const { mutateAsync: deleteRecipientList } = useDeleteRecipientList();

  const { data, isError, isLoading } = useRecipientLists(queryParams);
  const recipientLists = data?.nodes ?? [];

  const pagination = {
    page: queryParams.page,
    offset: queryParams.offset,
    first: queryParams.first,
  };

  return (
    <>
      {isOpen && (
        <RecipientListEditModal
          mode={ModalMode.Create}
          isOpen={isOpen}
          onClose={onClose}
          recipientList={null}
        />
      )}
      <AppBarButtonsPortal>
        <LoadingButton
          isLoading={false}
          startIcon={<PlusCircleIcon />}
          onClick={() => onOpen()}
        >
          {t('label.new-recipient-list')}
        </LoadingButton>
      </AppBarButtonsPortal>

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
