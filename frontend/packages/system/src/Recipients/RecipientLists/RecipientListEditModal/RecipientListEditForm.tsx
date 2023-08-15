import React from 'react';
import { BasicTextInput, Grid, useTranslation } from '@notify-frontend/common';
import { DraftRecipientList } from './types';

type RecipientListEditFormProps = {
  draft: DraftRecipientList;
  onUpdate: (patch: Partial<DraftRecipientList>) => void;
};

export const RecipientListEditForm = ({
  draft,
  onUpdate,
}: RecipientListEditFormProps) => {
  const t = useTranslation('system');

  return (
    <Grid flexDirection="column" display="flex" gap={2}>
      <BasicTextInput
        autoFocus
        required
        value={draft.name}
        onChange={e => onUpdate({ name: e.target.value })}
        label={t('label.name')}
        InputLabelProps={{ shrink: true }}
      />

      <BasicTextInput
        value={draft.description}
        onChange={e => onUpdate({ description: e.target.value })}
        label={t('label.description')}
        InputLabelProps={{ shrink: true }}
        required
      />
    </Grid>
  );
};
