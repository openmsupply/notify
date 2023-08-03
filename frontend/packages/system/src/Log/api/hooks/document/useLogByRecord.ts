import { useQuery, useUrlQueryParams } from '@notify-frontend/common';
import { useLogApi } from '../utils/useLogApi';

export const useLogByRecord = (recordId: string) => {
  const api = useLogApi();
  const { queryParams, updatePaginationQuery, updateSortQuery } =
    useUrlQueryParams({
      initialSort: { key: 'datetime', dir: 'desc' },
      initialFilter: { recordId: { equalTo: recordId } },
    });
  const { first, offset, sortBy, filterBy } = queryParams;
  const result = useQuery(
    api.keys.paramList({ first, offset, sortBy, filterBy }),
    () => api.get.list({ first, offset, sortBy, filterBy })
  );
  return { updatePaginationQuery, updateSortQuery, ...queryParams, ...result };
};
