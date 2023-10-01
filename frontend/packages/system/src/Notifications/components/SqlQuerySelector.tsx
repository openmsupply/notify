import React, { FC } from 'react';
import {
  useColumns,
  useTranslation,
  StringUtils,
  useEditModal,
  IconButton,
  EditIcon,
  DataTable,
  TableProvider,
  createTableStore,
} from '@notify-frontend/common';
import { NotificationQuerySelectionModal } from './NotificationQuerySelectionModal';
import { NotificationQueryRowFragment } from '../../Queries/api';

type QueryListProps = {
  allQueries: NotificationQueryRowFragment[];
  selectedQueryIds: string[];
  setSelection: (input: { notificationQueryIds: string[] }) => void;
  isLoading: boolean;
};

export const SqlQuerySelector: FC<QueryListProps> = ({
  allQueries,
  selectedQueryIds,
  setSelection,
  isLoading,
}) => {
  const t = useTranslation();

  const { isOpen, onClose, onOpen } = useEditModal();

  const columns = useColumns<NotificationQueryRowFragment>([
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
      accessor: ({ rowData }) => StringUtils.ellipsis(rowData?.query, 100),
    },
    {
      key: 'requiredParameters',
      label: 'label.parameters',
      sortable: false,
      accessor: ({ rowData }) => rowData?.requiredParameters.join(', '),
    },
  ]);

  const selectedQueries = (allQueries ?? []).filter(q =>
    selectedQueryIds.includes(q.id)
  );

  return (
    <>
      <NotificationQuerySelectionModal
        sqlQueries={allQueries}
        initialSelectedIds={selectedQueryIds}
        isOpen={isOpen}
        onClose={onClose}
        setSelection={setSelection}
      />
      <IconButton
        icon={<EditIcon />}
        label={t('label.edit')}
        onClick={onOpen}
      />
      <TableProvider createStore={createTableStore}>
        <DataTable
          isDisabled={false}
          isLoading={isLoading}
          columns={columns}
          data={selectedQueries}
          noDataMessage="No Queries Configured"
        />
      </TableProvider>
    </>
  );
};
