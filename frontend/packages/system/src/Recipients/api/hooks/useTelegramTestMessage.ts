import { useGql, useMutation } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';

export const useTelegramTestMessage = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  return useMutation(sdk.sendTestTelegramMessage);
};
