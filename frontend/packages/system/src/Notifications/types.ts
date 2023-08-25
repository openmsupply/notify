import { ConfigKind } from '@common/types';
import { NotificationConfigRowFragment } from './api';

export type BaseNotificationConfig = Pick<
  NotificationConfigRowFragment,
  'id' | 'kind' | 'title' | 'recipientIds' | 'recipientListIds'
>;

export interface CCNotification extends BaseNotificationConfig {
  kind: ConfigKind.ColdChain;
  highTemp: boolean;
  lowTemp: boolean;
  confirmOk: boolean;
  remind: boolean;
  reminderInterval: number;
  reminderUnits: 'seconds' | 'minutes' | 'hours';
  locationIds: string[];
}
