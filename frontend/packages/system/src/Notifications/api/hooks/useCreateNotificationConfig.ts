import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { NOTIFICATION_CONFIGS } from '../../../cacheKeys';

export const useCreateNotificationConfig = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  return useMutation(sdk.createNotificationConfig, {
    onSettled: () => queryClient.invalidateQueries(NOTIFICATION_CONFIGS),
  });
};
