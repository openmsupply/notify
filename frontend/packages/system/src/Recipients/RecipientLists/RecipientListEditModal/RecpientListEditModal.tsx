import React from 'react';
import { ModalMode, useTranslation, FnUtils } from '@notify-frontend/common';
import { RecipientListEditForm } from './RecipientListEditForm';
import { DraftRecipientList } from './types';
import { EditModal } from 'packages/system/src/shared/EditModal';
import { RecipientListRowFragment } from '../../api/operations.generated';
import { useCreateRecipientList, useUpdateRecipientList } from '../../api';

interface RecipientListEditModalProps {
  mode: ModalMode | null;
  isOpen: boolean;
  onClose: () => void;
  recipientList: RecipientListRowFragment | null;
}

const createRecipientList = (
  seed?: DraftRecipientList | null
): DraftRecipientList => ({
  id: FnUtils.generateUUID(),
  name: '',
  description: '',
  ...seed,
});

export const checkIsInvalid = (draft: DraftRecipientList) =>
  !draft.description.trim() || !draft.name.trim();

export const RecipientListEditModal = ({
  mode,
  isOpen,
  onClose,
  recipientList,
}: RecipientListEditModalProps) => {
  const t = useTranslation(['system']);

  const { mutateAsync: create, isLoading: createIsLoading } =
    useCreateRecipientList();
  const { mutateAsync: update, isLoading: updateIsLoading } =
    useUpdateRecipientList();

  const onSave = async (draft: DraftRecipientList) => {
    const { id, name, description } = draft;
    const input = { id, name, description };

    if (mode === ModalMode.Create) await create({ input });
    else await update({ input });
  };

  return (
    <EditModal
      isLoading={createIsLoading || updateIsLoading}
      isOpen={isOpen}
      checkIsInvalid={checkIsInvalid}
      mode={mode}
      logs={recipientList?.auditLogs ?? []}
      title={
        mode === ModalMode.Create
          ? t('label.new-recipient-list')
          : t('label.edit-recipient-list')
      }
      onClose={onClose}
      createDraft={() => createRecipientList(recipientList)}
      onSave={onSave}
      EditForm={RecipientListEditForm}
    />
  );
};
