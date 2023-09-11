import React, { useState } from 'react';
import Alert from '@mui/material/Alert';
import AlertTitle from '@mui/material/AlertTitle';
import {
  Grid,
  useTranslation,
  LoadingButton,
  InlineSpinner,
  ConfigKind,
} from '@notify-frontend/common';
import { BaseNotificationEditForm } from './BaseNotificationEditForm';
import { BaseNotificationConfig } from '../../types';

interface BaseNotificationEditPageProps<T extends BaseNotificationConfig> {
  kind: ConfigKind;
  isInvalid: boolean;
  isLoading: boolean;
  draft: T;
  setDraft: (draft: T) => void;
  onSave: (draft: T) => Promise<void>;
  CustomForm: React.FC<{
    onUpdate: (patch: Partial<T>) => void;
    draft: T;
  }>;
}

export const BaseNotificationEditPage = <T extends BaseNotificationConfig>({
  // kind,
  isLoading,
  // isInvalid,
  draft,
  // onSave,
  setDraft,
  CustomForm,
}: BaseNotificationEditPageProps<T>) => {
  const t = useTranslation(['system']);
  const [errorMessage, setErrorMessage] = useState('');

  const onUpdate = (patch: Partial<T>) => {
    setDraft({ ...draft, ...patch });
  };

  // TODO: add a save button somewhere
  // TODO: We'll add a parameters editor in issue https://github.com/openmsupply/notify/issues/116

  return (
    <>
      {isLoading ? (
        <InlineSpinner />
      ) : (
        <Grid flexDirection="column" display="flex" gap={2}>
          <BaseNotificationEditForm
            CustomForm={CustomForm}
            onUpdate={onUpdate}
            draft={draft}
          />
          {errorMessage ? (
            <Grid item>
              <Alert
                severity="error"
                onClose={() => {
                  setErrorMessage('');
                }}
              >
                <AlertTitle>{t('error')}</AlertTitle>
                {errorMessage}
              </Alert>
            </Grid>
          ) : null}
        </Grid>
      )}
    </>
  );
};
