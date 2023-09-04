import React from 'react';
import { ModalMode, useTranslation, FnUtils } from '@notify-frontend/common';
import { SqlRecipientListEditForm } from './SqlRecipientListEditForm';
import { DraftSqlRecipientList } from './types';
import { EditModal } from 'packages/system/src/shared/EditModal';
import { SqlRecipientListRowFragment } from '../../api/operations.generated';
import { useCreateSqlRecipientList, useUpdateSqlRecipientList } from '../../api';

interface SqlRecipientListEditModalProps {
  mode: ModalMode | null;
  isOpen: boolean;
  onClose: () => void;
  recipientList: SqlRecipientListRowFragment | null;
}

const createSqlRecipientList = (
  seed?: DraftSqlRecipientList | null
): DraftSqlRecipientList => ({
  id: FnUtils.generateUUID(),
  name: '',
  description: '',
  sqlQuery: '',
  ...seed,
});

export const checkIsInvalid = (draft: DraftSqlRecipientList) => {
  const nameIncorrectLength = draft.name.length < 3 || draft.name.length > 75;
  const nameContainsIllegalChars = draft.name.match(/[^ 0-9A-Za-z_\-@.+:/()]/);

  return (
    !draft.name.trim() || !!nameContainsIllegalChars || nameIncorrectLength
  );
};

export const SqlRecipientListEditModal = ({
  mode,
  isOpen,
  onClose,
  recipientList,
}: SqlRecipientListEditModalProps) => {
  const t = useTranslation(['system']);

  const { mutateAsync: create, isLoading: createIsLoading } =
    useCreateSqlRecipientList();
  const { mutateAsync: update, isLoading: updateIsLoading } =
    useUpdateSqlRecipientList();

  const onSave = async (draft: DraftSqlRecipientList) => {
    const { id, name, description, sqlQuery } = draft;
    const input = { id, name, description, sqlQuery };

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
      createDraft={() => createSqlRecipientList(recipientList)}
      onSave={onSave}
      EditForm={SqlRecipientListEditForm}
    />
  );
};
