import React, { useMemo, useState } from 'react';
import {
  BasicTextInput,
  Box,
  BufferedTextArea,
  EditIcon,
  FnUtils,
  Grid,
  IconButton,
  KeyedParams,
  LoadingButton,
  SaveIcon,
  TeraUtils,
  Typography,
  ZapIcon,
  useDetailPanel,
  useNotification,
  useToggle,
  useTranslation,
  useLocalStorage,
} from '@notify-frontend/common';
import { DraftSqlRecipientList } from './types';
import { useCreateSqlRecipientList, useUpdateSqlRecipientList } from '../api';
import { SqlRecipientListRowFragment } from '../api/operations.generated';
import { SidePanel } from './SidePanel';

const createSqlRecipientList = (
  seed?: DraftSqlRecipientList | null
): DraftSqlRecipientList => ({
  id: FnUtils.generateUUID(),
  name: '',
  description: '',
  query: '',
  parameters: [],
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

export const RecipientQueryEditor = ({
  list,
  queryRecipients,
  recipientsLoading,
}: SqlRecipientListEditFormProps) => {
  const t = useTranslation('system');

  const { error } = useNotification();

  const { open: openSidePanel, isOpen } = useDetailPanel();

  const isNew = list === undefined || list === null || list.name === '';

  const [isSaved, setIsSaved] = useState(!isNew);

  const {
    isOn: isEditingName,
    toggleOn: editNameToggleOn,
    toggleOff: editNameToggleOff,
  } = useToggle(isNew);

  const [draft, setDraft] = useState(createSqlRecipientList(list));
  const hasParams = useMemo(
    () => TeraUtils.extractParams(draft.query).length > 0,
    [draft]
  );

  const onUpdate = (patch: Partial<DraftSqlRecipientList>) => {
    setDraft({ ...draft, ...patch });
    // Update parameters
    if (patch.query) {
      const params = TeraUtils.extractParams(patch.query);
      setDraft({ ...draft, ...patch, parameters: params });
    }
    setIsSaved(false);
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

    if (isNew) await create({ input });
    else await update({ input });
    editNameToggleOff();
    setIsSaved(true);
  };

  const [userQueryParameters, setUserQueryParameters] = useLocalStorage('/query_parameters');

  const allParamsSet = TeraUtils.extractParams(draft.query).every(param => {
    if (param) {
      // This allows the user to set the param to an empty string if they edit the field then delete the value
      return (Object.keys(queryParams).length > 0)? queryParams[param] !== undefined : (userQueryParameters ?? queryParams)[param] !== (undefined || '');
    } else {
      return false;
    }
  });

  let testQueryButton = (
    <LoadingButton
      variant="outlined"
      disabled={recipientsLoading || !draft.query || !allParamsSet}
      isLoading={recipientsLoading}
      startIcon={<ZapIcon />}
      onClick={() => {
        queryRecipients(
          draft.query,
          TeraUtils.keyedParamsAsTeraJson(Object.keys(queryParams).length == 0? (userQueryParameters ?? queryParams) : queryParams)
        );
      }}
    >
      {t('label.test-sql-query')}
    </LoadingButton>
  );
  // If there is a query but we don't have all the parameters yet, replace the test button with an edit params button
  if (draft.query && hasParams && !isOpen && !allParamsSet) {
    testQueryButton = (
      <LoadingButton
        variant="contained"
        disabled={false}
        isLoading={false}
        startIcon={<EditIcon />}
        onClick={openSidePanel}
      >
        {t('label.edit-parameters')}
      </LoadingButton>
    );
  }
  return (
    <Box sx={{ width: '100%' }}>
      <SidePanel
        query={draft.query}
        queryParams={queryParams}
        onUpdateQueryParams={onUpdateQueryParams}
        userQueryParameters={userQueryParameters}
        setUserQueryParameters={setUserQueryParameters}
      />
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
            <IconButton
              onClick={editNameToggleOn}
              icon={<EditIcon />}
              label={t('label.edit')}
            />
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
          minRows={4}
        />
        <Box sx={{ display: 'flex', gap: '8px' }}>
          <Typography
            component={'span'}
            sx={{ fontWeight: 'bold', color: 'gray.dark' }}
          >
            {t('label.parameters')}:
          </Typography>

          {!hasParams ? (
            <Typography component={'span'} sx={{ color: 'gray.light' }}>
              {t('message.no-parameters')}
            </Typography>
          ) : (
            <>
              <Typography component={'span'} sx={{ color: 'gray.dark' }}>
                {TeraUtils.extractParams(draft.query).join(', ')}
              </Typography>
              <IconButton
                height="24px"
                width="24px"
                onClick={openSidePanel}
                icon={<EditIcon />}
                label={t('label.edit')}
              />
            </>
          )}
        </Box>
        <Box>
          <LoadingButton
            startIcon={<SaveIcon />}
            onClick={() => {
              onSave(draft).catch(err => {
                console.error(err);
                error(err)();
              });
            }}
            disabled={isSaved || invalidName(draft.name)}
            isLoading={createIsLoading || updateIsLoading}
            sx={{ marginRight: 1 }}
          >
            {t('button.save')}
          </LoadingButton>
          {testQueryButton}
        </Box>
      </Grid>
    </Box>
  );
};
