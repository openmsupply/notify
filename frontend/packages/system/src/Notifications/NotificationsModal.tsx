import React, { useEffect, useState } from 'react';
import { useEditModal } from '@notify-frontend/common';
import { CCNotificationEditModal } from './ColdChain/CCNotificationEditModal';
import {
  NotificationConfigType,
  SelectNotificationConfigModal,
} from './SelectNotificationConfigModal';

interface NotificationsModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const NotificationsModal = ({
  isOpen,
  onClose,
}: NotificationsModalProps) => {
  const [type, setType] = useState<NotificationConfigType | ''>('');

  useEffect(() => {
    if (isOpen) selectOnOpen();
  }, [isOpen]);

  const {
    isOpen: selectIsOpen,
    onClose: selectOnClose,
    onOpen: selectOnOpen,
    entity: selectEntity,
  } = useEditModal<NotificationConfigType>();

  const {
    isOpen: configIsOpen,
    mode: configMode,
    onClose: configOnClose,
    onOpen: configOnOpen,
  } = useEditModal();

  const ConfigModal = (() => {
    switch (type) {
      case NotificationConfigType.ColdChain:
        return CCNotificationEditModal;
      default:
        return () => <></>;
    }
  })();

  return (
    <>
      {selectIsOpen && (
        <SelectNotificationConfigModal
          isOpen={selectIsOpen}
          onClose={() => {
            onClose();
            selectOnClose();
          }}
          entity={selectEntity}
          submit={t => {
            setType(t);
            configOnOpen();
          }}
        />
      )}
      {configIsOpen && (
        <ConfigModal
          mode={configMode}
          isOpen={configIsOpen}
          onClose={() => {
            onClose();
            configOnClose();
          }}
        />
      )}
    </>
  );
};
