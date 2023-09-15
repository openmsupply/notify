import React, { useState } from 'react';
import Alert from '@mui/material/Alert';
import AlertTitle from '@mui/material/AlertTitle';
import {
  useDialog,
  Grid,
  DialogButton,
  useTranslation,
  LoadingButton,
  CheckIcon,
  InlineSpinner,
  ConfigKind,
  ConfigStatus,
  Checkbox,
} from '@notify-frontend/common';
import { BaseNotificationEditForm } from './BaseNotificationEditForm';
import { BaseNotificationConfig } from '../../types';

interface BaseNotificationEditModalProps<T extends BaseNotificationConfig> {
  isOpen: boolean;
  kind: ConfigKind;
  isInvalid: boolean;
  isLoading: boolean;
  draft: T;
  setDraft: (draft: T) => void;
  onClose: () => void;
  onSave: (draft: T) => Promise<void>;
  CustomForm: React.FC<{
    onUpdate: (patch: Partial<T>) => void;
    draft: T;
  }>;
  status: ConfigStatus;
}

export const BaseNotificationEditModal = <T extends BaseNotificationConfig>({
  isOpen,
  kind,
  isLoading,
  isInvalid,
  draft,
  onClose,
  onSave,
  setDraft,
  CustomForm,
  status,
}: BaseNotificationEditModalProps<T>) => {
  const t = useTranslation(['system']);

  const [errorMessage, setErrorMessage] = useState('');

  const { Modal } = useDialog({ isOpen, onClose });

  const onUpdate = (patch: Partial<T>) => {
    setDraft({ ...draft, ...patch });
  };

  const modalHeight = Math.min(window.innerHeight - 50, 800);
  const modalWidth = Math.min(window.innerWidth - 50, 1024);
  const isEnabled = (status: ConfigStatus) => {
    return  (status == ConfigStatus.Enabled);
  }

  return (
    <>
      <Modal
        height={modalHeight}
        width={modalWidth}
        okButton={
          <LoadingButton
            disabled={isInvalid}
            onClick={() => {
              onSave(draft).then(onClose, err => {
                if (!err || !err.message) {
                  err = { message: t('messages.unknown-error') };
                }
                setErrorMessage(err.message);
              });
            }}
            isLoading={isLoading}
            startIcon={<CheckIcon />}
            variant="contained"
          >
            {t('button.ok')}
          </LoadingButton>
        }
        cancelButton={<DialogButton variant="cancel" onClick={onClose} />}
        statusCheckbox={
        <ul style={{ listStyleType: 'none', paddingLeft: '30' }}>
          <li>
            <Checkbox checked={isEnabled(draft.status)}
              onClick={() => {
                if(isEnabled(draft.status)){
                onUpdate({
                    status: ConfigStatus.Disabled,
                  } as Partial<T>)
                }else{
                  onUpdate({
                    status: ConfigStatus.Enabled,
                  } as Partial<T>)
                }
              }}
            />
            <label style={{ display: 'inline-block', lineHeight: 1.3 }}>
              {t('label.enable')}
            </label>

          </li>
        </ul>
        }
        title={t('label.setup-notification', {
          type: t(`config-kind.${kind}`),
        })}
      >
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
      </Modal>
    </>
  );
};
