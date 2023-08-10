import React from 'react';
import {
  Box,
  DataTable,
  NothingHere,
  TableProvider,
  createTableStore,
  useColumns,
} from '@common/ui';

type Recipient = {
  id: string;
  name: string;
  notificationType: string;
  toAddress: string;
};

export const RecipientTable = ({
  recipients,
  nothingHereMessage,
}: {
  recipients: Recipient[];
  nothingHereMessage: string;
}) => {
  const columns = useColumns([
    { key: 'name', label: 'label.name' },
    { key: 'notificationType', label: 'label.type' },
    { key: 'toAddress', label: 'label.address' },
  ]);

  return (
    <Box sx={{ flex: '1', overflow: 'auto' }}>
      <TableProvider createStore={createTableStore}>
        <DataTable
          columns={columns}
          data={recipients}
          isError={false}
          isLoading={false}
          noDataElement={<NothingHere body={nothingHereMessage} />}
        />
      </TableProvider>
    </Box>
  );
};
