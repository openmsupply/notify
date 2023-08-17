import React, { PropsWithChildren, useState } from 'react';
import {
  AutocompleteMultiList,
  AutocompleteOptionRenderer,
  BasicTextInput,
  Box,
  ButtonProps,
  Checkbox,
  ChevronDownIcon,
  DropdownMenu,
  DropdownMenuItem,
  Grid,
  NotificationTypeNode,
  PositiveNumberInput,
  styled,
  useTranslation,
} from '@notify-frontend/common';
import { useRecipientLists, useRecipients } from '../../Recipients/api';

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
    padding: '7px 5px 7px 10px',
    color: theme.palette.gray.main,
    cursor: 'pointer',
  };
});

export const CCNotificationEditForm = ({
  onUpdate,
  draft,
}: CCNotificationEditFormProps) => {
  const t = useTranslation(['system']);
  const [open, setOpen] = useState(false);

  const { data: recipients } = useRecipients();
  const { data: recipientLists } = useRecipientLists();

  const options: RecipientOption[] = [
    { id: 'recipientLists title', name: '--- Recipient Lists ---', detail: '' },
    ...(recipientLists?.nodes ?? []).map(r => ({
      id: r.id,
      name: r.name,
      detail: r.description,
    })),
    { id: 'recipients title', name: '--- Recipients ---', detail: '' },
    ...(recipients?.nodes ?? []).map(r => ({
      id: r.id,
      name: r.name,
      detail:
        r.notificationType === NotificationTypeNode.Telegram
          ? 'Telegram'
          : r.toAddress,
    })),
  ];

  return (
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
      <SelectButton onClick={() => setOpen(!open)}>
        <span>Select Recipients</span>
        <ChevronDownIcon
          color="primary"
          style={{ transform: open ? 'rotate(180deg)' : '' }}
        />
      </SelectButton>
      {open && (
        <AutocompleteMultiList
          options={options}
          // onChange={onChangeSelectedRecipients}
          getOptionLabel={option => `${option.name} (${option.detail})`}
          renderOption={renderOption}
          filterProperties={['name', 'detail']}
          filterPlaceholder={t('placeholder.search')}
          width={976}
          height={150}
          getOptionDisabled={o => o.name.startsWith('--- Recipient')}
          showSelectedCount={false}
        />
      )}
    </Grid>
  );
};

const renderOption: AutocompleteOptionRenderer<RecipientOption> = (
  props,
  option,
  { selected }
): JSX.Element => (
  <li {...props}>
    {!option.name.startsWith('--- Recipient') && (
      <Checkbox checked={selected} />
    )}
    <span
      style={{
        fontWeight: 700,
        whiteSpace: 'nowrap',
        overflow: 'hidden',
        textOverflow: 'ellipsis',
        marginRight: 10,
      }}
    >
      {option.name}
    </span>
    {option.detail && (
      <>
        {' ('}
        <span
          style={{
            whiteSpace: 'nowrap',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            maxWidth: '200px',
          }}
        >
          {option.detail}
        </span>
        {')'}
      </>
    )}
  </li>
);
