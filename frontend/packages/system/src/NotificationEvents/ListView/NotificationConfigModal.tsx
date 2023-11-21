import React, { FC, useMemo, useState } from 'react';
import Alert from '@mui/material/Alert';
import AlertTitle from '@mui/material/AlertTitle';
import { useDialog } from '@common/hooks';
import { useTranslation } from '@common/intl';

import {
  AutocompleteMultiList,
  AutocompleteOptionRenderer,
  LoadingButton,
  Tooltip,
} from '@common/components';
import { NotificationConfigRowFragment } from '../../Notifications/api';
import { ConfigKind, ConfigStatus } from '@common/types';
import { CloseIcon, Grid } from '@common/ui';

interface NotificationConfigsModalProps {
  isOpen: boolean;
  notificationConfigs: NotificationConfigRowFragment[];
  onClose: () => void;
  setSelectedConfigId: (id: string) => void;
  selectedConfigId: string;
}

interface NotificationConfigOption {
  id: string;
  title: string;
  kind: ConfigKind;
  status: ConfigStatus;
}

export const NotificationConfigModal: FC<NotificationConfigsModalProps> = ({
  isOpen,
  notificationConfigs,
  selectedConfigId: selectedConfig,
  onClose,
  setSelectedConfigId: setSelectedConfig,
}) => {
  const t = useTranslation(['system']);
  const [errorMessage, setErrorMessage] = useState('');

  const { Modal } = useDialog({ isOpen, onClose });

  const options: NotificationConfigOption[] = useMemo(
    () =>
      notificationConfigs.map(c => ({
        id: c.id,
        title: c.title,
        kind: c.kind,
        status: c.status,
      })),
    [notificationConfigs]
  );

  const onChangeSelectedConfig = (ids: string[]) => {
    if (ids[0]) {
      setSelectedConfig(ids[0]);
      onClose();
    }
  };

  const modalHeight = Math.min(window.innerHeight - 100, 700);
  const modalWidth = Math.min(window.innerWidth - 100, 924);

  return (
    <Modal
      height={modalHeight}
      width={modalWidth}
      title={t('label.filter-by-notification-config')}
      slideAnimation={false}
      cancelButton={
        <LoadingButton
          disabled={!selectedConfig}
          onClick={() => {
            setSelectedConfig('');
            onClose();
          }}
          isLoading={false}
          startIcon={<CloseIcon />}
          variant="outlined"
        >
          {t('label.clear-filter')}
        </LoadingButton>
      }
    >
      <Grid
        flexDirection="column"
        display="flex"
        justifyContent="center"
        gap={2}
      >
        {errorMessage ? (
          <Grid item>
            <Alert
              severity="error"
              onClose={() => {
                setErrorMessage('');
              }}
            >
              <AlertTitle>{t('error')}</AlertTitle>
              {errorMessage}
            </Alert>
          </Grid>
        ) : null}
        <Grid item>
          <AutocompleteMultiList
            options={options}
            onChange={onChangeSelectedConfig}
            getOptionLabel={option => option.id}
            renderOption={renderOption}
            filterProperties={['title']}
            filterPlaceholder={t('placeholder.search')}
            showSelectedCount={false}
            width={modalWidth - 50}
            height={modalHeight - 200}
          />
        </Grid>
      </Grid>
    </Modal>
  );
};

const renderOption: AutocompleteOptionRenderer<NotificationConfigOption> = (
  props,
  option
): JSX.Element => (
  <li style={{ height: '45px' }} {...props}>
    <Tooltip title={option.title}>
      <span
        style={{
          fontWeight: 700,
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          width: 250,
          minWidth: 250,
          marginRight: 10,
        }}
      >
        {option.title}
      </span>
    </Tooltip>
    <Tooltip title={option.kind}>
      <span
        style={{
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          width: 150,
        }}
      >
        {option.kind}
      </span>
    </Tooltip>
    <Tooltip title={option.status}>
      <span
        style={{
          whiteSpace: 'nowrap',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
        }}
      >
        {option.status}
      </span>
    </Tooltip>
  </li>
);
