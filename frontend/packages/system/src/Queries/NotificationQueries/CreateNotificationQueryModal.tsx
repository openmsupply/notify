import React, { useState } from 'react';
import {
  useDialog,
  DialogButton,
  useTranslation,
  LoadingButton,
  ArrowRightIcon,
  FnUtils,
  useNavigate,
  BasicTextInput,
  Box,
  RouteBuilder,
} from '@notify-frontend/common';
import { useCreateNotificationQuery } from '../api';
import { AppRoute } from 'packages/config/src';

interface SelectNotificationTypeModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const CreateNotificationQueryModal = ({
  isOpen,
  onClose,
}: SelectNotificationTypeModalProps) => {
  const t = useTranslation('system');
  const navigate = useNavigate();

  const { mutateAsync: create, isLoading } = useCreateNotificationQuery();

  const [name, setName] = useState<string>('');

  const { Modal } = useDialog({ isOpen, onClose });

  const isInvalid = !name;

  return (
    <Modal
      okButton={
        <LoadingButton
          disabled={isInvalid}
          onClick={() => {
            const id = FnUtils.generateUUID();
            create({
              input: {
                id: id,
                name: name,
              },
            }).then(() => {
              navigate(
                RouteBuilder.create(AppRoute.Queries).addPart(id).build()
              );
            });
          }}
          isLoading={isLoading}
          startIcon={<ArrowRightIcon />}
          sx={{ marginLeft: 1 }}
        >
          {t('button.create')}
        </LoadingButton>
      }
      cancelButton={<DialogButton variant="cancel" onClick={onClose} />}
      title={t('label.new-query', { type: '' })}
    >
      <Box>
        <BasicTextInput
          fullWidth
          value={name}
          required
          onChange={e => setName(e.target.value)}
          label={t('label.name')}
          InputLabelProps={{ shrink: true }}
        />
      </Box>
    </Modal>
  );
};
