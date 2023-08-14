import React, { useEffect, useRef } from 'react';
import {
  useNotification,
  DropdownMenu,
  DropdownMenuItem,
  useTranslation,
  DeleteIcon,
  useTableStore,
  AppBarContentPortal,
  FilterController,
  AlertModal,
  useConfirmationModal,
  SearchBar,
  LocalStorage,
  RecordWithId,
} from '@notify-frontend/common';

type DeleteError = {
  // TODO: name?
  id: string;
  message: string;
};

export const SearchAndDeleteToolbar = <T extends RecordWithId>({
  data,
  filter,
  deleteItem,
  searchFilterKey = 'search',
}: {
  data: T[];
  filter: FilterController;
  deleteItem: (id: string) => Promise<unknown>;
  searchFilterKey?: string;
}) => {
  const t = useTranslation(['system']);
  const { success, info } = useNotification();

  const [deleteErrors, setDeleteErrors] = React.useState<DeleteError[]>([]);

  const { selectedRows } = useTableStore(state => ({
    selectedRows: Object.keys(state.rowState)
      .filter(id => state.rowState[id]?.isSelected)
      .map(selectedId => data?.find(({ id }) => selectedId === id))
      .filter(Boolean) as T[],
  }));

  const deleteAction = () => {
    if (selectedRows.length) {
      const errors: DeleteError[] = [];
      Promise.all(
        selectedRows.map(async item => {
          await deleteItem(item.id).catch(err => {
            errors.push({
              id: item.id,
              message: err.message,
            });
          });
        })
      ).then(() => {
        setDeleteErrors(errors);
        // Separate check for authorisation error, as this is handled globally i.e. not caught above.
        // Not using useLocalStorage here, as hook result only updates on re-render (after this function finishes running!)
        const authError = LocalStorage.getItem('/auth/error');
        if (errors.length === 0 && !authError) {
          const deletedMessage = t('messages.deleted-generic', {
            count: selectedRows.length,
          });
          const successSnack = success(deletedMessage);
          successSnack();
        }
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

  const filterString = (filter.filterBy?.[searchFilterKey] as string) || '';

  return (
    <AppBarContentPortal
      sx={{
        paddingBottom: '16px',
        flex: 1,
        justifyContent: 'space-between',
        display: 'flex',
      }}
    >
      <AlertModal
        message={
          <ul>
            {deleteErrors.map(({ id, message }) => (
              <li key={id}>
                {id}: {message}
              </li>
            ))}
          </ul>
        }
        title={t('messages.error-deleting-generic', {
          count: deleteErrors.length,
        })}
        open={deleteErrors.length > 0}
        onOk={() => setDeleteErrors([])}
      />
      <SearchBar
        placeholder={t('placeholder.search')}
        value={filterString}
        onChange={newValue => {
          filter.onChangeStringRule(searchFilterKey, newValue);
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
    </AppBarContentPortal>
  );
};
