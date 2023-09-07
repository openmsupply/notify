import React, { useEffect } from 'react';
import { useTranslation } from '@common/intl';
import {
  useBreadcrumbs,
  useDetailPanel,
  useNotification,
  useQueryParamsState,
} from '@common/hooks';
import {
  AppBarContentPortal,
  Box,
  DataTable,
  NothingHere,
  Paper,
  SearchToolbar,
  TableProvider,
  createTableStore,
  useColumns,
} from '@common/ui';
import { useSqlRecipientLists } from '../api';
import { useParams } from 'packages/common/src';
import { useSQLRecipients } from '../api/hooks/useSQLRecipients';
import { RecipientQueryEditor } from './RecipientQueryEditor';
import { BasicRecipientRowFragment } from '../api/operations.generated';

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

  const { mutateAsync: testSqlRecipients, isLoading: recipientsLoading } =
    useSQLRecipients();
  const [sqlRecipients, setSqlRecipients] = React.useState(
    [] as BasicRecipientRowFragment[]
  );

  const queryRecipients = async (query: string, params: string) => {
    await testSqlRecipients({ sqlQuery: query, params })
      .then(result => {
        setSqlRecipients(result.testSqlRecipientListQuery.nodes);
      })
      .catch(err => {
        error(err.message)();
      });
  };

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

  const recipients = sqlRecipients.filter(
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
          <RecipientQueryEditor
            list={list}
            queryRecipients={queryRecipients}
            recipientsLoading={recipientsLoading}
          />
        </Paper>
      </AppBarContentPortal>
      {/* Recipients table */}
      <TableProvider createStore={createTableStore}>
        <Box sx={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
          <Box sx={{ margin: '16px' }}>
            <SearchToolbar filter={searchFilter} />
          </Box>
          <Box sx={{ flex: '1', overflow: 'auto' }}>
            <DataTable
              columns={columns}
              data={recipients}
              isError={isError}
              isLoading={isLoading || recipientsLoading}
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
