import { ConfigKind } from '@common/types';
import { NotificationConfigRowFragment } from './api';

type BaseConfig = Pick<
  NotificationConfigRowFragment,
  'id' | 'kind' | 'title' | 'parameters'
>;

export interface BaseNotificationConfig extends BaseConfig {
  recipientIds: string[];
  recipientListIds: string[];
  sqlRecipientListIds: string[];
}

export interface CCNotification extends BaseNotificationConfig {
  kind: ConfigKind;
  highTemp: boolean;
  lowTemp: boolean;
  confirmOk: boolean;
  remind: boolean;
  reminderInterval: number;
  reminderUnits: 'seconds' | 'minutes' | 'hours';
  locationIds: string[];
}
