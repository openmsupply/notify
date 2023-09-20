import React from 'react';
import {
  BasicTextInput,
  Box,
  BufferedTextArea,
  DateTimeInput,
  Select,
  Typography,
  useTranslation,
} from '@notify-frontend/common';
import { ScheduledNotification } from '../../types';
import { SqlQuerySelector } from '../../components';

const dummySqlQueries = [
  {
    id: '1',
    name: 'Last Sync By Province',
    query: `SELECT 
  category3 AS project, category1_level2 AS province,
  COUNT(last_sync_date) AS num_of_stores_synced_once 
FROM store_categories sc 
JOIN store s ON sc.code = s.code 
JOIN (
  SELECT site_id, MAX(date) AS last_sync_date FROM site_log GROUP BY site_id
) sl ON s.sync_id_remote_site = sl.site_id
WHERE mode IN ('store', 'dispensary')
AND sc.disabled = false
AND category3 IN ({{project}})
AND category1_level2 IN ({{province}}) 
GROUP BY category3, category1_level2`,
    parameters: ['project', 'province'],
  },
  {
    id: '2',
    name: 'First Stock Take',
    query: `SELECT 
  category3 AS project, category1_level2 AS province,
  COUNT(first_stock_take_date) AS first_stock_take_date 
FROM store_categories sc 
JOIN store s ON sc.code = s.code 
JOIN (
  SELECT store_id, MAX(stock_take_created_date) AS fist_stock_take_date FROM stock_take GROUP BY store_id
) st ON s.id = st.store_id
WHERE mode IN ('store', 'dispensary')
AND sc.disabled = false
AND category3 IN ({{project}})
AND category1_level2 IN ({{province}}) 
GROUP BY category3, category1_level2 `,
    parameters: ['project', 'province'],
  },
];

type ScheduledNotificationEditFormProps = {
  onUpdate: (patch: Partial<ScheduledNotification>) => void;
  draft: ScheduledNotification;
};

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

      <BufferedTextArea
        value={draft.parameters}
        onChange={e => onUpdate({ parameters: e.target.value })}
        label={t('label.parameters')}
        InputProps={{ sx: { backgroundColor: 'background.menu' } }}
        InputLabelProps={{ shrink: true }}
      />
      <Typography sx={{ fontWeight: 700, fontSize: '13px' }}>
        Queries
      </Typography>
      <SqlQuerySelector records={dummySqlQueries} />

      <Typography
        sx={{ fontWeight: 700, fontSize: '13px', marginBottom: '2px' }}
      >
        Schedule
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

      <Box>
        <Typography
          sx={{ fontWeight: 700, fontSize: '13px', marginBottom: '10px' }}
        >
          Recipients
        </Typography>
      </Box>
    </>
  );
};
