import React, { FC } from 'react';
import Chip from '@mui/material/Chip';

import { EventStatus } from '@common/types';
import { Tooltip } from '@common/ui';

function getStatusColour(status: EventStatus) {
  switch (status) {
    case EventStatus.Errored:
      return 'error';
    case EventStatus.Failed:
      return 'error';
    case EventStatus.Queued:
      return 'info';
    case EventStatus.Sent:
      return 'success';
  }

  return 'info';
}

function getToolTip(status: EventStatus) {
  switch (status) {
    case EventStatus.Errored:
      return 'There was an error sending this notification, but it will be re-tried';
    case EventStatus.Failed:
      return 'This notification failed to send, it will not be re-tried';
    case EventStatus.Queued:
      return 'This notification is queued to be sent, please refresh the page to see the latest status';
    case EventStatus.Sent:
      return 'Sent successfully, if the user has not received the notification, please check the address is correct, and their spam folder';
  }

  return status;
}

export interface NotificationStatusChipProps {
  status: EventStatus;
}

export const NotificationStatusChip: FC<NotificationStatusChipProps> = ({
  status,
}) => {
  const statusColour = getStatusColour(status);
  const toolTip = getToolTip(status);
  const chipLabel = status;

  return (
    <Tooltip title={toolTip}>
      <Chip
        label={chipLabel.toLocaleUpperCase()}
        variant="filled"
        color={statusColour}
        size="small"
      />
    </Tooltip>
  );
};
