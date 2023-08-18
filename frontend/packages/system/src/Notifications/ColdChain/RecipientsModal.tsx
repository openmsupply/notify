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
import { useRecipientLists, useRecipients } from '../../Recipients/api';

interface RecipientsModalProps {
  isOpen: boolean;
  onClose: () => void;
  selectedIds: string[];
  setSelectedIds: (ids: string[]) => void;
}

enum recipTypes {
  Telegram = 'telegram',
  Email = 'email',
  Heading = 'heading',
  List = 'list',
}

interface RecipientOption {
  id: string;
  name: string;
  detail: string;
  type: recipTypes;
}

export const RecipientsModal: FC<RecipientsModalProps> = ({
  isOpen,
  selectedIds,
  onClose,
  setSelectedIds,
}) => {
  const t = useTranslation('system');
  const [errorMessage, setErrorMessage] = useState('');
  const [selectedRecipients, setSelectedRecipients] = useState<string[]>([]);

  console.log(selectedRecipients);

  const { Modal } = useDialog({ isOpen, onClose });

  const { data: recipients, isLoading: recipientIsLoading } = useRecipients();
  const { data: recipientLists, isLoading: recipientsListsIsLoading } =
    useRecipientLists();

  const options: RecipientOption[] = useMemo(
    () => [
      {
        id: 'recipientLists-heading',
        name: '--- Recipient Lists ---',
        detail: '',
        type: recipTypes.Heading,
      },
      ...(recipientLists?.nodes ?? []).map(r => ({
        id: r.id,
        name: r.name,
        detail: r.description,
        type: recipTypes.List,
      })),
      {
        id: 'recipients-heading',
        name: '--- Recipients ---',
        detail: '',
        type: recipTypes.Heading,
      },
      ...(recipients?.nodes ?? []).map(r => ({
        id: r.id,
        name: r.name,
        detail: r.toAddress,
        type:
          r.notificationType === NotificationTypeNode.Telegram
            ? recipTypes.Telegram
            : recipTypes.Email,
      })),
    ],
    [recipients, recipientLists]
  );

  const onChangeSelectedRecipients = (ids: string[]) => {
    setSelectedRecipients(ids);
  };

  const renderOption: AutocompleteOptionRenderer<RecipientOption> = (
    props,
    option,
    { selected }
  ): JSX.Element => (
    <li {...props}>
      {option.type !== recipTypes.Heading && (
        <Checkbox
          checked={
            selected // || recipientList.recipients.some(r => r.id === option.id)
          }
        />
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
            // maxWidth: 300,
          }}
        >
          {option.type === recipTypes.Telegram ? 'Telegram' : option.detail}
        </span>
      </Tooltip>
    </li>
  );

  const modalHeight = Math.min(window.innerHeight - 100, 700);
  const modalWidth = Math.min(window.innerWidth - 100, 924);

  return (
    <Modal
      height={modalHeight}
      width={modalWidth}
      okButton={
        <Tooltip title={t('label.add-to-list')}>
          <span>
            <LoadingButton
              disabled={!selectedRecipients.length}
              onClick={() => {
                setSelectedIds(selectedRecipients);
                onClose();
              }}
              isLoading={recipientIsLoading || recipientsListsIsLoading}
              startIcon={<CheckIcon />}
            >
              {t('button.ok')}
            </LoadingButton>
          </span>
        </Tooltip>
      }
      cancelButton={<DialogButton variant="cancel" onClick={onClose} />}
      title={t('label.add-members')}
      slideAnimation={false}
    >
      {recipientIsLoading ? (
        <></>
      ) : (
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
              getOptionDisabled={o => o.type === recipTypes.Heading}
              defaultSelection={options.filter(o => selectedIds.includes(o.id))}
            />
          </Grid>
        </Grid>
      )}
    </Modal>
  );
};
