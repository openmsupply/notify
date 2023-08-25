import {
  CreateNotificationConfigInput,
  UpdateNotificationConfigInput,
} from '@common/types';
import { NotificationConfigRowFragment } from '../../api';
import { CCNotification } from '../../types';

export function parseColdChainNotificationConfig(
  config: NotificationConfigRowFragment | null,
  showError: () => void
): CCNotification | null {
  if (!config) return null;
  try {
    const {
      highTemp,
      lowTemp,
      confirmOk,
      remind,
      reminderInterval,
      reminderUnits,
      locationIds,
    } = JSON.parse(config.configurationData);

    return {
      id: config.id,
      title: config.title,
      kind: config.kind,
      recipientIds: config.recipientIds,
      recipientListIds: config.recipientListIds,
      highTemp,
      lowTemp,
      confirmOk,
      remind,
      reminderInterval,
      reminderUnits,
      locationIds,
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
    } as CCNotification;
  }
}

export function buildColdChainNotificationInputs(config: CCNotification): {
  create: CreateNotificationConfigInput;
  update: UpdateNotificationConfigInput;
} {
  const {
    highTemp,
    lowTemp,
    confirmOk,
    remind,
    reminderInterval,
    reminderUnits,
    locationIds,
  } = config;

  const input = {
    id: config.id,
    title: config.title,
    recipientIds: config.recipientIds,
    recipientListIds: config.recipientListIds,
    configurationData: JSON.stringify({
      highTemp,
      lowTemp,
      confirmOk,
      remind,
      reminderInterval,
      reminderUnits,
      locationIds,
    }),
  };

  return {
    create: { ...input, kind: config.kind },
    update: input,
  };
}
