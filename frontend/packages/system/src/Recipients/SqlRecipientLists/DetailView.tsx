import React from 'react';
// import { useTranslation } from '@common/intl';
// import {
//   ModalMode,
//   useBreadcrumbs,
//   useEditModal,
//   useNotification,
//   useQueryParamsState,
// } from '@common/hooks';
// import {
//   AppBarContentPortal,
//   Box,
//   DataTable,
//   EditIcon,
//   LoadingButton,
//   NothingHere,
//   Paper,
//   PlusCircleIcon,
//   SearchAndDeleteToolbar,
//   TableProvider,
//   Typography,
//   ZapIcon,
//   createTableStore,
//   useColumns,
// } from '@common/ui';
// import { useSqlRecipientLists, useRemoveRecipientFromList } from '../api';
// import { useParams } from 'packages/common/src';
// import { SqlRecipientListRowFragment } from '../api/operations.generated';
// import { useSQLRecipients } from '../api/hooks/useSQLRecipients';
// import { SqlRecipientListEditModal } from './SqlRecipientListEditModal';

export const DetailView = () => {
  return <>Details View</>;
};

// export const DetailView = () => {
//   const t = useTranslation('system');
//   const urlParams = useParams();
//   const { suffix, setSuffix } = useBreadcrumbs();
//   const { error } = useNotification();

//   const { queryParams } = useQueryParamsState({
//     initialFilter: { id: { equalTo: urlParams['listId'] } },
//   });

//   const { data, isError, isLoading } = useSqlRecipientLists(queryParams);
//   const list = data?.nodes[0];

//   const { mutateAsync: runSqlQuery, isLoading: sqlIsLoading } =
//     useSQLRecipients();
//   const [sqlRecipients, setSqlRecipients] = React.useState(
//     [] as BasicRecipientRow[]
//   );
//   const { mutateAsync, invalidateQueries } = useRemoveRecipientFromList();

//   useEffect(() => {
//     const listName = list?.name;
//     if (!suffix && listName) {
//       setSuffix(listName);
//     }
//   }, [suffix, list]);

//   useEffect(() => {
//     if (list?.sqlQuery) {
//       runSqlQuery(list?.sqlQuery)
//         .then(result => {
//           console.log(result);
//           setSqlRecipients(result);
//         })
//         .catch(err => {
//           error(err.message)();
//         });
//     }
//   }, [list]);

//   const columns = useColumns([
//     { key: 'name', label: 'label.name' },
//     { key: 'notificationType', label: 'label.type' },
//     { key: 'toAddress', label: 'label.address' },
//     'selection',
//   ]);

//   // managing search in the frontend, seeing as all list members are already loaded
//   const { filter: searchFilter } = useQueryParamsState();

//   const searchString = (searchFilter.filterBy?.['search'] as string) ?? '';
//   const allRecipients: BasicRecipientRow[] = sqlRecipients.concat(
//     list?.recipients ?? []
//   );
//   const recipients = allRecipients.filter(
//     r => r.name.includes(searchString) || r.toAddress.includes(searchString)
//   );

//   return (
//     <>
//       {list &&
//       {editIsOpen && (
//         <SqlRecipientListEditModal
//           mode={ModalMode.Update}
//           isOpen={editIsOpen}
//           onClose={onCloseEdit}
//           recipientList={listEntity}
//         />
//       )}
//       {/* Description/Details section */}
//       <AppBarContentPortal sx={{ paddingBottom: '16px', flex: 1 }}>
//         <Paper
//           sx={{
//             borderRadius: '16px',
//             boxShadow: theme => theme.shadows[1],
//             padding: '21px',
//             height: 'fit-content',
//             backgroundColor: 'background.menu',
//             display: 'flex',
//             justifyContent: 'space-between',
//             gap: '16px',
//           }}
//         >
//           <Box>
//             <Typography
//               sx={{
//                 fontSize: '18px',
//                 fontWeight: 'bold',
//                 color: 'gray.dark',
//               }}
//             >
//               {list?.name}
//             </Typography>
//             <Typography sx={{ color: 'gray.dark' }}>
//               {list?.description}
//             </Typography>
//           </Box>
//           <LoadingButton
//             variant="outlined"
//             isLoading={false}
//             startIcon={<EditIcon />}
//             onClick={() => onOpenEdit(list)}
//           >
//             {t('label.edit')}
//           </LoadingButton>
//         </Paper>
//       </AppBarContentPortal>
//       {/* Recipients table */}
//       <TableProvider createStore={createTableStore}>
//         <Box sx={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
//           <Box sx={{ margin: '16px' }}>
//             <SearchAndDeleteToolbar
//               data={recipients}
//               filter={searchFilter}
//               deleteItem={removeRecipientFromList}
//               invalidateQueries={invalidateQueries}
//               deleteLabel={t('label.remove-members')}
//               ActionButtons={() => (
//                 <>
//                   {list?.sqlQuery && (
//                     <LoadingButton
//                       variant="outlined"
//                       isLoading={sqlIsLoading}
//                       startIcon={<ZapIcon />}
//                       onClick={() => {
//                         if (!list?.sqlQuery) return;
//                         runSqlQuery(list?.sqlQuery)
//                           .then(result => {
//                             console.log(result);
//                             setSqlRecipients(result);
//                           })
//                           .catch(err => {
//                             error(err.message)();
//                           });
//                       }}
//                     >
//                       {t('label.refresh-sql-recipients')}
//                     </LoadingButton>
//                   )}
//                 </>
//               )}
//             />
//           </Box>
//           <Box sx={{ flex: '1', overflow: 'auto' }}>
//             <DataTable
//               columns={columns}
//               data={recipients}
//               isError={isError}
//               isLoading={isLoading}
//               noDataElement={
//                 <NothingHere body={t('error.no-recipient-list-members')} />
//               }
//             />
//           </Box>
//         </Box>
//       </TableProvider>
//     </>
//   );
// };
