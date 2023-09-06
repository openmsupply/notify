import { useGql, useMutation } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';

export interface SqlRecipientParams {
  sqlQuery: string;
  params: string;
}

export const useSQLRecipients = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  return useMutation(sdk.testSqlRecipientListQuery);
};
