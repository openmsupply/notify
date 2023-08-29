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
  filterBy,
  sortBy,
  first,
  offset,
}: {
  filterBy?: FilterBy | null;
  sortBy?: SortBy<RecipientRowFragment>;
  first?: number;
  offset?: number;
} = {}) => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const cacheKeys = [RECIPIENTS, first, offset, filterBy, sortBy];

  return useQuery(cacheKeys, async () => {
    const response = await sdk.Recipients({
      filter: filterBy,
      sort: sortBy?.key
        ? {
            desc: sortBy.isDesc ?? false,
            key: sortBy.key as RecipientSortFieldInput,
          }
        : undefined,
      page: {
        first,
        offset,
      },
    });
    return response?.recipients;
  });
};
