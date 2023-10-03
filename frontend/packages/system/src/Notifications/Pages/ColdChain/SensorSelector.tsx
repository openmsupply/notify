import React, { FC } from 'react';
import {
  TableProvider,
  DataTable,
  useColumns,
  createTableStore,
  useTranslation,
  useEditModal,
  IconButton,
  EditIcon,
} from '@notify-frontend/common';
import { SensorData, sensorDisplayName } from '../../api';
import { SensorSelectionModal } from './SensorSelectionModal';

type SensorSelectorProps = {
  records: SensorData[];
  selectedIds: string[];
  setSelection: (input: { sensorIds: string[] }) => void;
  isLoading: boolean;
};

export const SensorSelector: FC<SensorSelectorProps> = ({
  records,
  selectedIds,
  setSelection,
  isLoading,
}) => {
  const t = useTranslation('system');

  const { isOpen, onClose, onOpen } = useEditModal();

  const columns = useColumns<SensorData>([
    {
      key: 'name',
      label: 'label.name',
      width: 150,
      sortable: true,
      accessor: ({ rowData }) => {
        return sensorDisplayName(rowData);
      },
    },
  ]);

  const selectedRecords = (records ?? []).filter(s =>
    selectedIds.includes(s.id)
  );

  return (
    <>
      <SensorSelectionModal
        sensors={records ?? []}
        initialSelectedIds={selectedIds}
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
          columns={columns}
          isLoading={isLoading}
          data={selectedRecords}
          noDataMessage={t('message.no-sensors-selected')}
          dense
        />
      </TableProvider>
    </>
  );
};
