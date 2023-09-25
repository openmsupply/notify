import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { NOTIFICATION_QUERIES } from '../../../cacheKeys';

export const useUpdateNotificationQuery = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  return useMutation(sdk.updateNotificationQuery, {
    onSettled: () => queryClient.invalidateQueries(NOTIFICATION_QUERIES),
  });
};
