import React, { FC } from 'react';
import { ModalMode, FnUtils } from '@notify-frontend/common';
import {
  CCNotification,
  CCNotificationEditForm,
} from './CCNotificationEditForm';
import { BaseNotificationEditModal } from '../Base/NotificationEditModal';

interface CCNotificationEditModalProps {
  mode: ModalMode | null;
  isOpen: boolean;
  onClose: () => void;
}

const createCCNotifcation = (): CCNotification => ({
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
});

export const CCNotificationEditModal: FC<CCNotificationEditModalProps> = ({
  mode,
  isOpen,
  onClose,
}) => {
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

  const isInvalid = (draft: CCNotification) =>
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
      createDraft={createCCNotifcation}
      CustomForm={CCNotificationEditForm}
    />
  );
};
