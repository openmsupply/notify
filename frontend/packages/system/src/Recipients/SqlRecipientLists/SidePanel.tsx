import React from 'react';
import { Box, DetailPanelPortal, PanelLabel, PanelRow } from '@common/ui';
import { BufferedTextInput, DetailPanelSection } from '@common/components';
import { KeyedParams, TeraUtils } from '@common/utils';
import { useTranslation } from '@common/intl';

export interface ParamsPanelProps {
  query: string;
  queryParams: KeyedParams;
  onUpdateQueryParams: (key: string, value: string) => void;
}

export const SidePanel = ({
  query,
  queryParams,
  onUpdateQueryParams,
}: ParamsPanelProps) => {
  const t = useTranslation('system');

  const paramEditor = (
    <DetailPanelSection title={t('label.parameters')}>
      {TeraUtils.extractParams(query).length === 0 ? (
        <PanelRow>
          <PanelLabel>{t('message.no-parameters')}</PanelLabel>
        </PanelRow>
      ) : (
        <>
          {TeraUtils.extractParams(query).map(param => {
            return (
              <Box key={`param-${param}`} paddingBottom={2}>
                <PanelRow>
                  <PanelLabel>{param}</PanelLabel>
                </PanelRow>
                <PanelRow>
                  <BufferedTextInput
                    sx={{ flex: 1 }}
                    InputProps={{
                      sx: {
                        backgroundColor: 'white',
                      },
                    }}
                    value={queryParams[param ?? '']}
                    onChange={e =>
                      onUpdateQueryParams(param ?? '', e.target.value)
                    }
                  />
                </PanelRow>
              </Box>
            );
          })}
        </>
      )}
    </DetailPanelSection>
  );

  return <DetailPanelPortal>{paramEditor}</DetailPanelPortal>;
};
