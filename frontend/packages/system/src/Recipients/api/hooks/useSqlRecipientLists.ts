import {
  FilterBy,
  RecipientListSortFieldInput,
  SortBy,
  useGql,
  useQuery,
} from '@notify-frontend/common';
import { SqlRecipientListRowFragment, getSdk } from '../operations.generated';
import { SQL_RECIPIENT_LISTS } from '../../../cacheKeys';

export const useSqlRecipientLists = ({
  filterBy,
  sortBy,
  first,
  offset,
}: {
  filterBy?: FilterBy | null;
  sortBy?: SortBy<SqlRecipientListRowFragment>;
  first?: number;
  offset?: number;
} = {}) => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const cacheKeys = [SQL_RECIPIENT_LISTS, first, offset, filterBy, sortBy];

  return useQuery(cacheKeys, async () => {
    const response = await sdk.SqlRecipientLists({
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
    return response?.sqlRecipientLists;
  });
};
