import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { NOTIFICATION_QUERIES } from 'packages/system/src/cacheKeys';

export const useDeleteNotificationQuery = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  const result = useMutation(async (id: string) =>
    sdk.deleteNotificationQuery({ sqlRecipientListId: id })
  );
  return {
    ...result,
    invalidateQueries: () =>
      queryClient.invalidateQueries(NOTIFICATION_QUERIES),
  };
};
