import React, { useState } from 'react';
import {
  useDialog,
  DialogButton,
  useTranslation,
  LoadingButton,
  ArrowRightIcon,
  Select,
  ConfigKind,
} from '@notify-frontend/common';

interface SelectNotificationTypeModalProps {
  isOpen: boolean;
  submit: (kind: ConfigKind) => void;
  onClose: () => void;
}

export const SelectConfigKindModal = ({
  isOpen,
  onClose,
  submit,
}: SelectNotificationTypeModalProps) => {
  const t = useTranslation(['system']);
  const [kind, setKind] = useState<ConfigKind | ''>('');

  const { Modal } = useDialog({ isOpen, onClose });

  const isInvalid = !kind;

  return (
    <>
      <Modal
        okButton={
          <LoadingButton
            disabled={isInvalid}
            onClick={() => {
              if (kind) {
                submit(kind);
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
          value={kind}
          onChange={e => setKind(e.target.value as ConfigKind)}
          options={[
            {
              label: t(`config-kind.${ConfigKind.ColdChain}`),
              value: ConfigKind.ColdChain,
            },
            {
              label: t(`config-kind.${ConfigKind.Scheduled}`),
              value: ConfigKind.Scheduled,
            },
          ]}
          InputLabelProps={{ shrink: true }}
        />
      </Modal>
    </>
  );
};
