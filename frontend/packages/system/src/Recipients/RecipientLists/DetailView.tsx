import React, { useEffect } from 'react';
import { useTranslation } from '@common/intl';
import { useBreadcrumbs } from '@common/hooks';
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

const listDummyData = {
  id: 'friends-id',
  name: 'Friends',
  description:
    'These are some of my closest friends so they need all the notifications',
};

const recipientsDummyData = [
  {
    id: 'lache-id',
    name: 'Laché',
    notificationType: 'email',
    address: 'lache@msupply.foundation',
  },
  {
    id: 'james-id',
    name: 'James',
    notificationType: 'email',
    toAddress: 'james@msupply.foundation',
  },
  {
    id: 'mai-id',
    name: 'Mai',
    notificationType: 'email',
    toAddress: 'mai@msupply.foundation',
  },
  {
    id: 'telegram-id',
    name: 'CC Notifications',
    notificationType: 'telegram',
    toAddress: 'abc123',
  },
  { id: '1-id', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '2-id', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '3-id', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '4-id', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '5-id', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '6-id', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '7-id', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '8-id', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '9-id', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '1-i', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '2-i', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '3-i', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '4-i', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '5-i', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '6-i', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '7-i', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '8-i', name: 'name', notificationType: 'telegram', toAddress: 'x' },
  { id: '9-i', name: 'name', notificationType: 'telegram', toAddress: 'x' },
];

export const DetailView = () => {
  const t = useTranslation('system');
  const x = useBreadcrumbs();

  useEffect(() => {
    if (!x.suffix) {
      x.setSuffix(listDummyData.name);
    }
  }, [x.suffix]);

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
            {listDummyData.name}
          </Typography>
          <Typography sx={{ color: 'gray.dark' }}>
            {listDummyData.description}
          </Typography>
        </Box>
      </Paper>
      <Box sx={{ flex: '1', overflow: 'auto' }}>
        <TableProvider createStore={createTableStore}>
          <DataTable
            columns={columns}
            data={recipientsDummyData}
            isError={false}
            isLoading={false}
            noDataElement={
              <NothingHere body={t('error.no-recipient-list-members')} />
            }
          />
        </TableProvider>
      </Box>
    </Box>
  );
};
