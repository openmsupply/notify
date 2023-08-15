import React from 'react';
import {
  ModalMode,
  useTranslation,
  FnUtils,
  NotificationTypeNode,
  RegexUtils,
} from '@notify-frontend/common';
import {
  RecipientRowFragment,
  useCreateRecipient,
  useUpdateRecipient,
} from '../../api';
import { RecipientEditForm } from './RecipientEditForm';
import { DraftRecipient } from './types';
import { EditModal } from 'packages/system/src/shared/EditModal';

interface RecipientEditModalProps {
  mode: ModalMode | null;
  isOpen: boolean;
  onClose: () => void;
  recipient: RecipientRowFragment | null;
}

const createRecipient = (seed?: DraftRecipient | null): DraftRecipient => ({
  id: FnUtils.generateUUID(),
  name: '',
  toAddress: '',
  notificationType: NotificationTypeNode.Email,
  ...seed,
});

export const checkIsInvalid = (draft: DraftRecipient, mode: ModalMode | null) =>
  !draft.toAddress.trim() ||
  (draft.notificationType === NotificationTypeNode.Email &&
    !RegexUtils.isValidEmail(draft.toAddress)) ||
  !draft.name.trim() ||
  (mode === ModalMode.Create &&
    draft.notificationType !== NotificationTypeNode.Email);

export const RecipientEditModal = ({
  mode,
  isOpen,
  onClose,
  recipient,
}: RecipientEditModalProps) => {
  const t = useTranslation(['system']);

  const { mutateAsync: create, isLoading: createIsLoading } =
    useCreateRecipient();
  const { mutateAsync: update, isLoading: updateIsLoading } =
    useUpdateRecipient();

  const onSave = async (draft: DraftRecipient) => {
    if (mode === ModalMode.Create) {
      const { id, name, toAddress, notificationType } = draft;
      await create({ input: { id, name, toAddress, notificationType } });
    } else {
      const { id, name, toAddress } = draft;
      await update({ input: { id, name, toAddress } });
    }
  };

  return (
    <EditModal
      isLoading={createIsLoading || updateIsLoading}
      isOpen={isOpen}
      checkIsInvalid={checkIsInvalid}
      mode={mode}
      logs={recipient?.auditLogs ?? []}
      title={
        mode === ModalMode.Create
          ? t('label.new-recipient')
          : t('label.edit-recipient')
      }
      onClose={onClose}
      createDraft={() => createRecipient(recipient)}
      onSave={onSave}
      EditForm={RecipientEditForm}
    />
  );
};
