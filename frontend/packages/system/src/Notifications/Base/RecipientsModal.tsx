import React, { FC, useMemo, useState } from 'react';
import Alert from '@mui/material/Alert';
import AlertTitle from '@mui/material/AlertTitle';
import { useDialog } from '@common/hooks';
import { useTranslation } from '@common/intl';

import {
  AutocompleteMultiList,
  AutocompleteOptionRenderer,
  Checkbox,
  DialogButton,
  LoadingButton,
  Tooltip,
} from '@common/components';
import { CheckIcon } from '@common/icons';
import { Grid, NotificationTypeNode } from 'packages/common/src';
import { RecipientRowFragment } from '../../Recipients/api';
import { RecipientListRowFragment } from '../../Recipients/api/operations.generated';

interface RecipientsModalProps {
  isOpen: boolean;
  recipients: RecipientRowFragment[];
  recipientLists: RecipientListRowFragment[];
  initialSelectedIds: string[];
  onClose: () => void;
  setSelection: (input: {
    recipients: string[];
    recipientLists: string[];
  }) => void;
}

enum RecipientOptionType {
  Telegram = 'telegram',
  Email = 'email',
  Heading = 'heading',
  List = 'list',
}

interface RecipientOption {
  id: string;
  name: string;
  detail: string;
  type: RecipientOptionType;
}

export const RecipientsModal: FC<RecipientsModalProps> = ({
  recipientLists,
  recipients,
  isOpen,
  initialSelectedIds,
  onClose,
  setSelection,
}) => {
  const t = useTranslation(['system', 'host']);
  const [errorMessage, setErrorMessage] = useState('');
  const [selectedIds, setSelectedIds] = useState<string[]>([]);

  const { Modal } = useDialog({ isOpen, onClose });

  const options: RecipientOption[] = useMemo(
    () => [
      {
        id: 'recipientLists-heading',
        name: `--- ${t('recipient-lists', { ns: 'host' })} ---`,
        detail: '',
        type: RecipientOptionType.Heading,
      },
      ...recipientLists.map(r => ({
        id: r.id,
        name: r.name,
        detail: r.description,
        type: RecipientOptionType.List,
      })),
      {
        id: 'recipients-heading',
        name: `--- ${t('recipients', { ns: 'host' })} ---`,
        detail: '',
        type: RecipientOptionType.Heading,
      },
      ...recipients.map(r => ({
        id: r.id,
        name: r.name,
        detail: r.toAddress,
        type:
          r.notificationType === NotificationTypeNode.Telegram
            ? RecipientOptionType.Telegram
            : RecipientOptionType.Email,
      })),
    ],
    [recipients, recipientLists]
  );

  const onChangeSelectedRecipients = (ids: string[]) => {
    setSelectedIds(ids);
  };

  const submitSelection = () => {
    setSelection({
      recipients: selectedIds.filter(id => recipients.some(r => r.id === id)),
      recipientLists: selectedIds.filter(id =>
        recipientLists.some(r => r.id === id)
      ),
    });
    onClose();
  };

  const modalHeight = Math.min(window.innerHeight - 100, 700);
  const modalWidth = Math.min(window.innerWidth - 100, 924);

  return (
    <Modal
      height={modalHeight}
      width={modalWidth}
      okButton={
        <Tooltip title={t('label.select-recipients')}>
          <span>
            <LoadingButton
              disabled={!selectedIds.length}
              onClick={submitSelection}
              isLoading={false}
              startIcon={<CheckIcon />}
            >
              {t('label.select-recipients')}
            </LoadingButton>
          </span>
        </Tooltip>
      }
      cancelButton={<DialogButton variant="cancel" onClick={onClose} />}
      title={t('label.select-recipients')}
      slideAnimation={false}
    >
      <Grid
        flexDirection="column"
        display="flex"
        justifyContent="center"
        gap={2}
      >
        {errorMessage ? (
          <Grid item>
            <Alert
              severity="error"
              onClose={() => {
                setErrorMessage('');
              }}
            >
              <AlertTitle>{t('error')}</AlertTitle>
              {errorMessage}
            </Alert>
          </Grid>
        ) : null}
        <Grid item>
          <AutocompleteMultiList
            options={options}
            onChange={onChangeSelectedRecipients}
            getOptionLabel={option => `${option.detail} ${option.name}`}
            renderOption={renderOption}
            filterProperties={['name', 'detail']}
            filterPlaceholder={t('placeholder.search')}
            width={modalWidth - 50}
            height={modalHeight - 300}
            getOptionDisabled={o => o.type === RecipientOptionType.Heading}
            defaultSelection={options.filter(o =>
              initialSelectedIds.includes(o.id)
            )}
          />
        </Grid>
      </Grid>
    </Modal>
  );
};

const renderOption: AutocompleteOptionRenderer<RecipientOption> = (
  props,
  option,
  { selected }
): JSX.Element => (
  <li {...props}>
    {option.type !== RecipientOptionType.Heading && (
      <Checkbox checked={selected} />
    )}
    <Tooltip title={option.name}>
      <span
        style={{
          fontWeight: 700,
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          width: 250,
          minWidth: 250,
          marginRight: 10,
        }}
      >
        {option.name}
      </span>
    </Tooltip>
    <Tooltip title={option.detail}>
      <span
        style={{
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
        }}
      >
        {option.type === RecipientOptionType.Telegram
          ? 'Telegram'
          : option.detail}
      </span>
    </Tooltip>
  </li>
);
