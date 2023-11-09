import React from 'react';
import { useTranslation } from '@common/intl';
import {
  AppBarButtonsPortal,
  AppBarContentPortal,
  DataTable,
  NothingHere,
  RelativeTimeDate,
  TableProvider,
  Tooltip,
  Typography,
  createTableStore,
  useColumns,
} from '@common/ui';
import { useQueryParamsState } from '@common/hooks';
import { NotificationEventRowFragment, useNotificationEvents } from '../api';

import { ConfigKind, useNavigate } from '@notify-frontend/common';
import { FilterBar } from './FilterBar';
import TruncatedTextField from './TruncatedTextField';

type ListViewProps = {
  kind: ConfigKind | null;
};

export const ListView = ({}: ListViewProps) => {
  const t = useTranslation('system');
  const navigate = useNavigate();

  const { filter, queryParams, updatePaginationQuery, updateSortQuery } =
    useQueryParamsState({
      initialSort: {
        key: 'createdAt',
        dir: 'desc',
      },
    });

  const columns = useColumns<NotificationEventRowFragment>(
    [
      {
        key: 'createdAt',
        label: 'label.date',
        Cell: props => (
          <Tooltip title={props.rowData.createdAt}>
            <RelativeTimeDate d={props.rowData.createdAt}></RelativeTimeDate>
          </Tooltip>
        ),
      },
      { key: 'title', label: 'label.title' },
      {
        key: 'status',
        label: 'label.status',
        sortable: true,
        Cell: props => <Typography>{props.rowData.status}</Typography>,
      },
      { key: 'toAddress', label: 'label.address' },
      {
        key: 'message',
        label: 'label.message',
        sortable: false,
        Cell: props => (
          <TruncatedTextField
            text={props.rowData.message ?? 'No Message Recorded'}
            maxLength={30}
          />
        ),
      },
      {
        key: 'notificationType',
        label: 'label.notification-type',
        sortable: false,
      },
      {
        key: 'errorMessage',
        label: 'error',
        sortable: true,
        Cell: props => (
          <TruncatedTextField
            text={props.rowData.errorMessage ?? ''}
            maxLength={10}
          />
        ),
      },
    ],
    { sortBy: queryParams.sortBy, onChangeSortBy: updateSortQuery },
    [queryParams.sortBy, updateSortQuery]
  );

  const { data, isError, isLoading } = useNotificationEvents(queryParams);
  const notificationEvents = data?.nodes ?? [];

  const pagination = {
    page: queryParams.page,
    offset: queryParams.offset,
    first: queryParams.first,
  };

  return (
    <>
      <AppBarButtonsPortal></AppBarButtonsPortal>
      <TableProvider createStore={createTableStore}>
        <AppBarContentPortal sx={{ paddingBottom: '16px', flex: 1 }}>
          <FilterBar filter={filter} />
        </AppBarContentPortal>
        <DataTable
          columns={columns}
          data={notificationEvents}
          isError={isError}
          isLoading={isLoading}
          onRowClick={evt => navigate(evt.id)}
          noDataElement={
            <NothingHere body={t('messages.no-events-matching-status')} />
          }
          pagination={pagination}
          onChangePage={updatePaginationQuery}
        />
      </TableProvider>
    </>
  );
};
