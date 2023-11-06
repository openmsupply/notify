import React, { useMemo } from 'react';
import {
  useTranslation,
  FilterController,
  SearchBar,
  Box,
  Select,
  EventStatus,
} from '@notify-frontend/common';

export const FilterBar = ({
  filter,
  searchFilterKey = 'search',
  ActionButtons = () => <></>,
}: {
  filter: FilterController;
  searchFilterKey?: string;
  ActionButtons?: () => JSX.Element;
}) => {
  const t = useTranslation(['system']);
  const [timeRange, setTimeRange] = React.useState('all');

  const filterString = (filter.filterBy?.[searchFilterKey] as string) || '';

  const statusOptions = useMemo(() => {
    const statuses = Object.values(EventStatus).map(status => ({
      label: status as string,
      value: status as string,
    }));

    const options = [
      {
        label: 'Any Status',
        value: 'all',
      },
      ...statuses,
    ];
    return options;
  }, []);

  const statusValue = () => {
    const filterValue = filter.filterBy?.['status'];
    if (filterValue === 'all' || !filterValue) {
      return 'all';
    }
    if (typeof filterValue === 'string') {
      return filterValue;
    }
    return filterValue.equalTo;
  };

  const timeRangeOptions = useMemo(() => {
    const options = [
      {
        label: 'Any time',
        value: 'all',
      },
      {
        label: 'Last hour',
        value: '1',
      },
      {
        label: 'Last 24 hours',
        value: '24',
      },
      {
        label: 'Last Week',
        value: 24 * 7,
      },
    ];
    return options;
  }, []);

  return (
    <Box
      sx={{
        justifyContent: 'space-between',
        display: 'flex',
      }}
    >
      <SearchBar
        placeholder={t('placeholder.search')}
        value={filterString}
        onChange={newValue =>
          filter.onChangeStringRule(searchFilterKey, newValue)
        }
      />

      <Select
        value={statusValue()}
        options={statusOptions}
        onChange={e => {
          if (e.target.value === 'all') {
            filter.onClearFilterRule('status');
          } else {
            filter.onChangeStringFilterRule(
              'status',
              'equalTo',
              e.target.value
            );
          }
        }}
      />
      <Select
        value={timeRange}
        options={timeRangeOptions}
        onChange={e => {
          setTimeRange(e.target.value);
          if (e.target.value === 'all') {
            filter.onClearFilterRule('createdAt');
          } else {
            filter.onChangeDateFilterRule(
              'createdAt',
              'afterOrEqualTo',
              new Date(Date.now() - parseInt(e.target.value) * 60 * 60 * 1000)
            );
          }
        }}
      />
      <Box sx={{ gap: '10px', display: 'flex' }}>
        <ActionButtons />
      </Box>
    </Box>
  );
};
