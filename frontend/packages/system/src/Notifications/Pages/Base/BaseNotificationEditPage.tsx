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
  ConfigStatus,
  FormLabel,
  Switch,
  useDetailPanel,
  AppBarButtonsPortal,
  KeyedParams,
  TeraUtils,
} from '@notify-frontend/common';

import { BaseNotificationConfig } from '../../types';
import { BaseNotificationAppBar } from './BaseNotificationAppBar';
import { NotificationDetailPanel } from './NotificationDetailPanel';
import {
  useRecipientLists,
  useRecipients,
  useSqlRecipientLists,
} from 'packages/system/src/Recipients/api';

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
  const { OpenButton } = useDetailPanel(t('label.parameters'));
  const { navigateUpOne } = useBreadcrumbs();
  const [errorMessage, setErrorMessage] = useState('');
  const [isSaved, setIsSaved] = useState(false);

  const { data: recipients } = useRecipients();
  const { data: recipientLists } = useRecipientLists();
  const { data: sqlRecipientLists } = useSqlRecipientLists();

  const onUpdate = (patch: Partial<T>) => {
    setDraft({ ...draft, ...patch });
    setIsSaved(false);
  };

  const isEnabled = (status: ConfigStatus) => {
    return status == ConfigStatus.Enabled;
  };

  const onUpdateParams = (key: string, value: string) => {
    const updatedParam = { [key]: value } as KeyedParams;
    const parseParams = { ...draft.parsedParameters, ...updatedParam };
    onUpdate({
      ...draft,
      parsedParameters: parseParams,
      parameters: TeraUtils.keyedParamsAsTeraJson(parseParams),
    });
  };

  const onDeleteParam = (key: string) => {
    const updatedParams = draft.parsedParameters;
    delete updatedParams[key];
    onUpdate({
      ...draft,
      parsedParameters: updatedParams,
      parameters: TeraUtils.keyedParamsAsTeraJson(updatedParams),
    });
  };

  const requiredParams = (sqlRecipientLists?.nodes ?? [])
    .filter(list => draft.sqlRecipientListIds.includes(list.id))
    .map(list => list.parameters)
    .flat(1);

  const allParamsSet = requiredParams.every(param => {
    if (param) {
      return draft.parsedParameters[param] !== undefined; // This allows the user to set the param to an empty string if they edit the field then delete the value
    } else {
      return false;
    }
  });

  return (
    <>
      {isLoading ? (
        <InlineSpinner />
      ) : (
        <>
          <NotificationDetailPanel
            requiredParams={requiredParams}
            params={draft.parsedParameters}
            onUpdateParams={onUpdateParams}
            onDeleteParam={onDeleteParam}
          />
          <AppBarButtonsPortal>{OpenButton}</AppBarButtonsPortal>
          <AppBarContentPortal sx={{ paddingBottom: '16px', flex: 1 }}>
            <BaseNotificationAppBar
              draft={draft}
              onUpdate={onUpdate}
              recipientLists={recipientLists?.nodes ?? []}
              recipients={recipients?.nodes ?? []}
              sqlRecipientLists={sqlRecipientLists?.nodes ?? []}
            />
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
                <Box
                  flex={1}
                  display="flex"
                  justifyContent="flex-start"
                  gap={1}
                >
                  <FormLabel
                    sx={{
                      alignSelf: 'flex-start',
                      alignItems: 'center',
                      paddingTop: 1,
                    }}
                  >
                    {t('label.enable')}
                  </FormLabel>
                  <Switch
                    checked={isEnabled(draft.status)}
                    onChange={() => {
                      if (isEnabled(draft.status)) {
                        onUpdate({
                          status: ConfigStatus.Disabled,
                        } as Partial<T>);
                      } else {
                        onUpdate({
                          status: ConfigStatus.Enabled,
                        } as Partial<T>);
                      }
                    }}
                  />
                </Box>
                <Box flex={1} display="flex" justifyContent="flex-end" gap={2}>
                  <ButtonWithIcon
                    shrinkThreshold="lg"
                    Icon={<CloseIcon />}
                    label={t('button.close')}
                    color="secondary"
                    sx={{ fontSize: '12px' }}
                    onClick={navigateUpOne}
                  />

                  <LoadingButton
                    disabled={isSaved || isInvalid || !allParamsSet}
                    isLoading={isLoading}
                    onClick={() => {
                      onSave(draft);
                      setIsSaved(true);
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
