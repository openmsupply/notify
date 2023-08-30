import React from 'react';
import {
  BasicTextInput,
  Box,
  BufferedTextArea,
  Select,
  Typography,
  useTranslation,
} from '@notify-frontend/common';
import { ScheduledNotification } from '../../types';

type ScheduledNotificationEditFormProps = {
  onUpdate: (patch: Partial<ScheduledNotification>) => void;
  draft: ScheduledNotification;
};

// const dummyLocations = [
//   { id: 'store-1-location-A', name: 'Store 1, Location A' },
//   { id: 'store-1-location-B', name: 'Store 1, Location B' },
//   { id: 'store-1-location-C', name: 'Store 1, Location C' },
//   { id: 'store-2-location-A', name: 'Store 1, Location A' },
//   { id: 'store-2-location-B', name: 'Store 1, Location B' },
//   { id: 'store-2-location-C', name: 'Store 1, Location C' },
//   { id: 'store-2-location-D', name: 'Store 1, Location D' },
// ];

export const ScheduledNotificationEditForm = ({
  onUpdate,
  draft,
}: ScheduledNotificationEditFormProps) => {
  const t = useTranslation('system');
  return (
    <>
      <BasicTextInput
        autoFocus
        value={draft.subjectTemplate}
        required
        onChange={e =>
          onUpdate({
            subjectTemplate: e.target
              .value as ScheduledNotification['subjectTemplate'],
          })
        }
        label={t('label.subject-template')}
        InputLabelProps={{ shrink: true }}
      />
      <BufferedTextArea
        value={draft.bodyTemplate}
        onChange={e => onUpdate({ bodyTemplate: e.target.value })}
        label={t('label.body-template')}
        InputProps={{ sx: { backgroundColor: 'background.menu' } }}
        InputLabelProps={{ shrink: true }}
      />
      <Select
        value={draft.scheduleFrequency}
        disabled={false}
        onChange={e =>
          onUpdate({
            scheduleFrequency: e.target
              .value as ScheduledNotification['scheduleFrequency'],
          })
        }
        options={[
          { label: t('label.daily'), value: 'daily' },
          { label: t('label.weekly'), value: 'weekly' },
          { label: t('label.monthly'), value: 'monthly' },
        ]}
      />

      <Box>
        <Typography
          sx={{ fontWeight: 700, fontSize: '13px', marginBottom: '10px' }}
        >
          Select Locations
        </Typography>
      </Box>
    </>
  );
};
