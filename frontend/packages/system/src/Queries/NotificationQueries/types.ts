export interface DraftNotificationQuery {
  id: string;
  name: string;
  description: string;
  query: string;
  requiredParameters: string[];
}
