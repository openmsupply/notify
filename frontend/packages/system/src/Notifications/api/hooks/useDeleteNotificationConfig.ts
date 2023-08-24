import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { NOTIFICATION_CONFIGS } from '../../../cacheKeys';

export const useDeleteNotificationConfig = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  const result = useMutation(async (id: string) =>
    sdk.deleteNotificationConfig({ id })
  );
  return {
    ...result,
    invalidateQueries: () =>
      queryClient.invalidateQueries(NOTIFICATION_CONFIGS),
  };
};
