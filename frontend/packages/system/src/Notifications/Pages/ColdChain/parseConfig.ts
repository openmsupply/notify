import {
  ConfigKind,
  ConfigStatus,
  CreateNotificationConfigInput,
  UpdateNotificationConfigInput,
} from '@common/types';
import { CCNotification, ReminderUnits } from '../../types';
import { NotificationConfigRowFragment } from '../../api';
import { FnUtils, TeraUtils } from '@common/utils';

export function parseColdChainNotificationConfig(
  config: NotificationConfigRowFragment | null,
  showError: () => void
): CCNotification | null {
  if (!config) return null;
  try {
    return {
      ...defaultCCNotification,
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
      ...defaultCCNotification,
      id: config.id,
      title: config.title,
      kind: config.kind,
      status: config.status,
    } as CCNotification;
  }
}

export const defaultCCNotification: CCNotification = {
  id: FnUtils.generateUUID(),
  title: '',
  kind: ConfigKind.ColdChain,
  status: ConfigStatus.Disabled,
  recipientListIds: [],
  recipientIds: [],
  sqlRecipientListIds: [],
  parameters: '[]',
  parsedParameters: [],
  highTemp: true,
  lowTemp: true,
  confirmOk: true,
  noData: true,
  noDataInterval: 4,
  noDataUnits: ReminderUnits.HOURS,
  remind: true,
  reminderInterval: 2,
  reminderUnits: ReminderUnits.HOURS,
  messageAlertResolved: true,
  locationIds: [],
  requiredParameters: [],
};

export function buildColdChainNotificationInputs(config: CCNotification): {
  create: CreateNotificationConfigInput;
  update: UpdateNotificationConfigInput;
} {
  const params = [];
  for (const param of config.parsedParameters) {
    params.push(TeraUtils.keyedParamsAsTeraParams(param));
  }

  const input = {
    id: config.id,
    title: config.title,
    configurationData: JSON.stringify(config),
    status: config.status,
    parameters: JSON.stringify(params),
    recipientIds: config.recipientIds,
    recipientListIds: config.recipientListIds,
    sqlRecipientListIds: config.sqlRecipientListIds,
  };

  return {
    create: { ...input, kind: config.kind },
    update: input,
  };
}
