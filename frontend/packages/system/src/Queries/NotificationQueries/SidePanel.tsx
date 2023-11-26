import React from 'react';
import { BufferedTextArea} from '@notify-frontend/common';
import { Box, DetailPanelPortal, PanelLabel, PanelRow, SaveIcon } from '@common/ui';
import { BufferedTextInput, DetailPanelSection, IconButton } from '@common/components';
import { KeyedParams, TeraUtils } from '@common/utils';
import { useTranslation } from '@common/intl';

export interface ParamsPanelProps {
  query: string;
  queryParams: KeyedParams;
  onUpdateQueryParams: (key: string, value: string) => void;
  generatedQuery: string;
  userQueryParameters: KeyedParams | null;
  setUserQueryParameters: (queryParams: KeyedParams) => void;
}
export const SidePanel = ({
  query,
  queryParams,
  onUpdateQueryParams,
  generatedQuery,
  setUserQueryParameters,
}: ParamsPanelProps) => {
  const t = useTranslation('system');

  const onSaveInLocalStorage = (queryParams: KeyedParams) => {    
    setUserQueryParameters(queryParams);
  };

  const paramEditor = (
    <DetailPanelSection 
      title={t('label.parameters')}
      actionButtons={
        <>
          <IconButton
            onClick={() => onSaveInLocalStorage(queryParams)}
            icon={<SaveIcon />}
            label={t('label.parameters-save-local')}
          />
        </>
      }  
    >
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

  const generatedSQLViewer = (
    <DetailPanelSection title={t('label.generated-sql')}> 
      <BufferedTextArea
          value={generatedQuery}
          InputProps={{ sx: { backgroundColor: 'white' } }}
          InputLabelProps={{ shrink: true }}
          minRows={10}
          disabled
        />
    </DetailPanelSection>
  );
  return <DetailPanelPortal>{paramEditor}{generatedSQLViewer}</DetailPanelPortal>;
};
