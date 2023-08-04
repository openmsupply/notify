import { useTranslation } from '@common/intl';
import {
  ButtonWithIcon,
  Grid,
  Paper,
  SettingsIcon,
  Typography,
} from '@common/ui';
import { useNavigate } from 'packages/common/src';
import React from 'react';

const dummyData = [
  { id: 'friends-id', name: 'Friends' },
  { id: 'foes-id', name: 'Foes' },
  { id: 'fries-id', name: 'Fries' },
  { id: 'kids-id', name: 'Kids' },
  { id: 'mates-id', name: 'Mates' },
  { id: 'lads-id', name: 'Lads' },
  { id: 'homies-id', name: 'Homies' },
  { id: 'cuties-id', name: 'Cuties' },
  { id: 'bros-id', name: 'Bros' },
];

export const GroupList = () => {
  const t = useTranslation('system');
  const navigate = useNavigate();

  return (
    <Grid
      container
      spacing={2}
      sx={{
        padding: '0 0 16px 16px',
        maxWidth: '1200px',
        margin: '0 auto',
        height: 'min-content',
      }}
    >
      {dummyData.map(group => (
        <Grid item xs={12} md={6} key={group.id} sx={{ height: 'fit-content' }}>
          <Paper
            sx={{
              borderRadius: '16px',
              boxShadow: theme => theme.shadows[1],
              padding: '24px 32px',
              width: '100%',
              backgroundColor: 'background.menu',
              display: 'flex',
              justifyContent: 'space-between',
            }}
            key={group.id}
          >
            <Typography
              sx={{ fontSize: '14px', fontWeight: 'bold', color: 'gray.dark' }}
            >
              {group.name}
            </Typography>
            <ButtonWithIcon
              Icon={<SettingsIcon />}
              onClick={() => navigate(group.id)}
              title={t('tooltip.manage-notification-group')}
              label={t('label.manage')}
            />
          </Paper>
        </Grid>
      ))}
    </Grid>
  );
};
