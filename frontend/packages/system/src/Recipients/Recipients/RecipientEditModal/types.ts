import { NotificationTypeNode } from '@common/types';

export interface DraftRecipient {
  id: string;
  name: string;
  toAddress: string;
  notificationType: NotificationTypeNode;
}
