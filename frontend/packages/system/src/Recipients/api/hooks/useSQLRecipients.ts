import { useGql, useMutation } from '@notify-frontend/common';
import { getSdk } from '../operations.generated';

export interface BasicRecipientRow {
  id: string;
  name: string;
  notificationType: string;
  toAddress: string;
}

interface LcRecipientRow {
  id: string;
  name: string;
  notificationtype: string;
  toaddress: string;
}

export const useSQLRecipients = () => {
  const { client } = useGql();
  const sdk = getSdk(client);

  return useMutation(async (sqlQuery: string) => {
    const result = await sdk.recipientsViaSQL({
      sqlQuery: sqlQuery,
    });
    const recipients: LcRecipientRow[] = JSON.parse(result.runSqlQuery);
    // TODO: Refactor to use a specific endpoint and parse in backend for recipients...
    return recipients.map(
      recipient =>
        ({
          id: recipient.id,
          name: recipient.name,
          notificationType: recipient.notificationtype,
          toAddress: recipient.toaddress,
        } as BasicRecipientRow)
    );
  });
};
