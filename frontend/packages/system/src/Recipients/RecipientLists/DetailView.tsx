import React, { useEffect } from 'react';
import { useTranslation } from '@common/intl';
import {
  ModalMode,
  useBreadcrumbs,
  useEditModal,
  useQueryParamsState,
} from '@common/hooks';
import {
  AppBarButtonsPortal,
  AppBarContentPortal,
  Box,
  DataTable,
  EditIcon,
  LoadingButton,
  NothingHere,
  Paper,
  PlusCircleIcon,
  SearchAndDeleteToolbar,
  TableProvider,
  Typography,
  createTableStore,
  useColumns,
} from '@common/ui';
import { useRecipientLists } from '../api';
import { useParams } from 'packages/common/src';
import { RecipientListEditModal } from './RecipientListEditModal';
import { RecipientListRowFragment } from '../api/operations.generated';
import { ListMemberAddModal } from './RecipientListMemberAddModal';

export const DetailView = () => {
  const t = useTranslation('system');
  const urlParams = useParams();
  const { suffix, setSuffix } = useBreadcrumbs();
  const {
    isOpen: editIsOpen,
    onClose: onCloseEdit,
    onOpen: onOpenEdit,
    entity: listEntity,
  } = useEditModal<RecipientListRowFragment>();

  const {
    isOpen: addIsOpen,
    onClose: onCloseAdd,
    onOpen: onOpenAdd,
  } = useEditModal();

  const { queryParams } = useQueryParamsState({
    initialFilter: { id: { equalTo: urlParams['listId'] } },
  });

  const { data, isError, isLoading } = useRecipientLists(queryParams);
  const list = data?.nodes[0];

  useEffect(() => {
    const listName = list?.name;
    if (!suffix && listName) {
      setSuffix(listName);
    }
  }, [suffix, list]);

  const columns = useColumns([
    { key: 'name', label: 'label.name' },
    { key: 'notificationType', label: 'label.type' },
    { key: 'toAddress', label: 'label.address' },
  ]);

  // managing search in the frontend, seeing as all list members are already loaded
  const { filter: searchFilter } = useQueryParamsState();

  const searchString = (searchFilter.filterBy?.['search'] as string) ?? '';
  const recipients = (list?.recipients ?? []).filter(
    r => r.name.includes(searchString) || r.toAddress.includes(searchString)
  );

  return (
    <TableProvider createStore={createTableStore}>
      <Box sx={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
        {list && addIsOpen && (
          <ListMemberAddModal
            isOpen={addIsOpen}
            onClose={onCloseAdd}
            recipientList={list}
          />
        )}{' '}
        {editIsOpen && (
          <RecipientListEditModal
            mode={ModalMode.Update}
            isOpen={editIsOpen}
            onClose={onCloseEdit}
            recipientList={listEntity}
          />
        )}
        <AppBarButtonsPortal>
          <LoadingButton
            isLoading={false}
            startIcon={<PlusCircleIcon />}
            onClick={() => onOpenAdd()}
          >
            {t('label.add-members')}
          </LoadingButton>
        </AppBarButtonsPortal>
        <AppBarContentPortal sx={{ paddingBottom: '16px', flex: 1 }}>
          <Paper
            sx={{
              borderRadius: '16px',
              boxShadow: theme => theme.shadows[1],
              padding: '21px',
              height: 'fit-content',
              backgroundColor: 'background.menu',
              display: 'flex',
              justifyContent: 'space-between',
              gap: '16px',
            }}
          >
            <Box>
              <Typography
                sx={{
                  fontSize: '18px',
                  fontWeight: 'bold',
                  color: 'gray.dark',
                }}
              >
                {list?.name}
              </Typography>
              <Typography sx={{ color: 'gray.dark' }}>
                {list?.description}
              </Typography>
            </Box>
            <LoadingButton
              variant="outlined"
              isLoading={false}
              startIcon={<EditIcon />}
              onClick={() => onOpenEdit(list)}
            >
              {t('label.edit')}
            </LoadingButton>
          </Paper>
        </AppBarContentPortal>
        <Box
          sx={{
            flex: '1',
            overflow: 'auto',
            display: 'flex',
            flexDirection: 'column',
          }}
        >
          <Box sx={{ margin: '16px 16px 0 16px' }}>
            <SearchAndDeleteToolbar
              data={recipients}
              filter={searchFilter}
              deleteItem={async () => {}}
              invalidateQueries={async () => {}}
            />
          </Box>
          <Box sx={{ flex: '1', overflow: 'auto' }}>
            <DataTable
              columns={columns}
              data={recipients}
              isError={isError}
              isLoading={isLoading}
              noDataElement={
                <NothingHere body={t('error.no-recipient-list-members')} />
              }
            />
          </Box>
        </Box>
      </Box>
    </TableProvider>
  );
};
