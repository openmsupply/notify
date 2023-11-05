import React, { useEffect } from 'react';
import { useBreadcrumbs, useQueryParamsState } from '@common/hooks';
import {
  AppBarButtonsPortal,
  AppBarContentPortal,
  BasicSpinner,
  Box,
  EditIcon,
  LoadingButton,
  RelativeTimeDate,
  Stack,
  TextArea,
  Typography,
} from '@common/ui';
import { useTranslation } from '@common/intl';
import { useNotificationEvents } from '../api';
import {
  ConfigKind,
  EventStatus,
  useNavigate,
  useParams,
} from 'packages/common/src';
import { configRoute } from '../../Notifications/navigate';
import { NotificationStatusChip } from '../components/NotificationStatusChip';

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
            {t('button.edit-config')} ({entity.notificationConfig?.title})
          </LoadingButton>
        )}
      </AppBarButtonsPortal>
      <AppBarContentPortal sx={{ paddingBottom: '16px', flex: 1 }}>
        <Box display="flex" flexDirection="row" gap={1} width="100%">
          <Box gap={1} alignItems="flex" display="flex" flex={1}>
            <Box flex={1} display="flex">
              <Box flex={0.2} gap={1}>
                <Stack gap={1}>
                  <NotificationStatusChip
                    status={entity?.status ?? EventStatus.Errored}
                  />
                  <RelativeTimeDate d={entity?.sentAt} />
                </Stack>
              </Box>
              <Box flex={1} justifyContent="center" display="flex" gap={1}>
                {entity?.errorMessage ? (
                  <TextArea value={entity?.errorMessage} />
                ) : (
                  <Typography variant="body1">No Error</Typography>
                )}
              </Box>
              <Box flex={1} justifyContent="right" display="flex" gap={1}>
                <Stack gap={1}>
                  <Typography variant="body1">
                    Created:
                    <RelativeTimeDate d={entity?.createdAt} />
                  </Typography>
                  <Typography variant="body1">
                    Updated:
                    <RelativeTimeDate d={entity?.updatedAt} />
                  </Typography>
                </Stack>
              </Box>
            </Box>
          </Box>
        </Box>
      </AppBarContentPortal>
      {/* Description/Details section */}
      <Box sx={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
        <Box sx={{ flex: '1', overflow: 'auto', padding: '4px' }}>
          {isLoading ? (
            <BasicSpinner />
          ) : (
            <>
              <Typography variant="h4">{entity?.title}</Typography>
              <Typography variant="h6">
                {entity?.toAddress} ({entity?.notificationType})
              </Typography>
              <TextArea
                minRows={2}
                maxRows={25}
                sx={{
                  border: '1px solid',
                  borderColor: 'grey.100',
                  width: '100%',
                }}
                value={entity?.message}
              />
            </>
          )}
        </Box>
      </Box>
    </>
  );
};
