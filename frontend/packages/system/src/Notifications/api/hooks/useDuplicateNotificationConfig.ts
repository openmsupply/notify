import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { NOTIFICATION_CONFIGS } from '../../../cacheKeys';

export const useDuplicateNotificationConfig = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  return useMutation(sdk.duplicateNotificationConfig, {
    onSettled: () => queryClient.invalidateQueries(NOTIFICATION_CONFIGS),
  });
};
