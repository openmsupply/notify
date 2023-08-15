import React, { useEffect } from 'react';
import { useTranslation } from '@common/intl';
import {
  ModalMode,
  useBreadcrumbs,
  useEditModal,
  useQueryParamsState,
} from '@common/hooks';
import {
  Box,
  DataTable,
  EditIcon,
  LoadingButton,
  NothingHere,
  Paper,
  TableProvider,
  Typography,
  createTableStore,
  useColumns,
} from '@common/ui';
import { useRecipientLists } from '../api';
import { useParams } from 'packages/common/src';
import { RecipientListEditModal } from './RecipientListEditModal';
import { RecipientListRowFragment } from '../api/operations.generated';

export const DetailView = () => {
  const t = useTranslation('system');
  const urlParams = useParams();
  const { suffix, setSuffix } = useBreadcrumbs();
  const { isOpen, onClose, onOpen, entity } =
    useEditModal<RecipientListRowFragment>();

  const { queryParams } = useQueryParamsState({
    initialFilter: {
      id: {
        equalTo: urlParams['listId'],
      },
    },
  });

  const { data, isError, isLoading } = useRecipientLists(queryParams);
  const list = data?.nodes[0];

  useEffect(() => {
    const listName = list?.name;
    if (!suffix && listName) {
      setSuffix(listName);
    }
  }, [suffix, list]);

  const columns = useColumns([
    { key: 'name', label: 'label.name' },
    { key: 'notificationType', label: 'label.type' },
    { key: 'toAddress', label: 'label.address' },
  ]);

  return (
    <Box sx={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
      {isOpen && (
        <RecipientListEditModal
          mode={ModalMode.Update}
          isOpen={isOpen}
          onClose={onClose}
          recipientList={entity}
        />
      )}
      <Paper
        sx={{
          borderRadius: '16px',
          boxShadow: theme => theme.shadows[1],
          padding: '21px',
          margin: '14px',
          height: 'fit-content',
          backgroundColor: 'background.menu',
          display: 'flex',
          justifyContent: 'space-between',
          gap: '14px',
        }}
      >
        <Box>
          <Typography
            sx={{
              fontSize: '18px',
              fontWeight: 'bold',
              color: 'gray.dark',
            }}
          >
            {list?.name}
          </Typography>
          <Typography sx={{ color: 'gray.dark' }}>
            {list?.description}
          </Typography>
        </Box>
        <LoadingButton
          variant="outlined"
          isLoading={false}
          startIcon={<EditIcon />}
          onClick={() => onOpen(list)}
        >
          {t('label.edit')}
        </LoadingButton>
      </Paper>
      <Box sx={{ flex: '1', overflow: 'auto' }}>
        <TableProvider createStore={createTableStore}>
          <DataTable
            columns={columns}
            data={list?.recipients}
            isError={isError}
            isLoading={isLoading}
            noDataElement={
              <NothingHere body={t('error.no-recipient-list-members')} />
            }
          />
        </TableProvider>
      </Box>
    </Box>
  );
};
