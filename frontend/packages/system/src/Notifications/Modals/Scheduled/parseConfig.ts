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
      scheduleStartTime,
      subjectTemplate,
      bodyTemplate,
      sqlQueries,
    } = JSON.parse(config.configurationData);

    const scheduledNotification: ScheduledNotification = {
      id: config.id,
      title: config.title,
      kind: config.kind,
      parameters,
      scheduleFrequency,
      scheduleStartTime,
      recipientIds,
      recipientListIds,
      subjectTemplate,
      bodyTemplate,
      sqlQueries,
      status: config.status,
    };
    return scheduledNotification;
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
  const input = {
    id: config.id,
    title: config.title,
    configurationData: JSON.stringify(config),
    status: config.status,
  };
  return {
    create: { ...input, kind: config.kind},
    update: input,
  };
}
