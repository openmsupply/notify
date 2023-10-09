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
  isValidVariableName,
  validateVariableNameHelperText,
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
  const [referenceName, setReferenceName] = useState<string>('');
  const [referenceNameEdited, setReferenceNameEdited] = useState(false);

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
                referenceName: referenceName,
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
          onChange={e => {
            setName(e.target.value);
            if (!referenceNameEdited) {
              const referenceName = e.target.value
                .replace(/[^a-zA-Z0-9]/g, '_')
                .toLocaleLowerCase();
              setReferenceName(referenceName);
            }
          }}
          label={t('label.name')}
          InputLabelProps={{ shrink: true }}
        />
        <BasicTextInput
          fullWidth
          value={referenceName}
          error={!isValidVariableName(referenceName)}
          helperText={validateVariableNameHelperText(referenceName, t)}
          required
          onChange={e => {
            setReferenceName(e.target.value);
            setReferenceNameEdited(true);
          }}
          label={t('label.reference-name')}
          InputLabelProps={{ shrink: true }}
        />
      </Box>
    </Modal>
  );
};
