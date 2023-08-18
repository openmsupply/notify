import React, { PropsWithChildren } from 'react';
import {
  BasicTextInput,
  ButtonProps,
  Grid,
  PlusCircleIcon,
  styled,
  useEditModal,
  useTranslation,
} from '@notify-frontend/common';
import { RecipientsModal } from './RecipientsModal';
import { useRecipientLists, useRecipients } from '../../Recipients/api';

export interface BaseNotificationConfig {
  id: string;
  title: string;
  recipientIds: string[];
  recipientListIds: string[];
}

type BaseNotificationEditFormProps<T extends BaseNotificationConfig> = {
  onUpdate: (patch: Partial<T>) => void;
  draft: T;
  CustomForm: React.FC<{
    onUpdate: (patch: Partial<T>) => void;
    draft: T;
  }>;
};

export const BaseNotificationEditForm = <T extends BaseNotificationConfig>({
  onUpdate,
  draft,
  CustomForm,
}: BaseNotificationEditFormProps<T>) => {
  const t = useTranslation('system');
  const { isOpen, onClose, onOpen } = useEditModal();

  const { data: recipients } = useRecipients();
  const { data: recipientLists } = useRecipientLists();

  const selectedRecipientLists = (recipientLists?.nodes ?? []).filter(list =>
    draft.recipientListIds.includes(list.id)
  );
  const selectedRecipients = (recipients?.nodes ?? []).filter(recipient =>
    draft.recipientIds.includes(recipient.id)
  );
  const selectedNames = [...selectedRecipientLists, ...selectedRecipients]
    .map(r => r.name)
    .join('; ');

  return (
    <>
      {isOpen && (
        <RecipientsModal
          isOpen={isOpen}
          onClose={onClose}
          initialSelectedIds={[
            ...draft.recipientListIds,
            ...draft.recipientIds,
          ]}
          setSelection={({ recipients, recipientLists }) =>
            onUpdate({
              recipientIds: recipients,
              recipientListIds: recipientLists,
            } as Partial<T>)
          }
          recipientLists={recipientLists?.nodes ?? []}
          recipients={recipients?.nodes ?? []}
        />
      )}
      <Grid flexDirection="column" display="flex" gap={2}>
        <BasicTextInput
          autoFocus
          value={draft.title}
          required
          onChange={e => onUpdate({ title: e.target.value } as Partial<T>)}
          label={t('label.notification-title')}
          InputLabelProps={{ shrink: true }}
        />
        <CustomForm draft={draft} onUpdate={onUpdate} />
        <StyledButton onClick={() => onOpen()}>
          {selectedNames || t('label.select-recipients')}
          <PlusCircleIcon color="primary" />
        </StyledButton>
      </Grid>
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
