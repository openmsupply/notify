import React from 'react';
import {
  BasicTextInput,
  Grid,
  useTranslation,
  Typography,
} from '@notify-frontend/common';
import { DraftRecipient } from './types';

type RecipientEditFormProps = {
  draft: DraftRecipient;
  onUpdate: (patch: Partial<DraftRecipient>) => void;
};

export const RecipientEditForm = ({
  draft,
  onUpdate,
}: RecipientEditFormProps) => {
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

      {/* Validate email address */}
      <BasicTextInput
        value={draft.toAddress}
        onChange={e => onUpdate({ toAddress: e.target.value })}
        label={t('label.email')}
        InputLabelProps={{ shrink: true }}
        required
      />

      <Typography>
        * TODO: something like: To create a Telegram recipient, add your Notify
        Bot to a Telegram group.
      </Typography>
    </Grid>
  );
};
