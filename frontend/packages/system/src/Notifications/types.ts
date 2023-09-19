import { ConfigKind } from '@common/types';
import { NotificationConfigRowFragment } from './api';

type BaseConfig = Pick<NotificationConfigRowFragment, 'id' | 'kind' | 'title'>;

// TODO: this should go away once recipient/list ids come through from the backend on the base config
export interface BaseNotificationConfig extends BaseConfig {
  recipientIds: string[];
  recipientListIds: string[];
}

export interface CCNotification extends BaseNotificationConfig {
  kind: ConfigKind;
  highTemp: boolean;
  lowTemp: boolean;
  confirmOk: boolean;
  noData: boolean;
  noDataInterval: number;
  noDataUnits: 'seconds' | 'minutes' | 'hours';
  remind: boolean;
  reminderInterval: number;
  reminderUnits: 'seconds' | 'minutes' | 'hours';
  locationIds: string[];
}
