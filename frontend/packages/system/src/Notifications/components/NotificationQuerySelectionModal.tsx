import React, { FC, useMemo, useState } from 'react';
import Alert from '@mui/material/Alert';
import AlertTitle from '@mui/material/AlertTitle';
import { useDialog } from '@common/hooks';
import { useTranslation } from '@common/intl';

import {
  AutocompleteMultiList,
  AutocompleteOptionRenderer,
  Checkbox,
  DialogButton,
  LoadingButton,
  Tooltip,
} from '@common/components';
import { CheckIcon } from '@common/icons';
import { NotificationQueryRowFragment } from '../../Queries/api';
import { Grid } from '@common/ui';

interface NotificationQuerySelectionModalProps {
  sqlQueries: NotificationQueryRowFragment[];
  initialSelectedIds: string[];
  isOpen: boolean;
  onClose: () => void;
  setSelection: (input: { notificationQueryIds: string[] }) => void;
}

interface NotificationQueryOption {
  id: string;
  name: string;
  detail: string;
}

export const NotificationQuerySelectionModal: FC<
  NotificationQuerySelectionModalProps
> = ({ sqlQueries, initialSelectedIds, isOpen, onClose, setSelection }) => {
  const t = useTranslation('system');
  const [errorMessage, setErrorMessage] = useState('');
  const [selectedIds, setSelectedIds] = useState<string[]>([]);

  const { Modal } = useDialog({ isOpen, onClose });

  const options = useMemo<NotificationQueryOption[]>(() => {
    return sqlQueries.map(sqlQuery => ({
      id: sqlQuery.id,
      name: sqlQuery.name,
      detail: sqlQuery.query,
    }));
  }, [sqlQueries]);

  const onChangeSelectedQueries = (ids: string[]) => {
    setSelectedIds(ids);
  };

  const onSubmit = async () => {
    setSelection({ notificationQueryIds: selectedIds });
    onClose();
  };

  const modalHeight = Math.min(window.innerHeight - 100, 700);
  const modalWidth = Math.min(window.innerWidth - 100, 924);

  return (
    <Modal
      height={modalHeight}
      width={modalWidth}
      okButton={
        <Tooltip title={t('label.select-queries')}>
          <span>
            <LoadingButton
              disabled={false}
              onClick={onSubmit}
              isLoading={false}
              startIcon={<CheckIcon />}
            >
              {t('label.select-queries')}
            </LoadingButton>
          </span>
        </Tooltip>
      }
      cancelButton={<DialogButton variant="cancel" onClick={onClose} />}
      title={t('label.select-queries')}
      slideAnimation={false}
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
            onChange={onChangeSelectedQueries}
            renderOption={renderOption}
            getOptionLabel={option => `${option.detail} ${option.name}`}
            filterProperties={['name', 'detail']}
            filterPlaceholder={t('placeholder.search')}
            width={modalWidth - 50}
            height={modalHeight - 300}
            defaultSelection={options.filter(o =>
              initialSelectedIds.includes(o.id)
            )}
          />
        </Grid>
      </Grid>
    </Modal>
  );
};

const renderOption: AutocompleteOptionRenderer<NotificationQueryOption> = (
  props,
  option,
  { selected }
): JSX.Element => (
  <li {...props}>
    <Checkbox checked={selected} />
    <Tooltip title={option.name}>
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
        {option.name}
      </span>
    </Tooltip>
    <Tooltip title={option.detail}>
      <span>{option.detail}</span>
    </Tooltip>
  </li>
);
