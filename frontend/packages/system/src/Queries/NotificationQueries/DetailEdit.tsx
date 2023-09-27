import React, { useEffect } from 'react';
import {
  useBreadcrumbs,
  useDetailPanel,
  useNotification,
  useQueryParamsState,
} from '@common/hooks';
import {
  AppBarButtonsPortal,
  AppBarContentPortal,
  BasicSpinner,
  Box,
  Paper,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
} from '@common/ui';
import { useTranslation } from '@common/intl';
import { useNotificationQuerys, useTestNotificationQuery } from '../api';
import { useParams } from 'packages/common/src';
import { QueryEditor } from './QueryEditor';
import { stringifyObjectKey } from './utils';

export const DetailEdit = () => {
  const t = useTranslation('system');
  const urlParams = useParams();
  const { suffix, setSuffix } = useBreadcrumbs();
  const { error } = useNotification();
  const { OpenButton } = useDetailPanel(t('label.parameters'));

  const { queryParams } = useQueryParamsState({
    initialFilter: { id: { equalTo: urlParams['id'] } },
  });

  const { data, isLoading } = useNotificationQuerys(queryParams);
  const entity = data?.nodes[0];

  useEffect(() => {
    const listName = entity?.name;
    if (!suffix && listName) {
      setSuffix(listName);
    }
  }, [suffix, entity]);

  const { mutateAsync: testNotificationQuery, isLoading: queryLoading } =
    useTestNotificationQuery();
  const [sqlResults, setSqlResults] = React.useState([] as never[]);
  const [queryColumns, setQueryColumns] = React.useState(['id'] as string[]);

  const runQuery = async (query: string, params: string) => {
    await testNotificationQuery({ sqlQuery: query, params: params })
      .then(result => {
        const results = JSON.parse(result.runSqlQueryWithParameters);

        const columns = Object.keys(results[0] ?? []);
        // If we have an id column, move it to the front
        // Would be nice to return the columns in the same order as the query specifies, but seems out of scope for now...
        const idIndex = columns.indexOf('id');
        if (idIndex > -1) {
          columns.splice(idIndex, 1);
          columns.unshift('id');
        }
        setQueryColumns(columns);

        setSqlResults(results);
      })
      .catch(err => {
        error(err.message)();
      });
  };

  return (
    <>
      <AppBarButtonsPortal>{OpenButton}</AppBarButtonsPortal>
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
          {entity && !isLoading ? (
            <QueryEditor
              entity={entity}
              runQuery={runQuery}
              queryLoading={queryLoading}
            />
          ) : (
            <BasicSpinner />
          )}
        </Paper>
      </AppBarContentPortal>
      {/* Sql Results table */}
      <Box sx={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
        <Box sx={{ flex: '1', overflow: 'auto' }}>
          <Table>
            <TableHead>
              <TableRow>
                {queryColumns.map(column => (
                  <TableCell
                    key={column}
                    role="columnheader"
                    sx={{
                      backgroundColor: 'transparent',
                      borderBottom: '0px',
                      paddingLeft: '16px',
                      paddingRight: '16px',
                      fontWeight: 'bold',
                      fontSize: '14px',
                    }}
                  >
                    {column}
                  </TableCell>
                ))}
              </TableRow>
            </TableHead>
            <TableBody>
              {sqlResults.map((row, idx) => (
                <TableRow key={`row-${idx}`}>
                  {queryColumns.map(column => (
                    <TableCell key={`row-${idx}-${column}`}>
                      {stringifyObjectKey(row[column])}
                    </TableCell>
                  ))}
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </Box>
      </Box>
    </>
  );
};
