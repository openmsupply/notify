import {
  FilterBy,
  RecipientSortFieldInput,
  SortBy,
  useGql,
  useQuery,
} from '@notify-frontend/common';
import { RecipientRowFragment, getSdk } from './../operations.generated';
import { RECIPIENTS } from '../../../cacheKeys';

export const useRecipients = ({
  filter,
  sortBy,
  first,
  offset,
}: {
  filter?: FilterBy | null;
  sortBy?: SortBy<RecipientRowFragment>;
  first?: number;
  offset?: number;
} = {}) => {
  const { client } = useGql();
  const sdk = getSdk(client);

  return useQuery(RECIPIENTS, async () => {
    const response = await sdk.Recipients({
      filter,
      sort: {
        desc: sortBy?.isDesc ?? false,
        key: sortBy?.key as RecipientSortFieldInput,
      },
      page: {
        first,
        offset,
      },
    });
    return response?.recipients;
  });
};
