import React, { PropsWithChildren } from 'react';
import {
  BasicTextInput,
  ButtonProps,
  FormLabel,
  Grid,
  PlusCircleIcon,
  styled,
  useEditModal,
  useTranslation,
} from '@notify-frontend/common';
import { RecipientsModal } from './RecipientsModal';
import { useRecipientLists, useRecipients } from '../../../Recipients/api';
import { BaseNotificationConfig } from '../../types';

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
  draft: BaseNotificationConfig;
}

export const BaseNotificationAppBar = <T extends BaseNotificationConfig>({
  onUpdate,
  draft,
}: BaseNotificationAppBarProps<T>) => {
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
      <Grid
        flexDirection="column"
        display="flex"
        paddingTop={2}
        paddingBottom={2}
      >
        <BasicTextInput
          autoFocus
          value={draft.title}
          required
          onChange={e => onUpdate({ title: e.target.value } as Partial<T>)}
          label={t('label.notification-title')}
          InputLabelProps={{ shrink: true }}
        />
        <FormLabel
          sx={{
            alignSelf: 'flex-start',
            paddingTop: 2,
            transform: 'translate(-10px, -1.5px) scale(0.75)',
          }}
        >
          {t('recipients', { ns: 'host' })}
        </FormLabel>
        <StyledButton onClick={() => onOpen()}>
          {selectedNames ? `${selectedNames};` : t('label.select-recipients')}
          <PlusCircleIcon color="primary" />
        </StyledButton>
      </Grid>
    </>
  );
};
