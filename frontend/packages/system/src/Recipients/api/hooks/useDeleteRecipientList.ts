import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { RECIPIENT_LISTS } from 'packages/system/src/cacheKeys';

export const useDeleteRecipientList = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  return useMutation(
    async (id: string) => sdk.deleteRecipientList({ recipientListId: id }),
    {
      onSettled: () => queryClient.invalidateQueries(RECIPIENT_LISTS),
    }
  );
};
