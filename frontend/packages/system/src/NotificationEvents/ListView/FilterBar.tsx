import React, { useEffect, useMemo } from 'react';
import {
  useTranslation,
  FilterController,
  SearchBar,
  Box,
  Select,
  EventStatus,
  useEditModal,
  LoadingButton,
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

  // TODO: isError, isLoading
  // TODO: https://github.com/msupply-foundation/notify/issues/238 handle pagination
  const { data } = useNotificationConfigs({ first: 1000 });
  const notificationConfigs = data?.nodes ?? [];

  const { urlQuery, updateQuery } = useUrlQuery();

  const notificationConfigId = urlQuery.notificationConfigId;

  const selectedConfig = notificationConfigs.find(
    c => c.id === notificationConfigId
  );

  useEffect(() => {
    if (!notificationConfigId) {
      filter.onClearFilterRule('notificationConfigId');
    } else {
      filter.onChangeStringFilterRule(
        'notificationConfigId',
        'equalTo',
        notificationConfigId
      );
    }
  }, [notificationConfigId]);

  const setFilterConfig = (id: string) => {
    updateQuery({ ...urlQuery, notificationConfigId: id });
  };

  return (
    <>
      {isOpen && (
        <NotificationConfigModal
          isOpen={isOpen}
          onClose={onClose}
          setSelectedConfigId={setFilterConfig}
          notificationConfigs={notificationConfigs ?? []}
          selectedConfigId={notificationConfigId}
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
            {selectedConfig
              ? `Events for: ${selectedConfig.title}`
              : t('label.filter-by-notification-config')}
          </LoadingButton>

          <Box sx={{ gap: '10px', display: 'flex' }}>
            <ActionButtons />
          </Box>
        </Box>
      </Box>
    </>
  );
};
