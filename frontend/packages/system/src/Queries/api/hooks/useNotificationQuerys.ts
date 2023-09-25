import {
  FilterBy,
  RecipientListSortFieldInput,
  SortBy,
  useGql,
  useQuery,
} from '@notify-frontend/common';
import { NotificationQueryRowFragment, getSdk } from '../operations.generated';
import { NOTIFICATION_QUERIES } from '../../../cacheKeys';

export const useNotificationQuerys = ({
  filterBy,
  sortBy,
  first,
  offset,
}: {
  filterBy?: FilterBy | null;
  sortBy?: SortBy<NotificationQueryRowFragment>;
  first?: number;
  offset?: number;
} = {}) => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const cacheKeys = [NOTIFICATION_QUERIES, first, offset, filterBy, sortBy];

  return useQuery(cacheKeys, async () => {
    const response = await sdk.notificationQuerys({
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
    return response?.notificationQuerys;
  });
};
