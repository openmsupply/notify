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
    description: 'This is a description about the list',
  },
  {
    id: 'mates-id',
    name: 'Mates',
    description: 'This is a description about the list',
  },
  {
    id: 'lads-id',
    name: 'Lads',
    description: 'This is a description about the list',
  },
  {
    id: 'homies-id',
    name: 'Homies',
    description: 'This is a description about the list',
  },
  {
    id: 'cuties-id',
    name: 'Cuties',
    description: 'This is a description about the list',
  },
  {
    id: 'bros-id',
    name: 'Bros',
    description: 'This is a description about the list',
  },
];

export const AllLists = () => {
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
      {dummyData.map(list => (
        <Grid item xs={12} md={6} key={list.id} sx={{ height: 'fit-content' }}>
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
            key={list.id}
          >
            <Box>
              <Typography
                sx={{
                  fontSize: '14px',
                  fontWeight: 'bold',
                  color: 'gray.dark',
                }}
              >
                {list.name}
              </Typography>
              <Typography sx={{ color: 'gray.dark' }}>
                {list.description}
              </Typography>
            </Box>
            <ButtonWithIcon
              Icon={<SettingsIcon />}
              onClick={() => navigate(list.id)}
              title={t('tooltip.manage-recipient-list')}
              label={t('label.manage')}
            />
          </Paper>
        </Grid>
      ))}
    </Grid>
  );
};
