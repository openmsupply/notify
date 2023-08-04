import {
  InviteUserInput,
  useMutation,
  useQueryClient,
} from '@notify-frontend/common';
import { useUserAccountApi } from '../utils/useUserAccountApi';

export const useUserAccountInvite = () => {
  const queryClient = useQueryClient();
  const api = useUserAccountApi();
  return useMutation(
    async (userAccount: InviteUserInput) => api.invite(userAccount),
    {
      onSettled: () => queryClient.invalidateQueries(api.keys.base()),
    }
  );
};
