import React, { FC } from 'react';
import {
  TableProvider,
  DataTable,
  useColumns,
  createTableStore,
  useTranslation,
  useFormatDateTime,
  ColumnFormat,
  Formatter,
} from '@notify-frontend/common';
import { LogRowFragment } from '../api';

type LogListProps = { records: LogRowFragment[] };

export const LogList: FC<LogListProps> = ({ records }) => {
  const t = useTranslation();
  const { localisedTime } = useFormatDateTime();

  const columns = useColumns<LogRowFragment>([
    {
      key: 'datetime',
      label: 'label.date',
      sortType: 'datetime',
      format: ColumnFormat.Date,
      width: 150,
      sortable: false,
    },
    {
      key: 'time',
      label: 'label.time',
      width: 150,
      sortable: false,
      accessor: ({ rowData }) => localisedTime(rowData.datetime),
    },
    {
      key: 'username',
      label: 'label.username',
      sortable: false,
      accessor: ({ rowData }) => rowData?.user?.username ?? '',
    },
    {
      key: 'logType',
      label: 'label.event',
      sortable: false,
      accessor: ({ rowData }) =>
        t(Formatter.logTypeTranslation(rowData.recordType), {
          defaultValue: rowData.recordType,
        }),
    },
  ]);

  return (
    <TableProvider createStore={createTableStore}>
      <DataTable
        columns={columns}
        data={records}
        noDataMessage={t('messages.no-log-entries')}
      />
    </TableProvider>
  );
};
