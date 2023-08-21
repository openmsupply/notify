import React, { FC, useState } from 'react';
import { ModalMode, FnUtils } from '@notify-frontend/common';
import {
  CCNotification,
  CCNotificationEditForm,
} from './CCNotificationEditForm';
import { BaseNotificationEditModal } from '../Base/BaseNotificationEditModal';

interface CCNotificationEditModalProps {
  mode: ModalMode | null;
  isOpen: boolean;
  onClose: () => void;
  entity: CCNotification | null;
}

const createCCNotifcation = (seed: CCNotification | null): CCNotification => ({
  id: FnUtils.generateUUID(),
  title: '',
  highTemp: false,
  lowTemp: false,
  confirmOk: false,
  remind: false,
  reminderInterval: 5,
  reminderUnits: 'minutes',
  recipientIds: [],
  recipientListIds: [],
  locationIds: [],
  ...seed,
});

export const CCNotificationEditModal: FC<CCNotificationEditModalProps> = ({
  mode,
  isOpen,
  onClose,
  entity,
}) => {
  const [draft, setDraft] = useState(() => createCCNotifcation(entity));

  const onSave = async (draft: CCNotification) => {
    const {
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
      locationIds,
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
      locationIds,
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
    // no locations selected
    !draft.locationIds.length ||
    // no recipients selected
    (!draft.recipientListIds.length && !draft.recipientIds.length);

  return (
    <BaseNotificationEditModal
      notificationType="Cold Chain"
      isOpen={isOpen}
      isInvalid={isInvalid}
      onClose={onClose}
      onSave={onSave}
      draft={draft}
      setDraft={setDraft}
      CustomForm={CCNotificationEditForm}
    />
  );
};
