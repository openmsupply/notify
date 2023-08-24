import React, { FC, useState } from 'react';
import { ModalMode, FnUtils, ConfigKind } from '@notify-frontend/common';
import { CCNotificationEditForm } from './CCNotificationEditForm';
import { BaseNotificationEditModal } from '../Base/BaseNotificationEditModal';
import { CCNotification } from '../../types';
import { useCreateNotificationConfig } from '../../api/hooks/useCreateNotificationConfig';
import { buildCreateInput, buildUpdateInput } from './parseConfig';
import { useUpdateNotificationConfig } from '../../api/hooks/useUpdateNotificationConfig';

interface CCNotificationEditModalProps {
  mode: ModalMode | null;
  isOpen: boolean;
  onClose: () => void;
  entity: CCNotification | null;
}

const createCCNotifcation = (seed: CCNotification | null): CCNotification => ({
  id: FnUtils.generateUUID(),
  title: '',
  kind: ConfigKind.ColdChain,
  highTemp: false,
  lowTemp: false,
  confirmOk: false,
  remind: false,
  reminderInterval: 5,
  reminderUnits: 'minutes',
  locationIds: [],
  recipientIds: [],
  recipientListIds: [],
  ...seed,
});

export const CCNotificationEditModal: FC<CCNotificationEditModalProps> = ({
  mode,
  isOpen,
  onClose,
  entity,
}) => {
  const [draft, setDraft] = useState(() => createCCNotifcation(entity));

  const { mutateAsync: create, isLoading: createIsLoading } =
    useCreateNotificationConfig();

  const { mutateAsync: update, isLoading: updateIsLoading } =
    useUpdateNotificationConfig();

  const onSave = async (draft: CCNotification) => {
    if (mode === ModalMode.Create) {
      await create({ input: buildCreateInput(draft) });
    } else {
      await update({ input: buildUpdateInput(draft) });
    }
  };

  const isInvalid =
    !draft.title ||
    // nothing selected
    (!draft.confirmOk && !draft.highTemp && !draft.lowTemp && draft.remind) ||
    // no locations selected
    !draft.locationIds.length ||
    // no recipients selected
    (!draft.recipientListIds.length && !draft.recipientIds.length);

  return (
    <BaseNotificationEditModal
      kind={ConfigKind.ColdChain}
      isOpen={isOpen}
      isLoading={createIsLoading || updateIsLoading}
      isInvalid={isInvalid}
      onClose={onClose}
      onSave={onSave}
      draft={draft}
      setDraft={setDraft}
      CustomForm={CCNotificationEditForm}
    />
  );
};
