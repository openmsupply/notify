import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { RECIPIENT_LISTS } from 'packages/system/src/cacheKeys';

export const useDeleteRecipientList = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  const result = useMutation(async (id: string) =>
    sdk.deleteRecipientList({ recipientListId: id })
  );
  return {
    ...result,
    invalidateQueries: () => queryClient.invalidateQueries(RECIPIENT_LISTS),
  };
};
