import React, { FC, useState } from 'react';
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
  // recipientList: RecipientListRowFragment;
  isOpen: boolean;
  onClose: () => void;
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
  // recipientList,
  isOpen,
  onClose,
}) => {
  const t = useTranslation('system');
  const [errorMessage, setErrorMessage] = useState('');
  const [selectedRecipients, setSelectedRecipients] = useState<string[]>([]);

  const { Modal } = useDialog({ isOpen, onClose });

  const { data: recipients, isLoading: recipientIsLoading } = useRecipients();
  const { data: recipientLists, isLoading: recipientsListsIsLoading } =
    useRecipientLists();

  const options: RecipientOption[] = [
    {
      id: 'recipientLists title',
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
      id: 'recipients title',
      name: '--- Recipients ---',
      detail: '',
      type: recipTypes.Heading,
    },
    ...(recipients?.nodes ?? []).map(r => ({
      id: r.id,
      name: r.name,
      detail:
        r.notificationType === NotificationTypeNode.Telegram
          ? 'Telegram'
          : r.toAddress,
      type:
        r.notificationType === NotificationTypeNode.Telegram
          ? recipTypes.Telegram
          : recipTypes.Email,
    })),
  ];

  const onChangeSelectedRecipients = (ids: string[]) => {
    // const filteredIds = ids.filter(
    //   id => !recipientList.recipients.some(r => r.id === id)
    // );
    // setSelectedRecipients(filteredIds);
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
            marginRight: 10,
          }}
        >
          {option.name}
        </span>
      </Tooltip>
      <span
        style={{
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
        }}
      >
        {option.detail}
      </span>
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
              onClick={() => onClose()}
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
            />
          </Grid>
        </Grid>
      )}
    </Modal>
  );
};
