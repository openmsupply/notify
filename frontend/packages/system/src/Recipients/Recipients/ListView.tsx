import React from 'react';
import { useTranslation } from '@common/intl';
import { Box } from '@common/ui';
import { RecipientTable } from './RecipientTable';
import { useRecipients } from '../api';
import { Toolbar } from './Toolbar';
import { useQueryParamsState } from '@common/hooks';

export const ListView = () => {
  const t = useTranslation('system');
  const { filter, queryParams } = useQueryParamsState();

  const { data: recipients } = useRecipients(queryParams);

  return (
    <>
      <Toolbar data={recipients?.nodes ?? []} filter={filter} />
      <Box sx={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
        <RecipientTable
          recipients={recipients?.nodes ?? []}
          nothingHereMessage={t('error.no-recipients')}
        />
      </Box>
    </>
  );
};
