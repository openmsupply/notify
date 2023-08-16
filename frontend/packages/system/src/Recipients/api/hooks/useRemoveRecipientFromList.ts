import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { RECIPIENT_LISTS } from '../../../cacheKeys';

export const useRemoveRecipientFromList = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  const result = useMutation(sdk.removeRecipientFromList);

  return {
    ...result,
    invalidateQueries: () => queryClient.invalidateQueries(RECIPIENT_LISTS),
  };
};
