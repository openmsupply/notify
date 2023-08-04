import {
  SortBy,
  LogSortInput,
  LogSortFieldInput,
  PaginationInput,
  FilterBy,
} from '@notify-frontend/common';
import { Sdk, LogRowFragment } from './operations.generated';

export type ListParams = {
  first: number;
  offset: number;
  sortBy: SortBy<LogRowFragment>;
  filterBy: FilterBy | null;
};

const logParsers = {
  toSortInput: (sortBy: SortBy<LogRowFragment>): LogSortInput => ({
    desc: sortBy.isDesc,
    key: sortBy.key as LogSortFieldInput,
  }),
  toPaginationInput: (first: number, offset: number): PaginationInput => ({
    first,
    offset,
  }),
};

export const getLogQueries = (sdk: Sdk) => ({
  get: {
    list: async ({ sortBy, first, offset, filterBy }: ListParams) => {
      const response = await sdk.logs({
        page: logParsers.toPaginationInput(first, offset),
        sort: [logParsers.toSortInput(sortBy)],
        filter: filterBy,
      });
      return response.logs;
    },
  },
});
