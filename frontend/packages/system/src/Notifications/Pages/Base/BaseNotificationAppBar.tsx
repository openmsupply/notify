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
import {
  useRecipientLists,
  useRecipients,
  useSqlRecipientLists,
} from '../../../Recipients/api';
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
  onChangeSqlParams: (requiredParams: string[]) => void;
  draft: BaseNotificationConfig;
}

export const BaseNotificationAppBar = <T extends BaseNotificationConfig>({
  onUpdate,
  onChangeSqlParams: onChangeSqlParams,
  draft,
}: BaseNotificationAppBarProps<T>) => {
  const t = useTranslation('system');
  const { isOpen, onClose, onOpen } = useEditModal();

  const { data: recipients } = useRecipients();
  const { data: recipientLists } = useRecipientLists();
  const { data: sqlRecipientLists } = useSqlRecipientLists();

  const selectedSqlRecipientLists = (sqlRecipientLists?.nodes ?? []).filter(
    list => draft.sqlRecipientListIds.includes(list.id)
  );

  const selectedRecipientLists = (recipientLists?.nodes ?? []).filter(list =>
    draft.recipientListIds.includes(list.id)
  );
  const selectedRecipients = (recipients?.nodes ?? []).filter(recipient =>
    draft.recipientIds.includes(recipient.id)
  );
  const selectedNames = [
    ...selectedSqlRecipientLists,
    ...selectedRecipientLists,
    ...selectedRecipients,
  ]
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
            ...draft.sqlRecipientListIds,
          ]}
          setSelection={({
            recipients: recipientIds,
            recipientLists,
            sqlRecipientLists: sqlRecipientListIds,
          }) => {
            onUpdate({
              recipientIds: recipientIds,
              recipientListIds: recipientLists,
              sqlRecipientListIds: sqlRecipientListIds,
            } as Partial<T>);
            const selectedSqlRecipientLists = (
              sqlRecipientLists?.nodes ?? []
            ).filter(list => sqlRecipientListIds.includes(list.id));
            const requiredParams = [
              ...new Set(
                selectedSqlRecipientLists.map(list => list.parameters).flat(1)
              ),
            ];
            onChangeSqlParams(requiredParams);
          }}
          recipientLists={recipientLists?.nodes ?? []}
          recipients={recipients?.nodes ?? []}
          sqlRecipientLists={sqlRecipientLists?.nodes ?? []}
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
