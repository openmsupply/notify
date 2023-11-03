import React, { useEffect } from 'react';
import { useBreadcrumbs, useQueryParamsState } from '@common/hooks';
import {
  AppBarButtonsPortal,
  BasicSpinner,
  Box,
  EditIcon,
  LoadingButton,
  TextArea,
  Typography,
} from '@common/ui';
import { useTranslation } from '@common/intl';
import { useNotificationEvents } from '../api';
import { ConfigKind, useNavigate, useParams } from 'packages/common/src';
import { configRoute } from '../../Notifications/navigate';

export const DetailView = () => {
  const t = useTranslation('system');
  const urlParams = useParams();
  const { suffix, setSuffix } = useBreadcrumbs();
  const navigate = useNavigate();

  const { queryParams } = useQueryParamsState({
    initialFilter: { id: { equalTo: urlParams['id'] } },
  });

  const { data, isLoading } = useNotificationEvents(queryParams);
  const entity = data?.nodes[0];

  useEffect(() => {
    const listName = entity?.title;
    if (!suffix && listName) {
      setSuffix(listName);
    }
  }, [suffix, entity]);

  return (
    <>
      <AppBarButtonsPortal>
        {/* if we have a config_id, create a link to edit the config */}
        {entity?.notificationConfigId && (
          <LoadingButton
            isLoading={isLoading}
            onClick={() => {
              navigate(
                configRoute(
                  entity.notificationConfig?.kind ?? ConfigKind.Scheduled,
                  entity.notificationConfigId ?? ''
                )
              );
            }}
            variant="outlined"
            endIcon={<EditIcon />}
          >
            {t('button.edit-config')}
          </LoadingButton>
        )}
      </AppBarButtonsPortal>
      {/* Description/Details section */}
      <Box sx={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
        <Box sx={{ flex: '1', overflow: 'auto', padding: '4px' }}>
          {isLoading ? (
            <BasicSpinner />
          ) : (
            <>
              <Typography variant="h4">{entity?.title}</Typography>
              <TextArea value={entity?.message} />
              {/* TODO: Show all the fields */}
            </>
          )}
        </Box>
      </Box>
    </>
  );
};
