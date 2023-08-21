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
  // TODO: this will be backend NotificationConfig type?
  entity: {
    id: string;
    title: string;
    configType: NotificationConfigType;
  } | null;
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
          // TODO: is there a better way than `as any` given this is generic?
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          entity={configEntity as any}
          onClose={() => {
            onClose();
            configOnClose();
          }}
        />
      )}
    </>
  );
};
