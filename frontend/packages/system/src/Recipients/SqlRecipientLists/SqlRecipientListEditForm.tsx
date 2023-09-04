import React, { useState } from 'react';
import {
  BasicTextInput,
  Box,
  BufferedTextArea,
  FnUtils,
  Grid,
  LoadingButton,
  SaveIcon,
  Typography,
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
  parameters: '[]',
  ...seed,
});

export const invalidName = (name: string) => {
  const nameIncorrectLength = name.length < 3 || name.length > 75;
  const nameContainsIllegalChars = name.match(/[^ 0-9A-Za-z_\-@.+:/()]/);

  return !name.trim() || !!nameContainsIllegalChars || nameIncorrectLength;
};

type SqlRecipientListEditFormProps = {
  list: SqlRecipientListRowFragment | undefined;
};

export const SqlRecipientListEditForm = ({
  list,
}: SqlRecipientListEditFormProps) => {
  const t = useTranslation('system');

  const [draft, setDraft] = useState(createSqlRecipientList(list));

  const onUpdate = (patch: Partial<DraftSqlRecipientList>) => {
    setDraft({ ...draft, ...patch });
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
  };

  return (
    <Box>
      <Typography
        sx={{
          fontSize: '18px',
          fontWeight: 'bold',
          color: 'gray.dark',
        }}
      >
        {draft?.name}
      </Typography>
      <Grid flexDirection="row" display="flex" gap={3}>
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
        <BufferedTextArea
          value={draft.description}
          onChange={e => onUpdate({ description: e.target.value })}
          label={t('label.description')}
          InputProps={{ sx: { backgroundColor: 'background.menu' } }}
          InputLabelProps={{ shrink: true }}
        />
      </Grid>
      <Grid flexDirection={'row'} display="flex" gap={2}>
        <Grid flexDirection={'column'} display="flex" gap={2}>
          <BufferedTextArea
            value={draft.query}
            onChange={e => onUpdate({ query: e.target.value })}
            label={t('label.query')}
            InputProps={{ sx: { backgroundColor: 'background.menu' } }}
            InputLabelProps={{ shrink: true }}
            helperText={t('helper-text.recipient-sql-query')}
          />
        </Grid>
        <Grid flexDirection={'column-reverse'} display="flex" gap={2}>
          <LoadingButton
            startIcon={<SaveIcon />}
            onClick={() => {
              onSave(draft)
                .catch(err => {
                  // TODO: Better Error!
                  console.error(err);
                })
                .then(() => {
                  // TODO: actually refresh...
                  console.log('Refresh the SQL recipients!');
                });
            }}
            disabled={invalidName(draft.name)}
            isLoading={createIsLoading || updateIsLoading}
          >
            {t('button.save')}
          </LoadingButton>
          <BufferedTextArea
            value={draft.parameters}
            onChange={e => onUpdate({ parameters: e.target.value })}
            label={t('label.parameters')}
            InputProps={{ sx: { backgroundColor: 'background.menu' } }}
            InputLabelProps={{ shrink: true }}
            helperText={t('helper-text.sql-query-parameters')}
          />
        </Grid>
      </Grid>
    </Box>
  );
};
