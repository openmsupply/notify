import { NotificationConfigRowFragment } from '../../api';
import { CCNotification } from '../../types';

export function parseColdChainNotificationConfig(
  config: NotificationConfigRowFragment
): CCNotification {
  const {
    highTemp,
    lowTemp,
    confirmOk,
    remind,
    reminderInterval,
    reminderUnits,
    locationIds,
    recipientIds,
    recipientListIds,
    // TODO: actually some checks/error handling here!
  } = JSON.parse(config.configurationData);

  return {
    id: config.id,
    title: config.title,
    kind: config.kind,
    highTemp,
    lowTemp,
    confirmOk,
    remind,
    reminderInterval,
    reminderUnits,
    locationIds,
    recipientIds,
    recipientListIds,
  };
}
