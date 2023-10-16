import React from 'react';
import {
  Box,
  DeleteIcon,
  DetailPanelPortal,
  PanelLabel,
  PanelRow,
} from '@common/ui';
import {
  BufferedTextInput,
  DetailPanelSection,
  IconButton,
} from '@common/components';
import { KeyedParams } from '@common/utils';
import { useTranslation } from '@common/intl';

export interface ParamsPanelProps {
  requiredParams: string[];
  params: KeyedParams[];
  onUpdateParams: (idx: number, key: string, value: string) => void;
  onDeleteParam: (idx: number, key: string) => void;
}

export const NotificationDetailPanel = ({
  requiredParams,
  params,
  onUpdateParams,
  onDeleteParam,
}: ParamsPanelProps) => {
  const t = useTranslation('system');

  const allParams = [
    ...new Set(requiredParams.concat(Object.keys(params[0] ?? {}))),
  ];

  if (params.length === 0 || params[0] === undefined) {
    params = [{} as KeyedParams];
  }

  const paramEditor = (
    <DetailPanelSection title={t('label.parameters')}>
      {allParams.map(param => {
        return (
          <Box key={`param-row-${param}`} paddingBottom={1}>
            <PanelRow>
              <PanelLabel key={`param-label-${param}`}>{param}</PanelLabel>
            </PanelRow>
            <PanelRow>
              <BufferedTextInput
                key={`param-value-${param}`}
                InputProps={{
                  sx: {
                    backgroundColor: 'background.white',
                  },
                }}
                value={params[0]![param ?? ''] ?? ''} // TODO: Don't hardcode to 0
                onChange={e => onUpdateParams(0, param ?? '', e.target.value)} // TODO: Don't hardcode to 0
              />
              {
                // if param is not required allow it to be removed
                !requiredParams.includes(param) && (
                  <IconButton
                    onClick={() => onDeleteParam(0, param ?? '')}
                    icon={<DeleteIcon />}
                    label={t('label.delete')}
                  />
                )
              }
            </PanelRow>
          </Box>
        );
      })}
    </DetailPanelSection>
  );

  return <DetailPanelPortal>{paramEditor}</DetailPanelPortal>;
};
