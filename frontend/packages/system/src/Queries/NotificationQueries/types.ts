export interface DraftNotificationQuery {
  id: string;
  name: string;
  referenceName: string;
  description: string;
  query: string;
  requiredParameters: string[];
}
