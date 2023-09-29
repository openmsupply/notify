import React from 'react';
import {
  BasicTextInput,
  BufferedTextArea,
  DateTimeInput,
  Select,
  Typography,
  useQueryParamsState,
  useTranslation,
} from '@notify-frontend/common';
import { ScheduledNotification } from '../../types';
import { SqlQuerySelector } from '../../components';
import { useNotificationQueries } from 'packages/system/src/Queries/api';

type ScheduledNotificationEditFormProps = {
  onUpdate: (patch: Partial<ScheduledNotification>) => void;
  draft: ScheduledNotification;
};

export const ScheduledNotificationEditForm = ({
  onUpdate,
  draft,
}: ScheduledNotificationEditFormProps) => {
  const t = useTranslation('system');

  const { queryParams } = useQueryParamsState();

  const { data, isLoading } = useNotificationQueries(queryParams);
  const queries = data?.nodes ?? [];

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

      <BufferedTextArea
        value={draft.parameters}
        onChange={e => onUpdate({ parameters: e.target.value })}
        label={t('label.parameters')}
        InputProps={{ sx: { backgroundColor: 'background.menu' } }}
        InputLabelProps={{ shrink: true }}
      />
      <Typography sx={{ fontWeight: 700, fontSize: '13px' }}>
        {t('label.queries')}
      </Typography>
      <SqlQuerySelector
        allQueries={queries}
        selectedQueryIds={draft.notificationQueryIds}
        isLoading={isLoading}
        setSelection={props => {
          console.log('props', props);
          onUpdate(props as Partial<ScheduledNotification>);
        }}
      />

      <Typography
        sx={{ fontWeight: 700, fontSize: '13px', marginBottom: '2px' }}
      >
        {t('label.schedule')}
      </Typography>
      <Typography sx={{ fontSize: '10px' }}>Starting From</Typography>
      <DateTimeInput
        onChange={d =>
          onUpdate({
            scheduleStartTime: d as ScheduledNotification['scheduleStartTime'],
          })
        }
        date={draft.scheduleStartTime}
      />
      <Typography sx={{ fontSize: '10px' }}>Repeat</Typography>
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
    </>
  );
};
