import React from 'react';
import {
  useTranslation,
  FilterController,
  SearchBar,
  RecordWithId,
  Box,
} from '@notify-frontend/common';

export const SearchToolbar = <T extends RecordWithId>({
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
      <Box sx={{ gap: '10px', display: 'flex' }}>
        <ActionButtons />
      </Box>
    </Box>
  );
};
