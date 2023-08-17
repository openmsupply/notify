import React, { FC, useState } from 'react';
import Alert from '@mui/material/Alert';
import AlertTitle from '@mui/material/AlertTitle';
import {
  ModalMode,
  useDialog,
  Grid,
  DialogButton,
  useTranslation,
  FnUtils,
  LoadingButton,
  CheckIcon,
  InlineSpinner,
} from '@notify-frontend/common';
import { CCNotificationEditForm } from './NotificationEditForm';

interface CCNotificationEditModalProps {
  mode: ModalMode | null;
  isOpen: boolean;
  onClose: () => void;
}

const createCCNotifcation = () => ({
  id: FnUtils.generateUUID(),
  name: '',
});

export const useDraft = (mode: ModalMode | null) => {
  const [draft, setDraft] = useState(() => createCCNotifcation());

  const onUpdate = (patch: Partial<{ name: string }>) => {
    setDraft({ ...draft, ...patch });
  };

  const onSave = async () => {
    console.log(draft);
    if (mode === ModalMode.Create) {
      //   await insert();
      // } else {
      //   await update();
    }
  };

  return {
    draft,
    onUpdate,
    onSave,
    isLoading: false,
  };
};

export const CCNotificationEditModal: FC<CCNotificationEditModalProps> = ({
  mode,
  isOpen,
  onClose,
}) => {
  const [errorMessage, setErrorMessage] = useState('');
  const { Modal } = useDialog({ isOpen, onClose });
  const t = useTranslation(['system']);
  const { draft, onUpdate, onSave, isLoading } = useDraft(mode);

  const isInvalid = !draft.name;

  const modalHeight = Math.min(window.innerHeight - 50, 800);
  const modalWidth = Math.min(window.innerWidth - 50, 1024);

  return (
    <Modal
      height={modalHeight}
      width={modalWidth}
      okButton={
        <LoadingButton
          disabled={isInvalid}
          onClick={() => {
            onSave().then(onClose, err => {
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
      title={
        'Setup Cold Chain Notification'
        // t('label.new-notification')
        // mode === ModalMode.Create
        //   ? t('label.create-user')
        //   : t('label.edit-user')
      }
    >
      {isLoading ? (
        <InlineSpinner />
      ) : (
        <Grid flexDirection="column" display="flex" gap={2}>
          <CCNotificationEditForm onUpdate={onUpdate} />
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
  );
};
