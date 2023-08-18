import React from 'react';
import {
  Box,
  Checkbox,
  PositiveNumberInput,
  Select,
  useTranslation,
} from '@notify-frontend/common';

export interface CCNotification {
  id: string;
  title: string;
  recipientIds: string[];
  recipientListIds: string[];
  highTemp: boolean;
  lowTemp: boolean;
  confirmOk: boolean;
  remind: boolean;
  reminderInterval: number;
  reminderUnits: 'seconds' | 'minutes' | 'hours';
}

type CCNotificationEditFormProps = {
  onUpdate: (patch: Partial<CCNotification>) => void;
  draft: CCNotification;
};

export const CCNotificationEditForm = ({
  onUpdate,
  draft,
}: CCNotificationEditFormProps) => {
  const t = useTranslation('system');
  return (
    <>
      <ul style={{ listStyleType: 'none', padding: '0' }}>
        <li>
          <Checkbox
            id="highTemp"
            checked={draft.highTemp}
            onClick={() => onUpdate({ highTemp: !draft.highTemp })}
          />
          <label htmlFor="highTemp">
            {t('label.coldchain-high-temp-alerts')}
          </label>
        </li>
        <li>
          <Checkbox
            id="lowTemp"
            checked={draft.lowTemp}
            onClick={() => onUpdate({ lowTemp: !draft.lowTemp })}
          />
          <label htmlFor="lowTemp">
            {t('label.coldchain-high-temp-alerts')}
          </label>
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
            id="remind"
            checked={draft.remind}
            onClick={() => onUpdate({ remind: !draft.remind })}
          />
          <label htmlFor="remind">{t('label.coldchain-reminder-alerts')}</label>
        </li>
        <Box
          sx={{
            display: 'flex',
            alignItems: 'center',
            gap: '10px',
            marginLeft: '40px',
          }}
        >
          <PositiveNumberInput
            disabled={!draft.remind}
            autoFocus
            value={draft.reminderInterval}
            required
            onChange={newValue => onUpdate({ reminderInterval: newValue })}
            sx={{ width: '60px' }}
          />
          <Select
            value={draft.reminderUnits}
            disabled={!draft.remind}
            onChange={e =>
              onUpdate({
                reminderUnits: e.target
                  .value as CCNotification['reminderUnits'],
              })
            }
            options={[
              { label: t('label.seconds'), value: 'seconds' },
              { label: t('label.minutes'), value: 'minutes' },
              { label: t('label.hours'), value: 'hours' },
            ]}
          />
        </Box>
      </ul>
    </>
  );
};
