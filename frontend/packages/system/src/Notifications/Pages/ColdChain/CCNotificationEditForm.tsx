import React from 'react';
import {
  Box,
  Checkbox,
  Grid,
  PositiveNumberInput,
  Select,
  Typography,
  useTranslation,
  NumberInput,
  InfoTooltipIcon,
} from '@notify-frontend/common';
import {
  CCNotification,
  getReminderUnitsAsOptions,
  getReminderUnitsFromString,
} from '../../types';
import { useColdChainSensors } from '../../api';
import { SensorSelector } from './SensorSelector';

type CCNotificationEditFormProps = {
  onUpdate: (patch: Partial<CCNotification>) => void;
  draft: CCNotification;
};

export const CCNotificationEditForm = ({
  onUpdate,
  draft,
}: CCNotificationEditFormProps) => {
  const t = useTranslation('system');

  const { data: sensors, isLoading: sensorsLoading } = useColdChainSensors();

  return (
    <>
      <Typography
        sx={{
          fontWeight: 700,
          fontSize: '13px',
          marginTop: '10px',
          marginBottom: '10px',
        }}
      >
        {t('heading.cold-chain-alerts')}
      </Typography>
      <ul style={{ listStyleType: 'none', padding: '0' }}>
        <li>
          <Checkbox
            id="highTemp"
            checked={draft.highTemp}
            onClick={() => onUpdate({ highTemp: !draft.highTemp })}
          />

          <label htmlFor="highTemp">
            {t('label.coldchain-high-temp-alerts')}
            {'  '}
          </label>
          <NumberInput
            id={'highTempThreshold'}
            disabled={!draft.highTemp}
            value={draft.highTempThreshold}
            required
            onChange={newValue => onUpdate({ highTempThreshold: newValue })}
            sx={{ width: '60px' }}
          />
          {`  ${t('label.degrees-celsius')}`}
          <InfoTooltipIcon
            title={t('messages.cold-chain-temperature-information')}
          />
        </li>
        <li>
          <Checkbox
            id="lowTemp"
            checked={draft.lowTemp}
            onClick={() => onUpdate({ lowTemp: !draft.lowTemp })}
          />
          <label htmlFor="lowTemp">
            {`${t('label.coldchain-low-temp-alerts')}  `}
          </label>
          <NumberInput
            id={'lowTempThreshold'}
            disabled={!draft.highTemp}
            value={draft.lowTempThreshold}
            required
            onChange={newValue => onUpdate({ lowTempThreshold: newValue })}
            sx={{ width: '60px' }}
          />
          {`  ${t('label.degrees-celsius')}`}
          <InfoTooltipIcon
            title={t('messages.cold-chain-temperature-information')}
          />
        </li>
        <li>
          <Checkbox
            id="confirmOk"
            checked={draft.confirmOk}
            onClick={() => onUpdate({ confirmOk: !draft.confirmOk })}
          />
          <label htmlFor="confirmOk">
            {t('label.coldchain-confirm-ok-alerts')}
          </label>
        </li>
        <li>
          <Checkbox
            id="noData"
            checked={draft.noData}
            onClick={() => onUpdate({ noData: !draft.noData })}
          />
          <label htmlFor="noData">{t('label.coldchain-no-data-alerts')}</label>
          <PositiveNumberInput
            disabled={!draft.noData}
            value={draft.noDataInterval}
            required
            onChange={newValue => onUpdate({ noDataInterval: newValue })}
            sx={{ marginLeft: '10px', width: '60px' }}
          />
          <Select
            value={draft.noDataUnits}
            disabled={!draft.noData}
            onChange={e =>
              onUpdate({
                noDataUnits: getReminderUnitsFromString(e.target.value),
              })
            }
            options={getReminderUnitsAsOptions(t, draft.noDataInterval)}
            sx={{ marginLeft: '10px' }}
          />
          <InfoTooltipIcon
            title={t('messages.cold-chain-no-data-information')}
          />
        </li>
      </ul>
      <Typography
        sx={{
          fontWeight: 700,
          fontSize: '13px',
          marginTop: '10px',
          marginBottom: '10px',
        }}
      >
        {t('heading.preference')}
      </Typography>
      <ul style={{ listStyleType: 'none', padding: '0' }}>
        <li>
          <Checkbox
            id="remind"
            checked={draft.remind}
            onClick={() => onUpdate({ remind: !draft.remind })}
          />
          <label htmlFor="remind">{t('label.coldchain-reminder-alerts')}</label>
          <PositiveNumberInput
            disabled={!draft.remind}
            value={draft.reminderInterval}
            required
            onChange={newValue => onUpdate({ reminderInterval: newValue })}
            sx={{ marginLeft: '10px', width: '60px' }}
          />

          <Select
            value={draft.reminderUnits}
            disabled={!draft.remind}
            onChange={e =>
              onUpdate({
                reminderUnits: getReminderUnitsFromString(e.target.value),
              })
            }
            options={getReminderUnitsAsOptions(t, draft.reminderInterval)}
            sx={{ marginLeft: '10px' }}
          />
        </li>
        <li>
          <Checkbox
            id="messageAlertResolved"
            checked={draft.messageAlertResolved}
            onClick={() =>
              onUpdate({ messageAlertResolved: !draft.messageAlertResolved })
            }
          />
          <label htmlFor="messageAlertResolved">
            {t('label.coldchain-message-alerts-resolved')}
          </label>
        </li>
      </ul>

      <Box>
        <Typography
          sx={{ fontWeight: 700, fontSize: '13px', marginBottom: '10px' }}
        >
          {t('heading.selected-sensors')}
        </Typography>
        <Grid container>
          <SensorSelector
            records={sensors ?? []}
            selectedIds={draft.sensorIds}
            setSelection={props => {
              console.log('props', props);
              onUpdate(props as Partial<CCNotification>);
            }}
            isLoading={sensorsLoading}
          />
        </Grid>
      </Box>
    </>
  );
};
