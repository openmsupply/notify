import { useGql } from '@notify-frontend/common';
import { getSdk } from './operations.generated';

export async function getNotificationConfig(id: string) {
  const { client } = useGql();
  const sdk = getSdk(client);

  const response = await sdk.NotificationConfigs({
    filter: { id: { equalTo: id } },
  });
  return response?.notificationConfigs.nodes[0];
}
