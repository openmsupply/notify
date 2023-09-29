import React, { FC } from 'react';
import {
  useColumns,
  useTranslation,
  StringUtils,
  useEditModal,
  Table,
  TableHead,
  TableRow,
  TableCell,
  TableBody,
  IconButton,
  EditIcon,
  LocaleKey,
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
      accessor: ({ rowData }) => StringUtils.ellipsis(rowData?.query, 35),
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
      {isLoading ? (
        <div>Loading Queries...</div>
      ) : (
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

          <Table>
            <TableHead>
              <TableRow>
                {columns.map(column => (
                  <TableCell
                    key={`header-${column.key}`}
                    role="columnheader"
                    sx={{
                      backgroundColor: 'transparent',
                      borderBottom: '0px',
                      paddingLeft: '16px',
                      paddingRight: '16px',
                      fontWeight: 'bold',
                      fontSize: '14px',
                    }}
                  >
                    {t(column.label as LocaleKey)}
                  </TableCell>
                ))}
              </TableRow>
            </TableHead>
            <TableBody>
              {selectedQueries.map((row, idx) => (
                <TableRow key={`row-${idx}`}>
                  {columns.map(column => (
                    <TableCell key={`row-${idx}-${column}`}>
                      {
                        column.accessor({
                          rowData: row,
                          rows: selectedQueries,
                        }) as string
                      }
                    </TableCell>
                  ))}
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </>
      )}
    </>
  );
};
