import React, { useState } from 'react';
import {
  BasicTextInput,
  Box,
  BufferedTextArea,
  DateTimeInput,
  Select,
  Typography,
  useQueryParamsState,
  useTranslation,
} from '@notify-frontend/common';
import { renderOneOff } from 'tera-web';
import { ScheduledNotification } from '../../types';
import { SqlQuerySelector } from '../../components';
import { useNotificationQueries } from 'packages/system/src/Queries/api';

type ScheduledNotificationEditFormProps = {
  onUpdate: (patch: Partial<ScheduledNotification>) => void;
  draft: ScheduledNotification;
};

const FormRow = ({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) => (
  <Box padding={1}>
    <Typography sx={{ fontWeight: 700, fontSize: '13px', marginBottom: '2px' }}>
      {title}
    </Typography>
    <Box paddingLeft={1}>{children}</Box>
  </Box>
);

export const ScheduledNotificationEditForm = ({
  onUpdate,
  draft,
}: ScheduledNotificationEditFormProps) => {
  const t = useTranslation('system');
  const [templateError, setTemplateError] = useState<string | null>(null);

  const { queryParams } = useQueryParamsState();
  queryParams.first = 1000; // Set a high limit to ensure all queries are fetched, if we end up with over 1000 queries we'll need a new solution, or if this is too slow...

  const { data, isLoading } = useNotificationQueries(queryParams);
  const queries = data?.nodes ?? [];

  const isTemplateError = (err: string) => {
    return err.startsWith('Template');
  };
  const isParamsError = (err: string) => {
    return err.startsWith('Parameter');
  };

  const validateTemplate = (template: string) => {
    try {
      renderOneOff(template, JSON.stringify({}));
    } catch (e) {
      if (typeof e === 'string') {
        if (isTemplateError(e)) {
          setTemplateError(e);
          return;
        }
        if (isParamsError(e)) {
          // Missing params are not an error when validating the template
          setTemplateError(null);
          return;
        }
      }
      setTemplateError('Unknown error :' + e);
      return;
    }
    setTemplateError(null);
  };

  return (
    <Box paddingTop={1} width={'100%'}>
      <FormRow title={t('label.details')}>
        <BasicTextInput
          autoFocus
          value={draft.subjectTemplate}
          required
          // TODO: need to validate this one too?
          onChange={e => {
            onUpdate({
              subjectTemplate: e.target
                .value as ScheduledNotification['subjectTemplate'],
            });
          }}
          label={t('label.subject-template')}
          InputLabelProps={{ shrink: true }}
        />
      </FormRow>
      <FormRow title="">
        <BufferedTextArea
          helperText={templateError}
          error={!!templateError}
          value={draft.bodyTemplate}
          onChange={e => {
            validateTemplate(e.target.value);
            // TODO: only update if valid?
            onUpdate({ bodyTemplate: e.target.value });
          }}
          label={t('label.body-template')}
          InputProps={{
            sx: {
              backgroundColor: 'background.menu',
              textarea: {
                resize: 'vertical',
                overflow: 'scroll',
              },
            },
          }}
          InputLabelProps={{ shrink: true }}
          minRows={3}
          maxRows={10}
        />
      </FormRow>
      <Box padding={1}>
        <Typography sx={{ fontWeight: 700, fontSize: '13px' }}>
          {t('label.queries')}
        </Typography>
        <SqlQuerySelector
          allQueries={queries}
          selectedQueryIds={draft.notificationQueryIds}
          isLoading={isLoading}
          setSelection={props => {
            onUpdate(props as Partial<ScheduledNotification>);
          }}
        />
      </Box>
      <FormRow title={t('label.schedule')}>
        <Typography sx={{ fontSize: '10px' }}>Starting From</Typography>
        <DateTimeInput
          onChange={d =>
            onUpdate({
              scheduleStartTime:
                d as ScheduledNotification['scheduleStartTime'],
            })
          }
          date={draft.scheduleStartTime}
        />
        <Typography sx={{ fontSize: '10px', paddingTop: 1 }}>Repeat</Typography>
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
      </FormRow>
    </Box>
  );
};
