import {
  FilterBy,
  NotificationEventSortFieldInput,
  SortBy,
  useGql,
  useQuery,
} from '@notify-frontend/common';
import { NotificationEventRowFragment, getSdk } from '../operations.generated';
import { NOTIFICATION_EVENTS } from '../../../cacheKeys';

export const useNotificationEvents = ({
  filterBy,
  sortBy,
  first,
  offset,
}: {
  filterBy?: FilterBy | null;
  sortBy?: SortBy<NotificationEventRowFragment>;
  first?: number;
  offset?: number;
} = {}) => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const cacheKeys = [NOTIFICATION_EVENTS, first, offset, filterBy, sortBy];

  return useQuery(cacheKeys, async () => {
    const response = await sdk.NotificationEvents({
      filter: filterBy,
      sort: sortBy?.key
        ? {
            desc: sortBy.isDesc ?? false,
            key: sortBy.key as NotificationEventSortFieldInput,
          }
        : undefined,
      page: {
        first,
        offset,
      },
    });
    return response?.notificationEvents;
  });
};
