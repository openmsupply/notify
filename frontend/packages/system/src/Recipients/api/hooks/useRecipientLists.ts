import {
  FilterBy,
  RecipientListSortFieldInput,
  SortBy,
  useGql,
  useQuery,
} from '@notify-frontend/common';
import { RecipientListRowFragment, getSdk } from '../operations.generated';
import { RECIPIENT_LISTS } from '../../../cacheKeys';

export const useRecipientLists = ({
  filterBy,
  sortBy,
  first,
  offset,
}: {
  filterBy?: FilterBy | null;
  sortBy?: SortBy<RecipientListRowFragment>;
  first?: number;
  offset?: number;
} = {}) => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const cacheKeys = [RECIPIENT_LISTS, first, offset, filterBy, sortBy];

  return useQuery(cacheKeys, async () => {
    const response = await sdk.RecipientLists({
      filter: filterBy,
      sort: sortBy?.key
        ? {
            desc: sortBy.isDesc ?? false,
            key: sortBy.key as RecipientListSortFieldInput,
          }
        : undefined,
      page: {
        first,
        offset,
      },
    });
    return response?.recipientLists;
  });
};
