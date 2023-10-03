import { ConfigKind } from '@common/types';
import { NotificationConfigRowFragment } from './api';
import { KeyedParams } from '@common/utils';
import { LocaleKey, TypedTFunction } from '@common/intl';

export enum ReminderUnits {
  MINUTES = 'minutes',
  HOURS = 'hours',
}

export function getReminderUnitsFromString(str: string): ReminderUnits {
  switch (str) {
    case 'minutes':
      return ReminderUnits.MINUTES;
    case 'hours':
      return ReminderUnits.HOURS;
    default:
      throw new Error(`Invalid reminder units: ${str}`);
  }
}

export function getReminderUnitsAsOptions(t: TypedTFunction<LocaleKey>): {
  label: string;
  value: string;
}[] {
  return [
    { label: t('label.minutes'), value: ReminderUnits.MINUTES },
    { label: t('label.hours'), value: ReminderUnits.HOURS },
  ];
}

type BaseConfig = Pick<
  NotificationConfigRowFragment,
  'id' | 'kind' | 'title' | 'status' | 'parameters'
>;

export interface BaseNotificationConfig extends BaseConfig {
  recipientIds: string[];
  recipientListIds: string[];
  sqlRecipientListIds: string[];
  parsedParameters: KeyedParams;
}

export interface CCNotification extends BaseNotificationConfig {
  kind: ConfigKind;
  highTemp: boolean;
  lowTemp: boolean;
  confirmOk: boolean;
  noData: boolean;
  noDataInterval: number;
  noDataUnits: ReminderUnits;
  remind: boolean;
  reminderInterval: number;
  reminderUnits: ReminderUnits;
  locationIds: string[];
  sensorIds: string[];
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
