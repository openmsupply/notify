import React from 'react';
import { BasicTextInput, Grid, useTranslation } from '@notify-frontend/common';

type CCNotificationEditFormProps = {
  onUpdate: (patch: Record<string, unknown>) => void;
};

export const CCNotificationEditForm = ({
  onUpdate,
}: CCNotificationEditFormProps) => {
  const t = useTranslation(['system']);

  return (
    <Grid flexDirection="column" display="flex" gap={2}>
      <BasicTextInput
        autoFocus
        value={'Name'}
        required
        onChange={e => onUpdate({ username: e.target.value })}
        label={t('label.name')}
        InputLabelProps={{ shrink: true }}
      />
    </Grid>
  );
};
