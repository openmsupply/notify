import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { RECIPIENTS } from '../../../cacheKeys';

export const useDeleteRecipient = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  return useMutation(
    async (id: string) => sdk.deleteRecipient({ recipientId: id }),
    {
      onSettled: () => queryClient.invalidateQueries(RECIPIENTS),
    }
  );
};
