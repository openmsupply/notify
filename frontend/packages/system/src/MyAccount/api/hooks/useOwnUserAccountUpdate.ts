import {
  UpdateOwnUserAccountInput,
  useGql,
  useMutation,
  useQueryClient,
} from '@notify-frontend/common';
import { getSdk } from './../operations.generated';
import { USER_ACCOUNT } from '../../../cacheKeys';

export const useOwnUserAccountUpdate = () => {
  const { client } = useGql();
  const sdk = getSdk(client);
  const queryClient = useQueryClient();

  return useMutation(
    async (input: UpdateOwnUserAccountInput) =>
      sdk.updateOwnUserAccount({ input }),
    {
      onSettled: () => queryClient.invalidateQueries(USER_ACCOUNT),
    }
  );
};
