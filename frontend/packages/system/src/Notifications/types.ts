import { NotificationConfigRowFragment } from './api';
import { KeyedParams } from '@common/utils';

type BaseConfig = Pick<
  NotificationConfigRowFragment,
  | 'id'
  | 'kind'
  | 'title'
  | 'status'
  | 'parameters'
  | 'recipientIds'
  | 'recipientListIds'
  | 'sqlRecipientListIds'
>;

export interface BaseNotificationConfig extends BaseConfig {
  parsedParameters: KeyedParams;
}

export interface CCNotification extends BaseNotificationConfig {
  highTemp: boolean;
  lowTemp: boolean;
  confirmOk: boolean;
  remind: boolean;
  reminderInterval: number;
  reminderUnits: 'seconds' | 'minutes' | 'hours';
  locationIds: string[];
}

export interface ScheduledNotification extends BaseNotificationConfig {
  scheduleFrequency: string;
  scheduleStartTime: Date;
  subjectTemplate: string;
  bodyTemplate: string;
  sqlQueries: string[];
}
