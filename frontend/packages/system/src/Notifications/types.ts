import { ConfigKind } from '@common/types';
import { NotificationConfigRowFragment } from './api';

type BaseConfig = Pick<NotificationConfigRowFragment, 'id' | 'kind' | 'title'>;

export interface BaseNotificationConfig extends BaseConfig {
  recipientIds: string[];
  recipientListIds: string[];
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
export interface ScheduledNotification extends BaseNotificationConfig {
  kind: ConfigKind;
  parameters: string; // JSON for now
  scheduleFrequency: string;
  scheduleStartTime: Date;
  subjectTemplate: string;
  bodyTemplate: string;
  sqlQueries: string[];
}
