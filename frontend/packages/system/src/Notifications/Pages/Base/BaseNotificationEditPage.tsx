import React, { useState } from 'react';
import Alert from '@mui/material/Alert';
import AlertTitle from '@mui/material/AlertTitle';
import {
  Grid,
  useTranslation,
  LoadingButton,
  InlineSpinner,
  AppBarContentPortal,
  Box,
  AppFooterPortal,
  ButtonWithIcon,
  CloseIcon,
  useBreadcrumbs,
  SaveIcon,
} from '@notify-frontend/common';

import { BaseNotificationConfig } from '../../types';
import { BaseNotificationAppBar } from './BaseNotificationAppBar';

interface BaseNotificationEditPageProps<T extends BaseNotificationConfig> {
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
  isLoading,
  isInvalid,
  draft,
  onSave,
  setDraft,
  CustomForm,
}: BaseNotificationEditPageProps<T>) => {
  const t = useTranslation(['system']);
  const { navigateUpOne } = useBreadcrumbs();
  const [errorMessage, setErrorMessage] = useState('');
  const [isSaved, setIsSaved] = useState(false);

  const onUpdate = (patch: Partial<T>) => {
    setDraft({ ...draft, ...patch });
    setIsSaved(false);
  };

  const onSaved = () => {
    setIsSaved(true);
  };

  // TODO: We'll add a parameters editor in issue https://github.com/openmsupply/notify/issues/116

  return (
    <>
      {isLoading ? (
        <InlineSpinner />
      ) : (
        <>
          <AppBarContentPortal sx={{ paddingBottom: '16px', flex: 1 }}>
            <BaseNotificationAppBar draft={draft} onUpdate={onUpdate} />
          </AppBarContentPortal>
          <Grid flexDirection="column" display="flex" gap={2}>
            <Box sx={{ paddingLeft: '10px' }}>
              <CustomForm draft={draft} onUpdate={onUpdate} />
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
            </Box>
          </Grid>
          <AppFooterPortal
            Content={
              <Box
                gap={2}
                display="flex"
                flexDirection="row"
                alignItems="center"
                height={64}
              >
                <Box flex={1} display="flex" justifyContent="flex-end" gap={2}>
                  <ButtonWithIcon
                    shrinkThreshold="lg"
                    Icon={<CloseIcon />}
                    label={t('button.close')}
                    color="secondary"
                    sx={{ fontSize: '12px' }}
                    onClick={() => navigateUpOne()}
                  />
                  <LoadingButton
                    disabled={isSaved || isInvalid}
                    isLoading={isLoading}
                    onClick={() => {
                      onSave(draft);
                      onSaved();
                    }}
                    startIcon={<SaveIcon />}
                    sx={{ fontSize: '12px' }}
                  >
                    {t('button.save')}
                  </LoadingButton>
                </Box>
              </Box>
            }
          />
        </>
      )}
    </>
  );
};
