import React from 'react';
import {
  BasicTextInput,
  Grid,
  useTranslation,
  Typography,
  ToggleButton,
  NotificationTypeNode,
  EnvUtils,
} from '@notify-frontend/common';
import { DraftRecipient } from './types';
import { ToggleButtonGroup } from '@mui/material';
import { ExternalURL } from 'packages/config/src';

type RecipientEditFormProps = {
  draft: DraftRecipient;
  onUpdate: (patch: Partial<DraftRecipient>) => void;
};

export const RecipientEditForm = ({
  draft,
  onUpdate,
}: RecipientEditFormProps) => {
  const t = useTranslation('system');

  const docsUrl = `${ExternalURL.PublicDocs}${
    EnvUtils.mapRoute(location.pathname).docs
  }`;

  return (
    <Grid flexDirection="column" display="flex" gap={2}>
      <ToggleButtonGroup exclusive sx={{ margin: '0 auto' }}>
        <ToggleButton
          label={t('label.email')}
          value={NotificationTypeNode.Email}
          selected={draft.notificationType === NotificationTypeNode.Email}
          onClick={() => {
            onUpdate({ notificationType: NotificationTypeNode.Email });
          }}
        />
        <ToggleButton
          label={t('label.telegram')}
          value={NotificationTypeNode.Telegram}
          selected={draft.notificationType === NotificationTypeNode.Telegram}
          onClick={() => {
            onUpdate({ notificationType: NotificationTypeNode.Telegram });
          }}
        />
      </ToggleButtonGroup>
      {draft.notificationType === NotificationTypeNode.Email ? (
        <>
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
        </>
      ) : (
        <>
          <Typography sx={{ color: 'gray.dark' }}>
            {t('text.telegram-recipient-creation-1')}
          </Typography>
          <Typography sx={{ color: 'gray.dark' }}>
            {t('text.telegram-recipient-creation-2')}{' '}
            <a rel="noreferrer" target="_blank" href={docsUrl}>
              {t('text.here')}
            </a>
            {'.'}
          </Typography>
        </>
      )}
    </Grid>
  );
};