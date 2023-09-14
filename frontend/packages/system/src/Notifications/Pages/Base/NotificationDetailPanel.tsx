import React from 'react';
import { DetailPanelPortal, PanelLabel, PanelRow } from '@common/ui';
import { BufferedTextInput, DetailPanelSection } from '@common/components';
import { KeyedParams } from '@common/utils';
import { useTranslation } from '@common/intl';

export interface ParamsPanelProps {
  requiredParams: string[];
  params: KeyedParams;
  onUpdateParams: (key: string, value: string) => void;
}

export const NotificationDetailPanel = ({
  requiredParams,
  params,
  onUpdateParams,
}: ParamsPanelProps) => {
  const t = useTranslation('system');

  const allParams = [...new Set(requiredParams.concat(Object.keys(params)))];
  console.log('allParams', allParams);
  console.log('params', params);
  const paramEditor = (
    <DetailPanelSection title={t('label.parameters')}>
      {allParams.map(param => {
        return (
          <>
            <PanelRow key={`param-label-row-${param}`}>
              <PanelLabel key={`param-label-${param}`}>{param}</PanelLabel>
            </PanelRow>
            <PanelRow key={`param-value-row-${param}`}>
              <BufferedTextInput
                key={`param-value-${param}`}
                InputProps={{
                  sx: {
                    backgroundColor: 'white',
                  },
                }}
                value={params[param ?? '']}
                onChange={e => onUpdateParams(param ?? '', e.target.value)}
              />
            </PanelRow>
          </>
        );
      })}
    </DetailPanelSection>
  );

  return <DetailPanelPortal>{paramEditor}</DetailPanelPortal>;
};
