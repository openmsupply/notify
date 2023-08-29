import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { RECIPIENT_LISTS } from '../../../cacheKeys';

export const useCreateRecipientList = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  return useMutation(sdk.createRecipientList, {
    onSettled: () => queryClient.invalidateQueries(RECIPIENT_LISTS),
  });
};
