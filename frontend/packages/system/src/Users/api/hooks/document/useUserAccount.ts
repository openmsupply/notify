import { useQuery, useUrlQueryParams } from '@notify-frontend/common';
import { useUserAccountApi } from '../utils/useUserAccountApi';

export const useUserAccounts = () => {
  const api = useUserAccountApi();
  const { queryParams } = useUrlQueryParams({
    filterKey: 'username',
    initialSort: { key: 'username', dir: 'asc' },
    additionalFilters: [
      {
        key: 'search',
      },
    ],
  });

  const result = useQuery(api.keys.paramList(queryParams), () =>
    api.get.list({ ...queryParams })
  );

  return { ...queryParams, ...result };
};
