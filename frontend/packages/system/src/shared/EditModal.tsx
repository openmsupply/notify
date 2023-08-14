import React, { useState } from 'react';
import Alert from '@mui/material/Alert';
import AlertTitle from '@mui/material/AlertTitle';
import {
  ModalMode,
  useDialog,
  Grid,
  DialogButton,
  useTranslation,
  LoadingButton,
  CheckIcon,
  InlineSpinner,
  ModalTabs,
  Box,
  RecordWithId,
} from '@notify-frontend/common';
import { LogList, LogRowFragment } from 'packages/system/src/Log';

interface EditModalProps<T extends RecordWithId> {
  mode: ModalMode | null;
  title: string;
  isOpen: boolean;
  logs: LogRowFragment[];
  isLoading: boolean;
  checkIsInvalid: (draft: T) => boolean;
  onClose: () => void;
  createDraft: () => T;
  onSave: (item: T) => Promise<unknown>;
  EditForm: React.FC<{ draft: T; onUpdate: (patch: Partial<T>) => void }>;
}

export const EditModal = <T extends RecordWithId>({
  mode,
  title,
  isOpen,
  isLoading,
  logs,
  checkIsInvalid,
  onClose,
  createDraft,
  onSave,
  EditForm,
}: EditModalProps<T>) => {
  const t = useTranslation(['system']);
  const [errorMessage, setErrorMessage] = useState('');
  const { Modal } = useDialog({ isOpen, onClose });

  const [draft, setDraft] = useState(() => createDraft());

  const onUpdate = (patch: Partial<T>) => {
    setDraft({ ...draft, ...patch });
  };

  const tabs = [
    {
      Component: <EditForm draft={draft} onUpdate={onUpdate} />,
      value: t('label.details'),
    },
    ...(mode === ModalMode.Update
      ? [
          {
            Component: (
              <Box sx={{ height: '450px' }}>
                <LogList records={logs ?? []} />
              </Box>
            ),
            value: t('label.log'),
          },
        ]
      : []),
  ];

  const isInvalid = checkIsInvalid(draft);

  return (
    <Modal
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
      title={title}
    >
      {isLoading ? (
        <InlineSpinner />
      ) : (
        <Grid flexDirection="column" display="flex" gap={2} width={500}>
          <ModalTabs tabs={tabs} />
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
