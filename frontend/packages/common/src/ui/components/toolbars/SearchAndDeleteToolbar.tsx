import React, { useEffect, useRef } from 'react';
import {
  useNotification,
  DropdownMenu,
  DropdownMenuItem,
  useTranslation,
  DeleteIcon,
  useTableStore,
  FilterController,
  AlertModal,
  useConfirmationModal,
  SearchBar,
  LocalStorage,
  RecordWithId,
  FilterRule,
  Box,
} from '@notify-frontend/common';

export const SearchAndDeleteToolbar = <T extends RecordWithId>({
  data,
  filter,
  deleteItem,
  invalidateQueries,
  searchFilterKey = 'search',
  asStringFilterRule: asStringFilter = false,
}: {
  data: T[];
  filter: FilterController;
  deleteItem: (id: string) => Promise<unknown>;
  invalidateQueries: () => Promise<void>;
  searchFilterKey?: string;
  /** Add the search term as a StringFilterRule rather than just a StringRule */
  asStringFilterRule?: boolean;
}) => {
  const t = useTranslation(['system']);
  const { success, info } = useNotification();

  const [errorCount, setErrorCount] = React.useState(0);

  const { selectedRows } = useTableStore(state => ({
    selectedRows: Object.keys(state.rowState)
      .filter(id => state.rowState[id]?.isSelected)
      .map(selectedId => data?.find(({ id }) => selectedId === id))
      .filter(Boolean) as T[],
  }));

  const deleteAction = () => {
    if (selectedRows.length) {
      let deleteErrorCount = 0;
      Promise.all(
        selectedRows.map(async item => {
          await deleteItem(item.id).catch(() => {
            deleteErrorCount += 1;
          });
        })
      ).then(() => {
        setErrorCount(deleteErrorCount);
        // Separate check for authorisation error, as this is handled globally i.e. not caught above.
        // Not using useLocalStorage here, as hook result only updates on re-render (after this function finishes running!)
        const authError = LocalStorage.getItem('/auth/error');
        if (deleteErrorCount === 0 && !authError) {
          const deletedMessage = t('messages.deleted-generic', {
            count: selectedRows.length,
          });
          const successSnack = success(deletedMessage);
          successSnack();
        }
        invalidateQueries();
      });
    } else {
      const selectRowsSnack = info(t('messages.select-rows-to-delete'));
      selectRowsSnack();
    }
  };

  const showDeleteConfirmation = useConfirmationModal({
    onConfirm: deleteAction,
    message: t('messages.confirm-delete-generic', {
      count: selectedRows.length,
    }),
    title: t('heading.are-you-sure'),
  });

  const ref = useRef(deleteAction);

  useEffect(() => {
    ref.current = deleteAction;
  }, [selectedRows]);

  const filterString =
    (asStringFilter
      ? ((filter.filterBy?.[searchFilterKey] as FilterRule)?.like as string)
      : (filter.filterBy?.[searchFilterKey] as string)) || '';

  return (
    <Box
      sx={{
        justifyContent: 'space-between',
        display: 'flex',
      }}
    >
      <AlertModal
        title={t('error.something-wrong')}
        message={t('messages.error-deleting-generic', {
          count: errorCount,
        })}
        open={errorCount > 0}
        onOk={() => setErrorCount(0)}
      />
      <SearchBar
        placeholder={t('placeholder.search')}
        value={filterString}
        onChange={newValue => {
          if (asStringFilter) {
            filter.onChangeStringFilterRule(searchFilterKey, 'like', newValue);
          } else filter.onChangeStringRule(searchFilterKey, newValue);
        }}
      />
      <DropdownMenu label={t('label.select')}>
        <DropdownMenuItem
          disabled={!selectedRows.length}
          IconComponent={DeleteIcon}
          onClick={() => showDeleteConfirmation()}
        >
          {t('button.delete-lines')}
        </DropdownMenuItem>
      </DropdownMenu>
    </Box>
  );
};
