import React, { FC } from 'react';
import {
  TableProvider,
  DataTable,
  useColumns,
  createTableStore,
  useTranslation,
  StringUtils,
} from '@notify-frontend/common';

interface SqlQuery {
  id: string;
  name: string;
  query: string;
  parameters: string[];
}

type QueryListProps = { records: SqlQuery[] };

export const SqlQuerySelector: FC<QueryListProps> = ({ records }) => {
  const t = useTranslation();

  const columns = useColumns<SqlQuery>([
    {
      key: 'name',
      label: 'label.name',
      width: 150,
      sortable: true,
    },
    {
      key: 'query',
      label: 'label.query',
      width: 150,
      sortable: false,
      accessor: ({ rowData }) => StringUtils.ellipsis(rowData?.query, 35),
    },
    {
      key: 'parameters',
      label: 'label.parameters',
      sortable: false,
      accessor: ({ rowData }) => rowData?.parameters.join(', '),
    },
  ]);

  return (
    <TableProvider createStore={createTableStore}>
      <DataTable
        columns={columns}
        data={records}
        noDataMessage={t('messages.nothing-selected')}
        dense
      />
    </TableProvider>
  );
};
