import React, { FC, useState } from 'react';
import {
  useDialog,
  useTranslation,
  LoadingButton,
  DialogButton,
  SaveIcon,
  BasicTextInput,
  IconButton,
  CheckIcon,
} from '@notify-frontend/common';
import { UserAccountRowFragment } from '../../api';

interface UserAccountDisableModalProps {
  isOpen: boolean;
  onClose: () => void;
  userAccount: UserAccountRowFragment | null;
}

export const UserAccountDisableModal: FC<UserAccountDisableModalProps> = ({
  isOpen,
  onClose,
  userAccount,
}) => {
  const [errorMessage, setErrorMessage] = useState('');
  const { Modal } = useDialog({ isOpen, onClose });
  const t = useTranslation(['system']);

  return (
    <Modal
      okButton={
        <LoadingButton
          onClick={() => {
            console.log('ok');
          }}
          startIcon={<CheckIcon />}
          variant="contained"
          isLoading={false}
        >
          {t('button.ok')}
        </LoadingButton>
      }
      cancelButton={<DialogButton variant="cancel" onClick={onClose} />}
      // title={t('label.are-you-sure-you-want-to-disable-this-user-account', {
      //   ns: 'system',
      // })}
    >
      <a href="https://github.com/openmsupply/health-supply-hub/issues/388">
        Todo: set up soft deactivation of user accounts:
      </a>
    </Modal>
  );
};
