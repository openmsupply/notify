import React, { FC, useEffect, useRef } from 'react';
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
  UserAccountNode,
  LocalStorage,
} from '@notify-frontend/common';
import { RecipientRowFragment } from '../api';

type DeleteError = {
  userAccountName: string;
  message: string;
};

export const Toolbar: FC<{
  data: RecipientRowFragment[];
  filter: FilterController;
}> = ({ data, filter }) => {
  const t = useTranslation(['system']);
  // TODO: DELETE
  // const { success, info } = useNotification();
  // const [deleteErrors, setDeleteErrors] = React.useState<DeleteError[]>([]);
  // const { selectedRows } = useTableStore(state => ({
  //   selectedRows: Object.keys(state.rowState)
  //     .filter(id => state.rowState[id]?.isSelected)
  //     .map(selectedId => data?.find(({ id }) => selectedId === id))
  //     .filter(Boolean) as RecipientRowFragment[],
  // }));

  // const deleteAction = () => {
  //   const numberSelected = selectedRows.length;
  //   if (selectedRows && numberSelected > 0) {
  //     const errors: DeleteError[] = [];
  //     Promise.all(
  //       selectedRows.map(async userAccount => {
  //         await deleteUserAccount(userAccount).catch(err => {
  //           errors.push({
  //             userAccountName: userAccount.username,
  //             message: err.message,
  //           });
  //         });
  //       })
  //     ).then(() => {
  //       setDeleteErrors(errors);
  //       // Separate check for authorisation error, as this is
  //       // handled globally i.e. not caught above.
  //       // Not using useLocalStorage here, as hook result only updates
  //       // on re-render (after this function finishes running!)
  //       const authError = LocalStorage.getItem('/auth/error');
  //       if (errors.length === 0 && !authError) {
  //         const deletedMessage = t('messages.deleted-generic', {
  //           count: numberSelected,
  //         });
  //         const successSnack = success(deletedMessage);
  //         successSnack();
  //       }
  //     });
  //   } else {
  //     const selectRowsSnack = info(t('messages.select-rows-to-delete'));
  //     selectRowsSnack();
  //   }
  // };

  // const showDeleteConfirmation = useConfirmationModal({
  //   onConfirm: deleteAction,
  //   message: t('messages.confirm-delete-generic', {
  //     count: selectedRows.length,
  //   }),
  //   title: t('heading.are-you-sure'),
  // });

  // const ref = useRef(deleteAction);

  // useEffect(() => {
  //   ref.current = deleteAction;
  // }, [selectedRows]);

  const filterString = (filter.filterBy?.['search'] as string) || '';

  return (
    <AppBarContentPortal
      sx={{
        paddingBottom: '16px',
        flex: 1,
        justifyContent: 'space-between',
        display: 'flex',
      }}
    >
      {/* <AlertModal
        message={
          <ul>
            {deleteErrors.map(({ userAccountName, message }) => (
              <li key={userAccountName}>
                {userAccountName}: {message}
              </li>
            ))}
          </ul>
        }
        title={t('messages.error-deleting-generic', {
          count: deleteErrors.length,
        })}
        open={deleteErrors.length > 0}
        onOk={() => setDeleteErrors([])}
      /> */}
      <SearchBar
        placeholder={t('placeholder.search')}
        value={filterString}
        onChange={newValue => {
          filter.onChangeStringRule('search', newValue);
        }}
      />
      {/* <DropdownMenu label={t('label.select')}>
        <DropdownMenuItem
          disabled={!selectedRows.length}
          IconComponent={DeleteIcon}
          onClick={() => showDeleteConfirmation()}
        >
          {t('button.delete-lines')}
        </DropdownMenuItem>
      </DropdownMenu> */}
    </AppBarContentPortal>
  );
};
