import React, { useEffect } from 'react';
import { useTranslation } from '@common/intl';
import {
  useBreadcrumbs,
  useNotification,
  useQueryParamsState,
} from '@common/hooks';
import {
  AppBarContentPortal,
  Box,
  DataTable,
  LoadingButton,
  NothingHere,
  Paper,
  SearchToolbar,
  TableProvider,
  ZapIcon,
  createTableStore,
  useColumns,
} from '@common/ui';
import { useSqlRecipientLists } from '../api';
import { useParams } from 'packages/common/src';
import { useSQLRecipients } from '../api/hooks/useSQLRecipients';
import { BasicRecipientRow } from '../types/BasicRecipientRow';
import { SqlRecipientListEditForm } from './SqlRecipientListEditForm';

export const DetailEdit = () => {
  const t = useTranslation('system');
  const urlParams = useParams();
  const { suffix, setSuffix } = useBreadcrumbs();
  const { error } = useNotification();

  const { queryParams } = useQueryParamsState({
    initialFilter: { id: { equalTo: urlParams['listId'] } },
  });

  const { data, isError, isLoading } = useSqlRecipientLists(queryParams);
  const list = data?.nodes[0];

  const { mutateAsync: runSqlQuery, isLoading: sqlIsLoading } =
    useSQLRecipients();
  const [sqlRecipients, setSqlRecipients] = React.useState(
    [] as BasicRecipientRow[]
  );

  useEffect(() => {
    const listName = list?.name;
    if (!suffix && listName) {
      setSuffix(listName);
    }
  }, [suffix, list]);

  // useEffect(() => {
  //   if (list?.sqlQuery) {
  //     runSqlQuery(list?.sqlQuery)
  //       .then(result => {
  //         console.log(result);
  //         setSqlRecipients(result);
  //       })
  //       .catch(err => {
  //         error(err.message)();
  //       });
  //   }
  // }, [list]);

  const columns = useColumns([
    { key: 'name', label: 'label.name' },
    { key: 'notificationType', label: 'label.type' },
    { key: 'toAddress', label: 'label.address' },
  ]);

  // managing search in the frontend, seeing as all list members are already loaded
  const { filter: searchFilter } = useQueryParamsState();

  const searchString = (searchFilter.filterBy?.['search'] as string) ?? '';
  const allRecipients: BasicRecipientRow[] = sqlRecipients;

  const recipients = allRecipients.filter(
    r => r.name.includes(searchString) || r.toAddress.includes(searchString)
  );

  return (
    <>
      {/* Description/Details section */}
      <AppBarContentPortal sx={{ paddingBottom: '16px', flex: 1 }}>
        <Paper
          sx={{
            borderRadius: '16px',
            boxShadow: theme => theme.shadows[1],
            padding: '21px',
            height: 'fit-content',
            backgroundColor: 'background',
            display: 'flex',
            justifyContent: 'space-between',
            gap: '16px',
          }}
        >
          <SqlRecipientListEditForm list={list} />
        </Paper>
      </AppBarContentPortal>
      {/* Recipients table */}
      <TableProvider createStore={createTableStore}>
        <Box sx={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
          <Box sx={{ margin: '16px' }}>
            <SearchToolbar
              filter={searchFilter}
              ActionButtons={() => (
                <>
                  {list?.query && (
                    <LoadingButton
                      variant="outlined"
                      isLoading={sqlIsLoading}
                      startIcon={<ZapIcon />}
                      onClick={() => {
                        if (!list?.query) return;
                        runSqlQuery(list?.query)
                          .then(result => {
                            console.log(result);
                            setSqlRecipients(result);
                          })
                          .catch(err => {
                            error(err.message)();
                          });
                      }}
                    >
                      {t('label.refresh-sql-recipients')}
                    </LoadingButton>
                  )}
                </>
              )}
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
      </TableProvider>
    </>
  );
};
