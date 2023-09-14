import React, { FC, useEffect, useState } from 'react';
import {
  ModalMode,
  FnUtils,
  ConfigKind,
  useTranslation,
  useNotification,
  ConfigStatus,
} from '@notify-frontend/common';
import { CCNotificationEditForm } from './CCNotificationEditForm';
import { BaseNotificationEditModal } from '../Base/BaseNotificationEditModal';
import { CCNotification } from '../../types';
import { useCreateNotificationConfig } from '../../api/hooks/useCreateNotificationConfig';
import {
  buildColdChainNotificationInputs,
  parseColdChainNotificationConfig,
} from './parseConfig';
import { useUpdateNotificationConfig } from '../../api/hooks/useUpdateNotificationConfig';
import { NotificationConfigRowFragment } from '../../api';

interface CCNotificationEditModalProps {
  mode: ModalMode | null;
  isOpen: boolean;
  onClose: () => void;
  entity: NotificationConfigRowFragment | null;
}

const createCCNotifcation = (seed: CCNotification | null): CCNotification => ({
  id: seed?.id ?? FnUtils.generateUUID(),
  title: seed?.title ?? '',
  kind: seed?.kind ?? ConfigKind.ColdChain,
  highTemp: seed?.highTemp ?? false,
  lowTemp: seed?.lowTemp ?? false,
  confirmOk: seed?.confirmOk ?? false,
  remind: seed?.remind ?? false,
  reminderInterval: seed?.reminderInterval ?? 5,
  reminderUnits: seed?.reminderUnits ?? 'minutes',
  locationIds: seed?.locationIds ?? [],
  recipientIds: seed?.recipientIds ?? [],
  recipientListIds: seed?.recipientListIds ?? [],
  status: seed?.status ?? ConfigStatus.Disabled,
});

export const CCNotificationEditModal: FC<CCNotificationEditModalProps> = ({
  mode,
  isOpen,
  onClose,
  entity,
}) => {
  const t = useTranslation('system');
  const { error } = useNotification();
  const parsingErrorSnack = error(t('error.parsing-notification-config'));

  const [draft, setDraft] = useState<CCNotification>(() =>
    createCCNotifcation(null)
  );

  useEffect(() => {
    const parsedDraft = parseColdChainNotificationConfig(
      entity,
      parsingErrorSnack
    );
    setDraft(createCCNotifcation(parsedDraft));
  }, []);

  const { mutateAsync: create, isLoading: createIsLoading } =
    useCreateNotificationConfig();

  const { mutateAsync: update, isLoading: updateIsLoading } =
    useUpdateNotificationConfig();

  const onSave = async (draft: CCNotification) => {
    const inputs = buildColdChainNotificationInputs(draft);

    if (mode === ModalMode.Create) {
      await create({ input: inputs.create });
    } else {
      await update({ input: inputs.update });
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
      status={ConfigStatus.Disabled}
      CustomForm={CCNotificationEditForm}
    />
  );
};
