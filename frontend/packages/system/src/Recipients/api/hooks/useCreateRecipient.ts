import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { RECIPIENTS } from '../../../cacheKeys';

export const useCreateRecipient = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  return useMutation(sdk.createRecipient, {
    onSettled: () => queryClient.invalidateQueries(RECIPIENTS),
  });
};
