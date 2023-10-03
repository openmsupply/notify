import {
  CreateNotificationConfigInput,
  UpdateNotificationConfigInput,
} from '@common/types';
import { CCNotification } from '../../types';
import { NotificationConfigRowFragment } from '../../api';
import { TeraUtils } from '@common/utils';

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
      noData,
      noDataInterval,
      noDataUnits,
      remind,
      reminderInterval,
      reminderUnits,
      messageAlertResolved,
      locationIds,
      recipientIds,
      recipientListIds,
      sqlRecipientListIds,
    } = JSON.parse(config.configurationData);

    return {
      id: config.id,
      title: config.title,
      kind: config.kind,
      highTemp,
      lowTemp,
      confirmOk,
      noData,
      noDataInterval,
      noDataUnits,
      remind,
      reminderInterval,
      reminderUnits,
      messageAlertResolved,
      locationIds,
      recipientIds,
      recipientListIds,
      status: config.status,
      sqlRecipientListIds,
      parameters: config.parameters,
      parsedParameters: TeraUtils.keyedParamsFromTeraJson(config.parameters),
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
      status: config.status,
    } as CCNotification;
  }
}

export function buildColdChainNotificationInputs(config: CCNotification): {
  create: CreateNotificationConfigInput;
  update: UpdateNotificationConfigInput;
} {
  const input = {
    id: config.id,
    title: config.title,
    configurationData: JSON.stringify(config),
    status: config.status,
    parameters: config.parameters,
  };

  return {
    create: { ...input, kind: config.kind },
    update: input,
  };
}
