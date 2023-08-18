import React, { FC, PropsWithChildren, useState } from 'react';
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
  useEditModal,
  PlusCircleIcon,
  styled,
  ButtonProps,
} from '@notify-frontend/common';
import { CCNotification, CCNotificationEditForm } from './NotificationEditForm';
import { RecipientsModal } from './RecipientsModal';
import { useRecipientLists, useRecipients } from '../../Recipients/api';

interface CCNotificationEditModalProps {
  mode: ModalMode | null;
  isOpen: boolean;
  onClose: () => void;
}

const createCCNotifcation = (): CCNotification => ({
  id: FnUtils.generateUUID(),
  title: '',
  highTemp: false,
  lowTemp: false,
  confirmOk: false,
  remind: false,
  reminderInterval: 5,
  reminderUnits: 'minutes',
});

export const CCNotificationEditModal: FC<CCNotificationEditModalProps> = ({
  mode,
  isOpen,
  onClose,
}) => {
  const t = useTranslation(['system']);

  const [errorMessage, setErrorMessage] = useState('');
  const [draft, setDraft] = useState(() => createCCNotifcation());
  const [recipientIds, setRecipientIds] = useState<string[]>([]);
  const [recipientListIds, setRecipientListIds] = useState<string[]>([]);

  const { Modal } = useDialog({ isOpen, onClose });
  const {
    isOpen: recipientsIsOpen,
    onClose: recipientsOnClose,
    onOpen: onRecipientsOpen,
  } = useEditModal();

  const setSelection = (input: {
    recipients: string[];
    recipientLists: string[];
  }) => {
    setRecipientIds(input.recipients);
    setRecipientListIds(input.recipientLists);
  };

  const onUpdate = (patch: Partial<CCNotification>) => {
    setDraft({ ...draft, ...patch });
  };

  const onSave = async () => {
    const {
      id,
      title,
      highTemp,
      lowTemp,
      confirmOk,
      remind,
      reminderInterval,
      reminderUnits,
    } = draft;
    const input = {
      id,
      title,
      highTemp,
      lowTemp,
      confirmOk,
      remind,
      reminderInterval,
      reminderUnits,
      recipientIds,
      recipientListIds,
    };
    console.log(input);
    if (mode === ModalMode.Create) {
      //   await insert(input);
      // } else {
      //   await update(input);
    }
  };

  const isInvalid =
    !draft.title ||
    // nothing selected
    (!draft.confirmOk && !draft.highTemp && !draft.lowTemp && draft.remind) ||
    // no recipients selected
    (!recipientListIds.length && !recipientIds.length);

  const { data: recipients } = useRecipients();
  const { data: recipientLists } = useRecipientLists();

  const selectedRecipientLists = (recipientLists?.nodes ?? []).filter(list =>
    recipientListIds.includes(list.id)
  );
  const selectedRecipients = (recipients?.nodes ?? []).filter(recipient =>
    recipientIds.includes(recipient.id)
  );
  const selectedNames = [...selectedRecipientLists, ...selectedRecipients]
    .map(r => r.name)
    .join('; ');

  const isLoading = false;

  const modalHeight = Math.min(window.innerHeight - 50, 800);
  const modalWidth = Math.min(window.innerWidth - 50, 1024);

  return (
    <>
      {isOpen && (
        <RecipientsModal
          isOpen={recipientsIsOpen}
          onClose={recipientsOnClose}
          initialSelectedIds={[...recipientListIds, ...recipientIds]}
          setSelection={setSelection}
          recipientLists={recipientLists?.nodes ?? []}
          recipients={recipients?.nodes ?? []}
        />
      )}
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
        title={t('label.setup-notification', { type: 'Cold Chain' })}
      >
        {isLoading ? (
          <InlineSpinner />
        ) : (
          <Grid flexDirection="column" display="flex" gap={2}>
            <CCNotificationEditForm onUpdate={onUpdate} draft={draft} />
            <StyledButton onClick={() => onRecipientsOpen()}>
              {selectedNames || t('label.select-recipients')}
              <PlusCircleIcon color="primary" />
            </StyledButton>
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

const Button = ({ children, ...props }: PropsWithChildren<ButtonProps>) => (
  <button {...props}>{children}</button>
);
const StyledButton = styled(Button)(({ theme }) => {
  return {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    borderRadius: '8px',
    backgroundColor: 'white',
    border: '1px',
    borderStyle: 'solid',
    borderColor: theme.palette.border,
    padding: '7px 10px',
    color: theme.palette.gray.main,
    cursor: 'pointer',
    fontSize: '14px',
    textAlign: 'left',
    gap: '10px',
    lineHeight: 1.5,
  };
});
