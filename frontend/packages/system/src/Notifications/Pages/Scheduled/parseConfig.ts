import {
  ConfigKind,
  ConfigStatus,
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
      ...defaultSchedulerNotification,
      ...config,
      ...JSON.parse(config.configurationData),
    };
  } catch (e) {
    showError();
    // There's not much the user can do, except contact support or input the data again
    // and hope it saves in such a way that we can parse it next time!
    // The missing fields will be populated by default values in the edit modal, but we'll return
    // the base NotificationConfig data that is still usable:
    return {
      ...defaultSchedulerNotification,
      id: config.id,
      title: config.title,
      kind: config.kind,
    };
  }
}

export const defaultSchedulerNotification: ScheduledNotification = {
  id: FnUtils.generateUUID(),
  title: '',
  kind: ConfigKind.Scheduled,
  recipientListIds: [],
  recipientIds: [],
  sqlRecipientListIds: [],
  parameters: '{}',
  parsedParameters: {},
  scheduleFrequency: 'daily',
  scheduleStartTime: new Date(),
  subjectTemplate: '',
  bodyTemplate: '',
  sqlQueries: [],
  status: ConfigStatus.Disabled,
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
    status: config.status,
    parameters: config.parameters,
    recipientIds: config.recipientIds,
    recipientListIds: config.recipientListIds,
    sqlRecipientListIds: config.sqlRecipientListIds,
  };
  return {
    create: { ...input, kind: config.kind },
    update: input,
  };
}
