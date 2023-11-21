import React, { useMemo } from 'react';
import {
  useTranslation,
  FilterController,
  SearchBar,
  Box,
  Select,
  EventStatus,
  useEditModal,
  LoadingButton,
  useUrlQueryParams,
  FilterIcon,
  useUrlQuery,
} from '@notify-frontend/common';
import { NotificationConfigModal } from './NotificationConfigModal';
import { useNotificationConfigs } from '../../Notifications/api';

function relativeStartTime(hoursAgo: string) {
  return new Date(Date.now() - parseInt(hoursAgo) * 60 * 60 * 1000);
}

export const FilterBar = ({
  filter,
  searchFilterKey = 'search',
  ActionButtons = () => <></>,
}: {
  filter: FilterController;
  searchFilterKey?: string;
  ActionButtons?: () => JSX.Element;
}) => {
  const t = useTranslation('system');
  const [timeRange, setTimeRange] = React.useState('all');
  const { isOpen, onClose, onOpen } = useEditModal();

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

  // // maybe 50 or so is fine if searching.... ugh kind of no...
  // // TODO: what if more than 1000... there's a Pagination::all() on the back end but not sure how to use from frontend...
  const { queryParams } = useUrlQueryParams({ rowsPerPage: 1000 });

  // TODO: isError, isLoading
  // should this go here or the modal?? ... just gotta pass the name and id out...
  // but if we pass the name out, we need a way of getting the name if its already in the url so...
  const { data } = useNotificationConfigs(queryParams);
  const notificationConfigs = data?.nodes ?? [];

  const { urlQuery, updateQuery } = useUrlQuery();

  const setFilterConfig = (id: string) => {
    updateQuery({ ...urlQuery, notificationConfig: id });
  };

  return (
    <>
      {isOpen && (
        <NotificationConfigModal
          isOpen={isOpen}
          onClose={onClose}
          setSelectedConfig={setFilterConfig}
          notificationConfigs={notificationConfigs ?? []}
        />
      )}
      <Box
        sx={{
          gap: '14px',
          justifyContent: 'space-between',
          display: 'flex',
          flexWrap: 'wrap',
        }}
      >
        <SearchBar
          placeholder={t('placeholder.search')}
          value={filterString}
          onChange={newValue =>
            filter.onChangeStringRule(searchFilterKey, newValue)
          }
        />
        <Box
          sx={{
            gap: '14px',
            display: 'flex',
            alignItems: 'center',
          }}
        >
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
                  relativeStartTime(e.target.value)
                );
              }
            }}
          />

          <LoadingButton
            isLoading={false}
            startIcon={<FilterIcon />}
            onClick={() => onOpen()}
            variant="outlined"
          >
            {t('label.filter-by-notification-config')}
          </LoadingButton>

          <Box sx={{ gap: '10px', display: 'flex' }}>
            <ActionButtons />
          </Box>
        </Box>
      </Box>
    </>
  );
};
