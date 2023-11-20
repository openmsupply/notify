import React, { useEffect } from 'react';
import { useBreadcrumbs, useQueryParamsState } from '@common/hooks';
import {
  AppBarButtonsPortal,
  AppBarContentPortal,
  BasicSpinner,
  Box,
  EditIcon,
  BaseButton,
  RelativeTimeDate,
  Stack,
  TextArea,
  Typography,
  Tooltip,
} from '@common/ui';
import { useTranslation } from '@common/intl';
import { useNotificationEvents } from '../api';
import {
  ConfigKind,
  EventStatus,
  NotificationTypeNode,
  useNavigate,
  useParams,
} from 'packages/common/src';
import { configRoute } from '../../Notifications/navigate';
import { NotificationStatusChip } from '../components/NotificationStatusChip';

type EventContext = {
  recipient?: {
    name: string;
    notification_type: NotificationTypeNode;
    to_address: string;
  };
  [key: string]: unknown;
};

const parseContext = (context: string | null | undefined): EventContext => {
  if (!context) return {};
  try {
    return JSON.parse(context);
  } catch (e) {
    return {};
  }
};

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
  const { recipient, ...params } = parseContext(entity?.context);

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
          <Tooltip title={entity.notificationConfig?.title ?? ''}>
            <BaseButton
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
              {t('button.edit-notification-config')}
            </BaseButton>
          </Tooltip>
        )}
      </AppBarButtonsPortal>
      <AppBarContentPortal sx={{ paddingBottom: '16px', flex: 1 }}>
        <Box flex={1} display="flex" gap={2}>
          <Box flex={0.3}>
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
      </AppBarContentPortal>
      {/* Description/Details section */}
      <Box
        sx={{ width: '100%' }}
        padding={2}
        display="flex"
        flexDirection="column"
      >
        {isLoading ? (
          <BasicSpinner />
        ) : (
          <>
            <Typography variant="h6">Generated Notification</Typography>
            <TextArea
              label="To"
              InputLabelProps={{ shrink: true }} // so label is always visisble
              minRows={1}
              maxRows={1}
              sx={{
                border: '1px solid',
                borderColor: 'grey.100',
                width: '100%',
              }}
              value={`${recipient?.name ? `${recipient.name}: ` : ''}${
                entity?.toAddress
              } (${entity?.notificationType})`}
            />

            <TextArea
              label="Title"
              InputLabelProps={{ shrink: true }}
              minRows={1}
              maxRows={1}
              sx={{
                border: '1px solid',
                borderColor: 'grey.100',
                width: '100%',
              }}
              value={entity?.title}
            />

            <TextArea
              label="Message"
              InputLabelProps={{ shrink: true }}
              minRows={2}
              maxRows={25}
              sx={{
                border: '1px solid',
                borderColor: 'grey.100',
                width: '100%',
              }}
              value={entity?.message}
            />

            <TextArea
              label="Input data (parameters and query results)"
              InputLabelProps={{ shrink: true }}
              minRows={2}
              maxRows={15}
              sx={{
                border: '1px solid',
                borderColor: 'grey.100',
                width: '100%',
              }}
              value={JSON.stringify(params, null, 2)}
            />
          </>
        )}
      </Box>
    </>
  );
};
