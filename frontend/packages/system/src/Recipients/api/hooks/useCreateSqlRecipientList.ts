import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { SQL_RECIPIENT_LISTS } from '../../../cacheKeys';

export const useCreateSqlRecipientList = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  return useMutation(sdk.createSqlRecipientList, {
    onSettled: () => queryClient.invalidateQueries(SQL_RECIPIENT_LISTS),
  });
};
