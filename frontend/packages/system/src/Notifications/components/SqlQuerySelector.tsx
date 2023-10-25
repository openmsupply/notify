import React, { FC } from 'react';
import {
  useColumns,
  useTranslation,
  StringUtils,
  useEditModal,
  EditIcon,
  DataTable,
  TableProvider,
  createTableStore,
  LoadingButton,
  Box,
} from '@notify-frontend/common';
import { NotificationQuerySelectionModal } from './NotificationQuerySelectionModal';
import { NotificationQueryRowFragment } from '../../Queries/api';

type QueryListProps = {
  allQueries: NotificationQueryRowFragment[];
  selectedQueryIds: string[];
  setSelection: (input: {
    notificationQueryIds: string[];
    requiredParameters: string[];
  }) => void;
  isLoading: boolean;
};

export const SqlQuerySelector: FC<QueryListProps> = ({
  allQueries,
  selectedQueryIds,
  setSelection,
  isLoading,
}) => {
  const t = useTranslation('system');

  const { isOpen, onClose, onOpen } = useEditModal();

  const columns = useColumns<NotificationQueryRowFragment>([
    {
      key: 'referenceName',
      label: 'label.reference-name',
      width: 200,
      sortable: false,
    },
    {
      key: 'name',
      label: 'label.name',
      width: 150,
      sortable: false,
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
      <TableProvider createStore={createTableStore}>
        <DataTable
          isDisabled={false}
          isLoading={isLoading}
          columns={columns}
          data={selectedQueries}
          noDataMessage={t('message.no-queries-selected')}
        />
      </TableProvider>
      <Box padding={2}>
        <LoadingButton
          disabled={false}
          onClick={onOpen}
          isLoading={false}
          startIcon={<EditIcon />}
        >
          {t('label.select-queries')}
        </LoadingButton>
      </Box>
    </>
  );
};
