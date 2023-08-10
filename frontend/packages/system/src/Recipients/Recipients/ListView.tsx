import React from 'react';
import { useTranslation } from '@common/intl';
import { Box } from '@common/ui';
import { RecipientTable } from './RecipientTable';

const recipientsDummyData = [
  {
    id: 'lache-id',
    name: 'Laché',
    notificationType: 'email',
    toAddress: 'lache@msupply.foundation',
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

export const ListView = () => {
  const t = useTranslation('system');

  return (
    <Box sx={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
      <RecipientTable
        recipients={recipientsDummyData}
        nothingHereMessage={t('error.no-recipients')}
      />
    </Box>
  );
};
