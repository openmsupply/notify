import React, { useEffect, useState } from 'react';
import { ConfigKind, useEditModal } from '@notify-frontend/common';
import { CCNotificationEditModal } from './ColdChain/CCNotificationEditModal';
import { SelectConfigKindModal } from './SelectConfigKindModal';
import { NotificationConfigRowFragment } from '../api';
import { parseColdChainNotificationConfig } from './ColdChain/parseConfig';

interface NotificationsModalProps {
  isOpen: boolean;
  onClose: () => void;
  entity: NotificationConfigRowFragment | null;
}

export const NotificationsModal = ({
  isOpen,
  entity,
  onClose,
}: NotificationsModalProps) => {
  const [kind, setKind] = useState<ConfigKind | ''>('');

  useEffect(() => {
    if (isOpen) {
      if (entity) {
        setKind(entity.kind);
        configOnOpen(entity);
      } else {
        selectOnOpen();
      }
    }
  }, [isOpen]);

  const {
    isOpen: selectIsOpen,
    onClose: selectOnClose,
    onOpen: selectOnOpen,
  } = useEditModal<ConfigKind>();

  const {
    isOpen: configIsOpen,
    mode: configMode,
    onClose: configOnClose,
    onOpen: configOnOpen,
    entity: configEntity,
  } = useEditModal<NotificationConfigRowFragment>();

  const ConfigModal = () => {
    const props = {
      mode: configMode,
      isOpen: configIsOpen,
      onClose: () => {
        configOnClose();
        onClose();
      },
    };

    switch (kind) {
      case ConfigKind.ColdChain:
        return (
          <CCNotificationEditModal
            {...props}
            entity={
              configEntity
                ? parseColdChainNotificationConfig(configEntity)
                : null
            }
          />
        );
      default:
        return <></>;
    }
  };

  return (
    <>
      {selectIsOpen && (
        <SelectConfigKindModal
          isOpen={selectIsOpen}
          onClose={() => {
            onClose();
            selectOnClose();
          }}
          submit={k => {
            setKind(k);
            configOnOpen();
          }}
        />
      )}
      {configIsOpen && <ConfigModal />}
    </>
  );
};
