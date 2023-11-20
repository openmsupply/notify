import { NotificationTypeNode } from '@common/types';

export type EventContext = {
  recipient?: {
    name: string;
    notification_type: NotificationTypeNode;
    to_address: string;
  };
  [key: string]: unknown;
};

export const useParsedEventContext = (
  context: string | null | undefined
): EventContext => {
  if (!context) return {};
  try {
    return JSON.parse(context);
  } catch (e) {
    return {};
  }
};
