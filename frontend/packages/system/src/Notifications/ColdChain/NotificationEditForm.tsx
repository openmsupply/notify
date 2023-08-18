import React from 'react';
import {
  BasicTextInput,
  Box,
  Checkbox,
  Grid,
  PositiveNumberInput,
  Select,
} from '@notify-frontend/common';

export interface CCNotification {
  id: string;
  title: string;
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
  return (
    <>
      <Grid flexDirection="column" display="flex" gap={2}>
        <BasicTextInput
          autoFocus
          value={draft.title}
          required
          onChange={e => onUpdate({ title: e.target.value })}
          label={'Notification Title'}
          InputLabelProps={{ shrink: true }}
        />
        <ul style={{ listStyleType: 'none', padding: '0' }}>
          <li>
            <Checkbox
              id="highTemp"
              checked={draft.highTemp}
              onClick={() => onUpdate({ highTemp: !draft.highTemp })}
            />
            <label htmlFor="highTemp">
              Send high temperature alerts (Limits are based on your mSupply
              configuration)
            </label>
          </li>
          <li>
            <Checkbox
              id="lowTemp"
              checked={draft.lowTemp}
              onClick={() => onUpdate({ lowTemp: !draft.lowTemp })}
            />
            <label htmlFor="lowTemp">
              Send low temperature alerts (Limits are based on your mSupply
              configuration)
            </label>
          </li>
          <li>
            <Checkbox
              id="confirmOk"
              checked={draft.confirmOk}
              onClick={() => onUpdate({ confirmOk: !draft.confirmOk })}
            />
            <label htmlFor="confirmOk">Send temperature OK confirmation</label>
          </li>
          <li>
            <Checkbox
              id="remind"
              checked={draft.remind}
              onClick={() => onUpdate({ remind: !draft.remind })}
            />
            <label htmlFor="remind">
              Send follow-up reminders until alert resolved, every:
            </label>
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
                { label: 'Seconds', value: 'seconds' },
                { label: 'Minutes', value: 'minutes' },
                { label: 'Hours', value: 'hours' },
              ]}
            />
          </Box>
        </ul>
      </Grid>
    </>
  );
};
