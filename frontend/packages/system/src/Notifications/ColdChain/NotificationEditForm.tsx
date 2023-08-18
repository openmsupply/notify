import React, { PropsWithChildren, useState } from 'react';
import {
  BasicTextInput,
  Box,
  ButtonProps,
  Checkbox,
  Grid,
  PlusCircleIcon,
  PositiveNumberInput,
  Select,
  styled,
  useEditModal,
} from '@notify-frontend/common';
import { RecipientsModal } from './RecipientsModal';

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

// TODO: this is a hackkkkk can I make dropdown work?
const Button = ({ children, ...props }: PropsWithChildren<ButtonProps>) => (
  <div {...props}>{children}</div>
);
const SelectButton = styled(Button)(({ theme }) => {
  return {
    display: 'flex',
    justifyContent: 'space-between',
    borderRadius: '8px',
    backgroundColor: 'white',
    border: '1px',
    borderStyle: 'solid',
    borderColor: theme.palette.border,
    padding: '7px 10px',
    color: theme.palette.gray.main,
    cursor: 'pointer',
  };
});

export const CCNotificationEditForm = ({
  onUpdate,
  draft,
}: CCNotificationEditFormProps) => {
  const { isOpen, onClose, onOpen } = useEditModal();

  const [selectedIds, setSelectedIds] = useState<string[]>([]);
  return (
    <>
      {isOpen && (
        <RecipientsModal
          isOpen={isOpen}
          onClose={onClose}
          selectedIds={selectedIds}
          setSelectedIds={setSelectedIds}
        />
      )}
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
        <SelectButton onClick={() => onOpen()}>
          <span>Select Recipients</span>
          <PlusCircleIcon color="primary" />
        </SelectButton>
      </Grid>
    </>
  );
};
