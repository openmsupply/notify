import {
  useMutation,
  useQueryClient,
} from '@notify-frontend/common';
import { usePasswordResetApi } from './utils/usePasswordResetApi';

export const useInitiatePasswordReset = () => {
  const queryClient = useQueryClient();
  const api = usePasswordResetApi();
  return useMutation(
    async (emailOrUserId: string) => api.initiatePasswordReset(emailOrUserId),
    {
      onSettled: () => queryClient.invalidateQueries(api.keys.base()),
    }
  );
};
