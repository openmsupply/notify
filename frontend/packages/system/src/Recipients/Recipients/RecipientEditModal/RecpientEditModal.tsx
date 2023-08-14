import React from 'react';
import {
  ModalMode,
  useTranslation,
  FnUtils,
  NotificationTypeNode,
} from '@notify-frontend/common';
import { RecipientRowFragment, useCreateRecipient } from '../../api';
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

export const RecipientEditModal = ({
  mode,
  isOpen,
  onClose,
  recipient,
}: RecipientEditModalProps) => {
  const t = useTranslation(['system']);

  const { mutateAsync: create, isLoading: createIsLoading } =
    useCreateRecipient();
  // const { mutateAsync: update, isLoading: updateIsLoading } =
  //   useUserAccount.document.update();

  const onSave = async (draft: DraftRecipient) => {
    if (mode === ModalMode.Create) {
      await create({ input: draft });
    } else {
      //   await update();
    }
  };

  const checkIsInvalid = (draft: DraftRecipient) =>
    !draft.toAddress || !draft.name;

  return (
    <EditModal
      isLoading={createIsLoading}
      isOpen={isOpen}
      checkIsInvalid={checkIsInvalid}
      mode={mode}
      logs={recipient?.auditLogs ?? []}
      title={
        mode === ModalMode.Create
          ? t('label.new-recipient', { ns: 'system' })
          : t('label.edit-recipient', { ns: 'system' })
      }
      onClose={onClose}
      createDraft={createRecipient}
      onSave={onSave}
      EditForm={RecipientEditForm}
    />
  );
};
