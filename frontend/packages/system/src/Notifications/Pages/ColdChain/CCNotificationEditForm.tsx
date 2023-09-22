import React from 'react';
import {
  Box,
  Checkbox,
  Grid,
  PositiveNumberInput,
  Select,
  Typography,
  useTranslation,
  Tooltip,
  InfoIcon,
} from '@notify-frontend/common';
import { CCNotification } from '../../types';

type CCNotificationEditFormProps = {
  onUpdate: (patch: Partial<CCNotification>) => void;
  draft: CCNotification;
};

const dummyLocations = [
  { id: 'store-1-location-A', name: 'Store 1, Location A' },
  { id: 'store-1-location-B', name: 'Store 1, Location B' },
  { id: 'store-1-location-C', name: 'Store 1, Location C' },
  { id: 'store-2-location-A', name: 'Store 1, Location A' },
  { id: 'store-2-location-B', name: 'Store 1, Location B' },
  { id: 'store-2-location-C', name: 'Store 1, Location C' },
  { id: 'store-2-location-D', name: 'Store 1, Location D' },
];

export const CCNotificationEditForm = ({
  onUpdate,
  draft,
}: CCNotificationEditFormProps) => {
  const t = useTranslation('system');
  return (
    <>
      <Typography
          sx={{ fontWeight: 700, fontSize: '13px', marginTop: '10px', marginBottom: '10px' }}
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
            <Tooltip title={t('messages.cold-chain-temperature-information')}>
              <span>
                {' '}
                <InfoIcon fontSize="small" color="inherit" />
              </span>
            </Tooltip>
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
            <Tooltip title={t('messages.cold-chain-temperature-information')}>
              <span>
                {' '}
                <InfoIcon fontSize="small" color="inherit" />
              </span>
            </Tooltip>
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
            id="noData"
            checked={draft.noData}
            onClick={() => onUpdate({ noData: !draft.noData })}
          />
          <label htmlFor="noData">
            {t('label.coldchain-no-data-alerts')}
            <Tooltip title={t('messages.cold-chain-no-data-information')}>
              <span>
                {' '}
                <InfoIcon fontSize="small" color="inherit" />
              </span>
            </Tooltip>
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
            disabled={!draft.noData}
            value={draft.noDataInterval}
            required
            onChange={newValue => onUpdate({ noDataInterval: newValue })}
            sx={{ width: '60px' }}
          />

          <Select
            value={draft.noDataUnits}
            disabled={!draft.noData}
            onChange={e =>
              onUpdate({
                noDataUnits: e.target.value as CCNotification['noDataUnits'],
              })
            }
            options={[
              { label: t('label.seconds'), value: 'seconds' },
              { label: t('label.minutes'), value: 'minutes' },
              { label: t('label.hours'), value: 'hours' },
            ]}
          />
        </Box>
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

      <Box>
        <Typography
          sx={{ fontWeight: 700, fontSize: '13px', marginBottom: '10px' }}
        >
          {t('heading.select-locations')}
        </Typography>
        <Grid container>
          {dummyLocations.map(location => {
            const isSelected = draft.locationIds.includes(location.id);
            return (
              <Grid
                item
                md={4}
                key={location.id}
                sx={{ display: 'flex', alignItems: 'center' }}
              >
                <Checkbox
                  id={location.id}
                  checked={isSelected}
                  onClick={() => {
                    if (isSelected) {
                      onUpdate({
                        locationIds: draft.locationIds.filter(
                          id => id !== location.id
                        ),
                      });
                    } else {
                      onUpdate({
                        locationIds: [...draft.locationIds, location.id],
                      });
                    }
                  }}
                />
                <label
                  htmlFor={location.id}
                  style={{ display: 'inline-block', lineHeight: 1.3 }}
                >
                  {location.name}
                </label>
              </Grid>
            );
          })}
        </Grid>
      </Box>
    </>
  );
};
