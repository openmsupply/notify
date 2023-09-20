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
import { BaseNotificationConfig } from '../../types';
import {
  RecipientListRowFragment,
  RecipientRowFragment,
} from 'packages/system/src/Recipients/api/operations.generated';

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

interface BaseNotificationAppBarProps<T> {
  onUpdate: (patch: Partial<T>) => void;
  recipientLists?: RecipientListRowFragment[];
  recipients?: RecipientRowFragment[];
  draft: BaseNotificationConfig;
}

export const BaseNotificationAppBar = <T extends BaseNotificationConfig>({
  onUpdate,
  recipientLists,
  recipients,
  draft,
}: BaseNotificationAppBarProps<T>) => {
  const t = useTranslation('system');
  const { isOpen, onClose, onOpen } = useEditModal();

  const selectedRecipientLists = (recipientLists ?? []).filter(list =>
    draft.recipientListIds.includes(list.id)
  );
  const selectedRecipients = (recipients ?? []).filter(recipient =>
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
          setSelection={({ recipients: recipientIds, recipientLists }) => {
            onUpdate({
              recipientIds: recipientIds,
              recipientListIds: recipientLists,
            } as Partial<T>);
          }}
          recipientLists={recipientLists ?? []}
          recipients={recipients ?? []}
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
        <StyledButton onClick={() => onOpen()}>
          {selectedNames ? `${selectedNames};` : t('label.select-recipients')}
          <PlusCircleIcon color="primary" />
        </StyledButton>
      </Grid>
    </>
  );
};
