import React, { FC, useEffect, useState } from 'react';
import {
  ModalMode,
  FnUtils,
  ConfigKind,
  useTranslation,
  useNotification,
} from '@notify-frontend/common';
import { ScheduledNotificationEditForm } from './ScheduledNotificationEditForm';
import { BaseNotificationEditModal } from '../Base/BaseNotificationEditModal';
import { ScheduledNotification } from '../../types';
import { useCreateNotificationConfig } from '../../api/hooks/useCreateNotificationConfig';
import {
  buildScheduledNotificationInputs,
  parseScheduledNotificationConfig,
} from './parseConfig';
import { useUpdateNotificationConfig } from '../../api/hooks/useUpdateNotificationConfig';
import { NotificationConfigRowFragment } from '../../api';

interface ScheduledNotificationEditModalProps {
  mode: ModalMode | null;
  isOpen: boolean;
  onClose: () => void;
  entity: NotificationConfigRowFragment | null;
}

const createScheduledNotification = (
  seed: ScheduledNotification | null
): ScheduledNotification => ({
  id: seed?.id ?? FnUtils.generateUUID(),
  title: seed?.title ?? '',
  kind: seed?.kind ?? ConfigKind.Scheduled,
  recipientListIds: seed?.recipientListIds ?? [],
  recipientIds: seed?.recipientIds ?? [],
  parameters: seed?.parameters ?? '[]',
  scheduleFrequency: seed?.scheduleFrequency ?? 'daily',
  scheduleStartTime: seed?.scheduleStartTime ?? new Date(),
  subjectTemplate: seed?.subjectTemplate ?? '',
  bodyTemplate: seed?.bodyTemplate ?? '',
  sqlQueries: seed?.sqlQueries ?? [],
});

export const ScheduledNotificationEditModal: FC<
  ScheduledNotificationEditModalProps
> = ({ mode, isOpen, onClose, entity }) => {
  const t = useTranslation('system');
  const { error } = useNotification();
  const parsingErrorSnack = error(t('error.parsing-notification-config'));

  const [draft, setDraft] = useState<ScheduledNotification>(() =>
    createScheduledNotification(null)
  );

  useEffect(() => {
    const parsedDraft = parseScheduledNotificationConfig(
      entity,
      parsingErrorSnack
    );
    setDraft(createScheduledNotification(parsedDraft));
  }, []);

  const { mutateAsync: create, isLoading: createIsLoading } =
    useCreateNotificationConfig();

  const { mutateAsync: update, isLoading: updateIsLoading } =
    useUpdateNotificationConfig();

  const onSave = async (draft: ScheduledNotification) => {
    const inputs = buildScheduledNotificationInputs(draft);

    if (mode === ModalMode.Create) {
      await create({ input: inputs.create });
    } else {
      await update({ input: inputs.update });
    }
  };

  const isInvalid =
    !draft.title ||
    // nothing selected
    // no recipients selected
    (!draft.recipientListIds.length && !draft.recipientIds.length);

  return (
    <BaseNotificationEditModal
      kind={ConfigKind.Scheduled}
      isOpen={isOpen}
      isLoading={createIsLoading || updateIsLoading}
      isInvalid={isInvalid}
      onClose={onClose}
      onSave={onSave}
      draft={draft}
      setDraft={setDraft}
      CustomForm={ScheduledNotificationEditForm}
    />
  );
};
