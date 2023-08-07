import { useTranslation } from '@common/intl';
import {
  Box,
  ButtonWithIcon,
  Grid,
  Paper,
  SettingsIcon,
  Typography,
} from '@common/ui';
import { useNavigate } from 'packages/common/src';
import React from 'react';

const dummyData = [
  { id: 'friends-id', name: 'Friends', description: 'My good friends' },
  { id: 'foes-id', name: 'Foes', description: 'Keep your enemies closer' },
  { id: 'fries-id', name: 'Fries', description: 'With ketchup please' },
  {
    id: 'kids-id',
    name: 'Kids',
    description: 'This is a description about the group',
  },
  {
    id: 'mates-id',
    name: 'Mates',
    description: 'This is a description about the group',
  },
  {
    id: 'lads-id',
    name: 'Lads',
    description: 'This is a description about the group',
  },
  {
    id: 'homies-id',
    name: 'Homies',
    description: 'This is a description about the group',
  },
  {
    id: 'cuties-id',
    name: 'Cuties',
    description: 'This is a description about the group',
  },
  {
    id: 'bros-id',
    name: 'Bros',
    description: 'This is a description about the group',
  },
];

export const GroupList = () => {
  const t = useTranslation('system');
  const navigate = useNavigate();

  return (
    <Grid
      container
      spacing={2}
      sx={{
        padding: '0 16px 16px 0',
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
            <Box>
              <Typography
                sx={{
                  fontSize: '14px',
                  fontWeight: 'bold',
                  color: 'gray.dark',
                }}
              >
                {group.name}
              </Typography>
              <Typography sx={{ color: 'gray.dark' }}>
                {group.description}
              </Typography>
            </Box>
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
