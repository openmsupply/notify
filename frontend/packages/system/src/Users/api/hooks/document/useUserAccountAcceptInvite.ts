import {
  AcceptUserInviteInput,
  useMutation,
  useQueryClient,
} from '@notify-frontend/common';
import { useUserAccountApi } from '../utils/useUserAccountApi';

type AcceptUserInviteParams = {
  input: AcceptUserInviteInput;
  token: string;
};

export const useUserAccountAcceptInvite = () => {
  const queryClient = useQueryClient();
  const api = useUserAccountApi();
  return useMutation(
    async (params: AcceptUserInviteParams) =>
      api.acceptInvite(params.input, params.token),
    { onSettled: () => queryClient.invalidateQueries(api.keys.base()) }
  );
};
