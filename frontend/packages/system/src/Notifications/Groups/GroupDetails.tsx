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

const groupDummyData = {
  id: 'friends-id',
  name: 'Friends',
  description:
    'These are some of my closest friends so they need all the notifications',
};

const contactsDummyData = [
  {
    id: 'lache-id',
    name: 'Laché',
    type: 'email',
    address: 'lache@msupply.foundation',
  },
  {
    id: 'james-id',
    name: 'James',
    type: 'email',
    address: 'james@msupply.foundation',
  },
  {
    id: 'mai-id',
    name: 'Mai',
    type: 'email',
    address: 'mai@msupply.foundation',
  },
  {
    id: 'telegram-id',
    name: 'CC Notifications',
    type: 'telegram',
    address: 'abc123',
  },
  { id: '1-id', name: 'name', type: 'telegram', address: 'x' },
  { id: '2-id', name: 'name', type: 'telegram', address: 'x' },
  { id: '3-id', name: 'name', type: 'telegram', address: 'x' },
  { id: '4-id', name: 'name', type: 'telegram', address: 'x' },
  { id: '5-id', name: 'name', type: 'telegram', address: 'x' },
  { id: '6-id', name: 'name', type: 'telegram', address: 'x' },
  { id: '7-id', name: 'name', type: 'telegram', address: 'x' },
  { id: '8-id', name: 'name', type: 'telegram', address: 'x' },
  { id: '9-id', name: 'name', type: 'telegram', address: 'x' },
  { id: '1-i', name: 'name', type: 'telegram', address: 'x' },
  { id: '2-i', name: 'name', type: 'telegram', address: 'x' },
  { id: '3-i', name: 'name', type: 'telegram', address: 'x' },
  { id: '4-i', name: 'name', type: 'telegram', address: 'x' },
  { id: '5-i', name: 'name', type: 'telegram', address: 'x' },
  { id: '6-i', name: 'name', type: 'telegram', address: 'x' },
  { id: '7-i', name: 'name', type: 'telegram', address: 'x' },
  { id: '8-i', name: 'name', type: 'telegram', address: 'x' },
  { id: '9-i', name: 'name', type: 'telegram', address: 'x' },
];

export const GroupDetails: React.FC = () => {
  const t = useTranslation('system');
  const x = useBreadcrumbs();

  useEffect(() => {
    if (!x.suffix) {
      x.setSuffix(groupDummyData.name);
    }
  }, [x.suffix]);

  const columns = useColumns([
    { key: 'name', label: 'label.name' },
    { key: 'type', label: 'label.type' },
    { key: 'address', label: 'label.address' },
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
            {groupDummyData.name}
          </Typography>
          <Typography sx={{ color: 'gray.dark' }}>
            {groupDummyData.description}
          </Typography>
        </Box>
      </Paper>
      <Box sx={{ flex: '1', overflow: 'auto' }}>
        <TableProvider createStore={createTableStore}>
          <DataTable
            columns={columns}
            data={contactsDummyData}
            isError={false}
            isLoading={false}
            noDataElement={
              <NothingHere body={t('error.no-notification-group-members')} />
            }
          />
        </TableProvider>
      </Box>
    </Box>
  );
};
