import { useGql, useMutation, useQueryClient } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';
import { SQL_RECIPIENT_LISTS } from 'packages/system/src/cacheKeys';

export const useDeleteSqlRecipientList = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const queryClient = useQueryClient();

  const result = useMutation(async (id: string) =>
    sdk.deleteSqlRecipientList({ sqlRecipientListId: id })
  );
  return {
    ...result,
    invalidateQueries: () => queryClient.invalidateQueries(SQL_RECIPIENT_LISTS),
  };
};
