import React, { useState } from 'react';
import {
  useDialog,
  DialogButton,
  useTranslation,
  LoadingButton,
  ArrowRightIcon,
  Select,
} from '@notify-frontend/common';

interface SelectNotificationTypeModalProps {
  isOpen: boolean;
  entity: NotificationConfigType | null;
  submit: (type: NotificationConfigType) => void;
  onClose: () => void;
}

// TODO: this will come from backend query
export enum NotificationConfigType {
  ColdChain = 'ColdChain',
}

export const SelectNotificationConfigModal = ({
  isOpen,
  entity,
  onClose,
  submit,
}: SelectNotificationTypeModalProps) => {
  const t = useTranslation(['system']);
  const [type, setType] = useState<NotificationConfigType | ''>(entity ?? '');

  const { Modal } = useDialog({ isOpen, onClose });

  const isInvalid = !type;

  return (
    <>
      <Modal
        okButton={
          <LoadingButton
            disabled={isInvalid}
            onClick={() => {
              if (type) {
                submit(type);
                onClose();
              }
            }}
            isLoading={false}
            startIcon={<ArrowRightIcon />}
            variant="contained"
          >
            {t('button.next')}
          </LoadingButton>
        }
        cancelButton={<DialogButton variant="cancel" onClick={onClose} />}
        title={t('label.setup-notification', { type: '' })}
      >
        <Select
          fullWidth
          label={t('label.select-notification-type')}
          value={type}
          onChange={e => setType(e.target.value as NotificationConfigType)}
          options={[
            // TODO: these options come from backend
            { label: 'Cold Chain', value: NotificationConfigType.ColdChain },
          ]}
          InputLabelProps={{ shrink: true }}
        />
      </Modal>
    </>
  );
};
