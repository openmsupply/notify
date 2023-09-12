import {
  FilterBy,
  NotificationConfigSortFieldInput,
  SortBy,
  useGql,
  useQuery,
} from '@notify-frontend/common';
import {
  NotificationConfigRowFragment,
  getSdk,
} from './../operations.generated';
import { NOTIFICATION_CONFIGS } from '../../../cacheKeys';

export const useNotificationConfigs = ({
  filterBy,
  sortBy,
  first,
  offset,
}: {
  filterBy?: FilterBy | null;
  sortBy?: SortBy<NotificationConfigRowFragment>;
  first?: number;
  offset?: number;
} = {}) => {
  const { client } = useGql();
  const sdk = getSdk(client);

  const cacheKeys = [NOTIFICATION_CONFIGS, first, offset, filterBy, sortBy];

  return useQuery(cacheKeys, async () => {
    console.log('querying');
    const response = await sdk.NotificationConfigs({
      filter: filterBy,
      sort: sortBy?.key
        ? {
            desc: sortBy.isDesc ?? false,
            key: sortBy.key as NotificationConfigSortFieldInput,
          }
        : undefined,
      page: {
        first,
        offset,
      },
    });
    console.log('response', response);
    return response?.notificationConfigs;
  });
};
