import React from 'react';
import {
  useTranslation,
  AppBarContentPortal,
  FilterController,
  SearchBar,
  UserAccountNode,
} from '@notify-frontend/common';

export const Toolbar = ({ filter }: { filter: FilterController }) => {
  const t = useTranslation(['system']);

  const key = 'username' as keyof UserAccountNode;
  const filterString = (filter.filterBy?.[key]?.like as string) || '';

  return (
    <AppBarContentPortal
      sx={{
        paddingBottom: '16px',
        flex: 1,
        justifyContent: 'space-between',
        display: 'flex',
      }}
    >
      <SearchBar
        placeholder={t('placeholder.search')}
        value={filterString}
        onChange={newValue => {
          filter.onChangeStringFilterRule('search', 'like', newValue);
        }}
      />
    </AppBarContentPortal>
  );
};
