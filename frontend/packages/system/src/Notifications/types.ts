import { ConfigKind, ConfigStatus } from '@common/types';
import { NotificationConfigRowFragment } from './api';

type BaseConfig = Pick<NotificationConfigRowFragment, 'id' | 'kind' | 'title' | 'status'>;

// TODO: this should go away once recipient/list ids come through from the backend on the base config
export interface BaseNotificationConfig extends BaseConfig {
  recipientIds: string[];
  recipientListIds: string[];
  status: ConfigStatus;
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