import {
  CreateNotificationConfigInput,
  UpdateNotificationConfigInput,
} from '@common/types';
import { NotificationConfigRowFragment } from '../../api';
import { ScheduledNotification } from '../../types';

export function parseScheduledNotificationConfig(
  config: NotificationConfigRowFragment | null,
  showError: () => void
): ScheduledNotification | null {
  if (!config) return null;
  try {
    const {
      recipientIds,
      recipientListIds,
      parameters,
      scheduleFrequency,
      subjectTemplate,
      bodyTemplate,
    } = JSON.parse(config.configurationData);

    return {
      id: config.id,
      title: config.title,
      kind: config.kind,
      parameters,
      scheduleFrequency,
      recipientIds,
      recipientListIds,
      subjectTemplate,
      bodyTemplate,
    };
  } catch (e) {
    showError();
    // There's not much the user can do, except contact support or input the data again
    // and hope it saves in such a way that we can parse it next time!
    // The missing fields will be populated by default values in the edit modal, but we'll return
    // the base NotificationConfig data that is still usable:
    return {
      id: config.id,
      title: config.title,
      kind: config.kind,
    } as ScheduledNotification;
  }
}

export function buildScheduledNotificationInputs(
  config: ScheduledNotification
): {
  create: CreateNotificationConfigInput;
  update: UpdateNotificationConfigInput;
} {
  const {
    recipientIds,
    recipientListIds,
    scheduleFrequency,
    subjectTemplate,
    bodyTemplate,
  } = config;

  const input = {
    id: config.id,
    title: config.title,
    configurationData: JSON.stringify({
      recipientIds,
      recipientListIds,
      scheduleFrequency,
      subjectTemplate,
      bodyTemplate,
    }),
  };
  return {
    create: { ...input, kind: config.kind },
    update: input,
  };
}
