import React, { useEffect, useState } from 'react';
import { useEditModal } from '@notify-frontend/common';
import { CCNotificationEditModal } from './ColdChain/CCNotificationEditModal';
import { SelectNotificationConfigModal } from './SelectNotificationConfigModal';
import {
  BaseNotificationConfig,
  CCNotification,
  NotificationConfigType,
} from '../types';

interface NotificationsModalProps {
  isOpen: boolean;
  onClose: () => void;
  entity: BaseNotificationConfig | null;
}

export const NotificationsModal = ({
  isOpen,
  entity,
  onClose,
}: NotificationsModalProps) => {
  const [type, setType] = useState<NotificationConfigType | ''>('');

  useEffect(() => {
    if (isOpen) {
      if (entity) {
        setType(entity.configType);
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
  } = useEditModal<NotificationConfigType>();

  const {
    isOpen: configIsOpen,
    mode: configMode,
    onClose: configOnClose,
    onOpen: configOnOpen,
    entity: configEntity,
  } = useEditModal<BaseNotificationConfig>();

  const ConfigModal = () => {
    const props = {
      mode: configMode,
      isOpen: configIsOpen,
      onClose: () => {
        configOnClose();
        onClose();
      },
    };

    switch (type) {
      case NotificationConfigType.ColdChain:
        return (
          <CCNotificationEditModal
            {...props}
            entity={configEntity as CCNotification}
          />
        );
      default:
        return <></>;
    }
  };

  return (
    <>
      {selectIsOpen && (
        <SelectNotificationConfigModal
          isOpen={selectIsOpen}
          onClose={() => {
            onClose();
            selectOnClose();
          }}
          submit={t => {
            setType(t);
            configOnOpen();
          }}
        />
      )}
      {configIsOpen && <ConfigModal />}
    </>
  );
};
