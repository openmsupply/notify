import React, { useState } from 'react';
import {
  BasicTextInput,
  Box,
  BufferedTextArea,
  FnUtils,
  Grid,
  LoadingButton,
  SaveIcon,
  TeraUtils,
  Typography,
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
};

export const SqlRecipientListEditForm = ({
  list,
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
        {TeraUtils.extractParams(draft.query).length === 0 && (
          <Typography component={'span'} sx={{ color: 'gray.light' }}>
            {t('message.no-parameters')}
          </Typography>
        )}
        <ul>
          {TeraUtils.extractParams(draft.query).map(param => {
            return (
              <li key={`${param}-${draft.id}`}>
                <Typography component={'span'} sx={{ color: 'gray.dark' }}>
                  {param}
                </Typography>
              </li>
            );
          })}
        </ul>
        <LoadingButton
          startIcon={<SaveIcon />}
          onClick={() => {
            onSave(draft)
              .catch(err => {
                console.error(err);
                error(err)();
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
      </Grid>
    </Box>
  );
};
