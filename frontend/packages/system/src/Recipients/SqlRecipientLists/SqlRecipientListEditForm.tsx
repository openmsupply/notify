import React, { useState } from 'react';
import {
  BasicTextInput,
  Box,
  BufferedTextArea,
  BufferedTextInput,
  FnUtils,
  Grid,
  KeyedParams,
  LoadingButton,
  SaveIcon,
  Table,
  TableCell,
  TableRow,
  TeraUtils,
  Typography,
  ZapIcon,
  useNotification,
  useToggle,
  useTranslation,
} from '@notify-frontend/common';
import { DraftSqlRecipientList } from './types';
import { useCreateSqlRecipientList, useUpdateSqlRecipientList } from '../api';
import { SqlRecipientListRowFragment } from '../api/operations.generated';

const createSqlRecipientList = (
  seed?: DraftSqlRecipientList | null
): DraftSqlRecipientList => ({
  id: FnUtils.generateUUID(),
  name: '',
  description: '',
  query: '',
  parameters: '{}',
  ...seed,
});

export const invalidName = (name: string) => {
  const nameIncorrectLength = name.length < 3 || name.length > 75;
  const nameContainsIllegalChars = name.match(/[^ 0-9A-Za-z_\-@.+:/()]/);

  return !name.trim() || !!nameContainsIllegalChars || nameIncorrectLength;
};

type SqlRecipientListEditFormProps = {
  list: SqlRecipientListRowFragment | undefined;
  queryRecipients: (query: string, params: string) => Promise<void>;
  recipientsLoading: boolean;
};

export const SqlRecipientListEditForm = ({
  list,
  queryRecipients,
  recipientsLoading,
}: SqlRecipientListEditFormProps) => {
  const t = useTranslation('system');

  const { error } = useNotification();
  const {
    isOn: isEditingName,
    toggleOn: editNameToggleOn,
    toggleOff: editNameToggleOff,
  } = useToggle(!(list === undefined || list === null || list.name !== ''));

  const [draft, setDraft] = useState(createSqlRecipientList(list));
  const onUpdate = (patch: Partial<DraftSqlRecipientList>) => {
    setDraft({ ...draft, ...patch });
  };

  const [queryParams, setQueryParams] = useState<KeyedParams>({});
  const onUpdateQueryParams = (key: string, value: string) => {
    const patch = { [key]: value };
    setQueryParams({ ...queryParams, ...patch });
  };

  const { mutateAsync: create, isLoading: createIsLoading } =
    useCreateSqlRecipientList();
  const { mutateAsync: update, isLoading: updateIsLoading } =
    useUpdateSqlRecipientList();

  const onSave = async (draft: DraftSqlRecipientList) => {
    const { id, name, description, query, parameters } = draft;
    const input = { id, name, description, query, parameters };

    if (!list) await create({ input });
    else await update({ input });

    editNameToggleOff();
  };

  return (
    <Box sx={{ width: '100%' }}>
      <Grid flexDirection="column" display="flex" gap={1}>
        {isEditingName ? (
          <BasicTextInput
            autoFocus
            required
            value={draft.name}
            helperText={
              invalidName(draft.name)
                ? t('helper-text.recipient-list-name')
                : null
            }
            onChange={e => onUpdate({ name: e.target.value })}
            label={t('label.name')}
            InputLabelProps={{ shrink: true }}
          />
        ) : (
          <Typography
            sx={{
              fontSize: '18px',
              fontWeight: 'bold',
              color: 'gray.dark',
            }}
            onClick={editNameToggleOn}
          >
            {draft?.name}
          </Typography>
        )}

        <BufferedTextArea
          value={draft.description}
          onChange={e => onUpdate({ description: e.target.value })}
          label={t('label.description')}
          InputProps={{ sx: { backgroundColor: 'background.menu' } }}
          InputLabelProps={{ shrink: true }}
          rows={2}
        />
        <BufferedTextArea
          value={draft.query}
          onChange={e => onUpdate({ query: e.target.value })}
          label={t('label.query')}
          InputProps={{ sx: { backgroundColor: 'background.menu' } }}
          InputLabelProps={{ shrink: true }}
          helperText={t('helper-text.recipient-sql-query')}
        />
        <Typography
          component={'span'}
          sx={{ fontWeight: 'bold', color: 'gray.dark' }}
        >
          {t('label.parameters')}
        </Typography>
        {TeraUtils.extractParams(draft.query).length === 0 ? (
          <Typography component={'span'} sx={{ color: 'gray.light' }}>
            {t('message.no-parameters')}
          </Typography>
        ) : (
          <Table>
            {TeraUtils.extractParams(draft.query).map(param => {
              return (
                <TableRow key={`${param}-${draft.id}`}>
                  <TableCell>
                    <Typography component={'span'} sx={{ color: 'gray.dark' }}>
                      {param}
                    </Typography>
                  </TableCell>
                  <TableCell>
                    <BufferedTextInput
                      value={queryParams[param ?? '']}
                      onChange={e =>
                        onUpdateQueryParams(param ?? '', e.target.value)
                      }
                    />
                  </TableCell>
                </TableRow>
              );
            })}
          </Table>
        )}
        <ul></ul>
        <LoadingButton
          variant="outlined"
          isLoading={recipientsLoading}
          startIcon={<ZapIcon />}
          onClick={() => {
            queryRecipients(
              draft.query,
              TeraUtils.keyedParamsAsTeraJson(queryParams)
            );
          }}
        >
          {t('label.test-sql-query')}
        </LoadingButton>
        <LoadingButton
          startIcon={<SaveIcon />}
          onClick={() => {
            onSave(draft).catch(err => {
              console.error(err);
              error(err)();
            });
          }}
          disabled={invalidName(draft.name)}
          isLoading={createIsLoading || updateIsLoading}
        >
          {t('button.save')}
        </LoadingButton>
      </Grid>
    </Box>
  );
};
