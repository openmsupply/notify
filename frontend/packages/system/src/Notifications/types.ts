export enum NotificationConfigType {
  ColdChain = 'ColdChain',
}

export interface BaseNotificationConfig {
  id: string;
  title: string;
  configType: NotificationConfigType;
  recipientIds: string[];
  recipientListIds: string[];
}

export interface CCNotification extends BaseNotificationConfig {
  configType: NotificationConfigType.ColdChain;
  highTemp: boolean;
  lowTemp: boolean;
  confirmOk: boolean;
  remind: boolean;
  reminderInterval: number;
  reminderUnits: 'seconds' | 'minutes' | 'hours';
  locationIds: string[];
}
