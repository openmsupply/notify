import React, { FC, useState } from 'react';
import Alert from '@mui/material/Alert';
import AlertTitle from '@mui/material/AlertTitle';
import { useDialog, useNotification } from '@common/hooks';
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
import { Grid } from 'packages/common/src';
import { RecipientListRowFragment } from '../api/operations.generated';
import { useAddRecipientToList, useRecipients } from '../api';

interface ListMemberAddModalProps {
  recipientList: RecipientListRowFragment;
  isOpen: boolean;
  onClose: () => void;
}

interface RecipientOption {
  id: string;
  name: string;
  toAddress: string;
}

export const ListMemberAddModal: FC<ListMemberAddModalProps> = ({
  recipientList,
  isOpen,
  onClose,
}) => {
  const t = useTranslation('system');
  const { success } = useNotification();
  const [errorMessage, setErrorMessage] = useState('');
  const [addingInProgress, setAddingInProgress] = useState(false);
  const [selectedRecipients, setSelectedRecipients] = useState<string[]>([]);

  const { Modal } = useDialog({ isOpen, onClose });

  const onChangeSelectedRecipients = (ids: string[]) => {
    const filteredIds = ids.filter(
      id => !recipientList.recipients.some(r => r.id === id)
    );
    setSelectedRecipients(filteredIds);
  };

  const renderOption: AutocompleteOptionRenderer<RecipientOption> = (
    props,
    option,
    { selected }
  ): JSX.Element => (
    <li {...props}>
      <Checkbox
        checked={
          selected || recipientList.recipients.some(r => r.id === option.id)
        }
      />
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
        {option.toAddress}
      </span>
    </li>
  );

  const { mutateAsync: addRecipientToList, invalidateQueries } =
    useAddRecipientToList();

  const addAction = async () => {
    const errors: string[] = [];

    if (selectedRecipients.length > 0) {
      setAddingInProgress(true);
      let numberAdded = 0;
      const remainingRecords = [...selectedRecipients];
      while (remainingRecords.length) {
        await Promise.all(
          remainingRecords.splice(0, 10).map(async id => {
            await addRecipientToList({
              input: {
                recipientId: id,
                recipientListId: recipientList.id,
              },
            })
              .catch(err => {
                if (!err) {
                  err = { message: 'Unknown error' };
                }
                errors.push(err.message);
              })
              .then(() => {
                numberAdded++;
              });
          })
        );
      }
      setAddingInProgress(false);
      if (errors.length) {
        setErrorMessage(errors.join(', '));
      } else {
        invalidateQueries();
        const importMessage = t('messages.recipients-added', {
          count: numberAdded,
        });
        const successSnack = success(importMessage);
        successSnack();
        onClose();
      }
    }
  };

  const { data: recipients, isLoading: recipientsLoading } = useRecipients();
  const options = recipients?.nodes ?? [];

  const modalHeight = Math.min(window.innerHeight - 50, 800);
  const modalWidth = Math.min(window.innerWidth - 50, 1024);

  return (
    <Modal
      height={modalHeight}
      width={modalWidth}
      okButton={
        <Tooltip title={t('label.add-to-list')}>
          <span>
            <LoadingButton
              disabled={!selectedRecipients.length}
              onClick={async () => {
                addAction();
              }}
              isLoading={recipientsLoading || addingInProgress}
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
      {recipientsLoading ? (
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
              getOptionLabel={option => `${option.toAddress} ${option.name}`}
              renderOption={renderOption}
              filterProperties={['name', 'toAddress']}
              filterPlaceholder={t('placeholder.search')}
              width={modalWidth - 50}
              height={modalHeight - 300}
              getOptionDisabled={o =>
                recipientList.recipients.some(r => r.id === o.id)
              }
            />
          </Grid>
        </Grid>
      )}
    </Modal>
  );
};
