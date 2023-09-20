import {
  ConfigKind,
  CreateNotificationConfigInput,
  UpdateNotificationConfigInput,
} from '@common/types';
import { NotificationConfigRowFragment } from '../../api';
import { ScheduledNotification } from '../../types';
import { FnUtils } from '@common/utils';

export function parseScheduledNotificationConfig(
  config: NotificationConfigRowFragment | null,
  showError: () => void
): ScheduledNotification | null {
  if (!config) return null;
  try {
    return {
      id: config.id,
      title: config.title,
      kind: config.kind,
      ...JSON.parse(config.configurationData),
    };
  } catch (e) {
    showError();
    // There's not much the user can do, except contact support or input the data again
    // and hope it saves in such a way that we can parse it next time!
    // The missing fields will be populated by default values in the edit modal, but we'll return
    // the base NotificationConfig data that is still usable:
    return {
      ...defautlSchedulerNotification,
      id: config.id,
      title: config.title,
      kind: config.kind,
    };
  }
}

export const defautlSchedulerNotification: ScheduledNotification = {
  id: FnUtils.generateUUID(),
  title: '',
  kind: ConfigKind.Scheduled,
  recipientListIds: [],
  recipientIds: [],
  parameters: '[]',
  scheduleFrequency: 'daily',
  scheduleStartTime: new Date(),
  subjectTemplate: '',
  bodyTemplate: '',
  sqlQueries: [],
};

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
  };
  return {
    create: { ...input, kind: config.kind },
    update: input,
  };
}
