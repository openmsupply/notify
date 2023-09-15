import React, { useEffect, useState } from 'react';
import {
  FnUtils,
  ConfigKind,
  useTranslation,
  useNotification,
  useParams,
} from '@notify-frontend/common';
import { ScheduledNotificationEditForm } from './ScheduledNotificationEditForm';
import { BaseNotificationEditPage } from '../Base/BaseNotificationEditPage';
import { ScheduledNotification } from '../../types';
import {
  buildScheduledNotificationInputs,
  parseScheduledNotificationConfig,
} from './parseConfig';
import { useUpdateNotificationConfig } from '../../api/hooks/useUpdateNotificationConfig';
import {
  NotificationConfigRowFragment,
  useNotificationConfigs,
} from '../../api';

const createScheduledNotification = (
  seed: ScheduledNotification | null
): ScheduledNotification => ({
  id: seed?.id ?? FnUtils.generateUUID(),
  title: seed?.title ?? '',
  kind: seed?.kind ?? ConfigKind.Scheduled,
  recipientIds: seed?.recipientIds ?? [],
  recipientListIds: seed?.recipientListIds ?? [],
  sqlRecipientListIds: seed?.sqlRecipientListIds ?? [],
  parameters: seed?.parameters ?? '{}',
  parsedParameters: seed?.parsedParameters ?? {},
  scheduleFrequency: seed?.scheduleFrequency ?? 'daily',
  scheduleStartTime: seed?.scheduleStartTime ?? new Date(),
  subjectTemplate: seed?.subjectTemplate ?? '',
  bodyTemplate: seed?.bodyTemplate ?? '',
  sqlQueries: seed?.sqlQueries ?? [],
});

export const ScheduledNotificationEditPage = () => {
  const t = useTranslation('system');
  const { error } = useNotification();
  const parsingErrorSnack = error(t('error.parsing-notification-config'));

  const { id } = useParams<{ id: string }>();
  const [draft, setDraft] = useState<ScheduledNotification>(() =>
    createScheduledNotification(null)
  );

  // Get the notification config from the API
  const { data, isLoading } = useNotificationConfigs({
    filterBy: { id: { equalTo: id } },
  });
  useEffect(() => {
    const entity = data?.nodes[0];
    // Once we get the notification config from the API, parse it and load into the draft
    const parsedDraft = parseScheduledNotificationConfig(
      (entity as NotificationConfigRowFragment) ?? null,
      parsingErrorSnack
    );
    setDraft(createScheduledNotification(parsedDraft));
  }, [data]);

  const { mutateAsync: update, isLoading: updateIsLoading } =
    useUpdateNotificationConfig();

  const onSave = async (draft: ScheduledNotification) => {
    const inputs = buildScheduledNotificationInputs(draft);
    await update({ input: inputs.update });
  };

  const isInvalid =
    !draft.title ||
    // nothing selected
    // no recipients selected
    (!draft.recipientListIds.length && !draft.recipientIds.length);

  return (
    <BaseNotificationEditPage
      isLoading={isLoading || updateIsLoading}
      isInvalid={isInvalid}
      onSave={onSave}
      draft={draft}
      setDraft={setDraft}
      CustomForm={ScheduledNotificationEditForm}
    />
  );
};
