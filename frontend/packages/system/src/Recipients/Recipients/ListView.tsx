import React from 'react';
import { useTranslation } from '@common/intl';
import { Box } from '@common/ui';
import { RecipientTable } from './RecipientTable';
import { useRecipients } from '../api';

export const ListView = () => {
  const t = useTranslation('system');

  const { data: recipients } = useRecipients();

  return (
    <Box sx={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
      <RecipientTable
        recipients={recipients?.nodes ?? []}
        nothingHereMessage={t('error.no-recipients')}
      />
    </Box>
  );
};
