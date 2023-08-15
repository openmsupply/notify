import React, { useEffect } from 'react';
import { useTranslation } from '@common/intl';
import { useBreadcrumbs, useQueryParamsState } from '@common/hooks';
import {
  Box,
  DataTable,
  NothingHere,
  Paper,
  TableProvider,
  Typography,
  createTableStore,
  useColumns,
} from '@common/ui';
import { useRecipientLists } from '../api';
import { useParams } from 'packages/common/src';

export const DetailView = () => {
  const t = useTranslation('system');
  const urlParams = useParams();
  const { suffix, setSuffix } = useBreadcrumbs();

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
