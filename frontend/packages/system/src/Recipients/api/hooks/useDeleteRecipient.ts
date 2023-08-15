import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { RECIPIENTS } from '../../../cacheKeys';

export const useDeleteRecipient = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  const result = useMutation(async (id: string) =>
    sdk.deleteRecipient({ recipientId: id })
  );
  return {
    ...result,
    invalidateQueries: () => queryClient.invalidateQueries(RECIPIENTS),
  };
};
