import { useGql, SortBy, FilterBy } from '@notify-frontend/common';
import { getLogQueries, ListParams } from '../../api';
import { getSdk, LogRowFragment } from '../../operations.generated';

export const useLogApi = () => {
  const keys = {
    base: () => ['log'] as const,
    list: () => [...keys.base(), 'list'] as const,
    paramList: (params: ListParams) => [...keys.list(), params] as const,
    sortList: (sortBy: SortBy<LogRowFragment>, filterBy?: FilterBy) =>
      [...keys.list(), sortBy, filterBy] as const,
  };
  const { client } = useGql();
  const sdk = getSdk(client);
  const queries = getLogQueries(sdk);
  return { ...queries, keys };
};
