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
import { Grid } from '@common/ui';
import { SensorData, sensorDisplayName } from '../../api';

interface SensorSelectionSelectionModalProps {
  sensors: SensorData[];
  initialSelectedIds: string[];
  isOpen: boolean;
  onClose: () => void;
  setSelection: (input: { sensorIds: string[] }) => void;
}

interface SensorSelectionOption {
  id: string;
  name: string;
}

export const SensorSelectionModal: FC<SensorSelectionSelectionModalProps> = ({
  sensors,
  initialSelectedIds,
  isOpen,
  onClose,
  setSelection,
}) => {
  const t = useTranslation('system');
  const [errorMessage, setErrorMessage] = useState('');
  const [selectedIds, setSelectedIds] = useState<string[]>([]);

  const { Modal } = useDialog({ isOpen, onClose });

  const options = useMemo<SensorSelectionOption[]>(() => {
    return sensors.map(sensor => ({
      id: sensor.id,
      name: sensorDisplayName(sensor),
    }));
  }, [sensors]);

  const onChangeSelectedQueries = (ids: string[]) => {
    setSelectedIds(ids);
  };

  const onSubmit = async () => {
    if (selectedIds.length === 0) {
      setErrorMessage(t('messages.nothing-selected'));
      //   return;
    }

    setSelection({ sensorIds: selectedIds });
    onClose();
  };

  const modalHeight = Math.min(window.innerHeight - 100, 700);
  const modalWidth = Math.min(window.innerWidth - 100, 924);

  return (
    <Modal
      height={modalHeight}
      width={modalWidth}
      okButton={
        <Tooltip title={t('heading.select-sensors')}>
          <span>
            <LoadingButton
              disabled={false}
              onClick={onSubmit}
              isLoading={false}
              startIcon={<CheckIcon />}
            >
              {t('heading.select-sensors')}
            </LoadingButton>
          </span>
        </Tooltip>
      }
      cancelButton={<DialogButton variant="cancel" onClick={onClose} />}
      title={t('heading.select-sensors')}
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
            selectAllAllowed
            options={options}
            onChange={onChangeSelectedQueries}
            renderOption={renderOption}
            getOptionLabel={option => `${option.name}`}
            filterProperties={['name']}
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

const renderOption: AutocompleteOptionRenderer<SensorSelectionOption> = (
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
          minWidth: 550,
          marginRight: 10,
        }}
      >
        {option.name}
      </span>
    </Tooltip>
  </li>
);
