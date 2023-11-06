import React, { useMemo } from 'react';
import {
  useTranslation,
  FilterController,
  SearchBar,
  Box,
  Grid,
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
      <Grid item>
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
      </Grid>
      <Box sx={{ gap: '10px', display: 'flex' }}>
        <ActionButtons />
      </Box>
    </Box>
  );
};
