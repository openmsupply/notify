import React from 'react';
import { CopyIcon, DeleteIcon, DetailPanelPortal, SaveIcon } from '@common/ui';
import Alert from '@mui/material/Alert';
import AlertTitle from '@mui/material/AlertTitle';
import {
  BufferedTextArea,
  DetailPanelSection,
  IconButton,
} from '@common/components';
import { KeyedParams } from '@common/utils';

import { ParameterEditor } from './ParameterEditor';
import { useTranslation } from '@common/intl';

export interface ParamsPanelProps {
  requiredParams: string[];
  params: KeyedParams[];
  allowParameterSets?: boolean;
  onUpdateParams: (idx: number, key: string, value: string) => void;
  onDeleteParam: (idx: number, key: string | null) => void; // Warning: null deletes everything for that index
}

export const NotificationDetailPanel = ({
  requiredParams,
  params,
  allowParameterSets = false,
  onUpdateParams,
  onDeleteParam,
}: ParamsPanelProps) => {
  const t = useTranslation('system');

  const [isJsonEdited, setIsJsonEdited] = React.useState(false);
  const [paramsString, setParamsString] = React.useState('');
  const [errorMessage, setErrorMessage] = React.useState('');

  if (!Array.isArray(params)) {
    params = [params];
  }

  if (params.length === 0 || params[0] === undefined) {
    params = [{} as KeyedParams];
  }

  const paramEditors = (
    <>
      {params.map((_, idx) => {
        return (
          <DetailPanelSection
            key={`param-editor-detail-${idx}`}
            title={`${t('label.parameters')}: ${idx + 1}`}
            defaultExpanded={idx === params.length - 1}
            actionButtons={
              <>
                <IconButton
                  onClick={() => {
                    params.push(params[idx] ?? {});
                    onDeleteParam(idx + 1, 'this-is-a-hack-to-force-an-update');
                  }}
                  disabled={!allowParameterSets}
                  icon={<CopyIcon />}
                  label={t('button.duplicate')}
                />
                <IconButton
                  onClick={() => onDeleteParam(idx, null)}
                  disabled={params.length === 1}
                  icon={<DeleteIcon />}
                  label={t('label.delete')}
                />
              </>
            }
          >
            <ParameterEditor
              key={`param-editor-${idx}`}
              requiredParams={requiredParams}
              params={params[idx] ?? {}}
              onUpdateParams={(key, value) => onUpdateParams(idx, key, value)}
              onDeleteParam={key => onDeleteParam(idx, key)}
            />
          </DetailPanelSection>
        );
      })}
    </>
  );

  const jsonParamsEditor = (
    <DetailPanelSection
      key={`param-editor-detail-json`}
      title={t('label.parameters-as-json')}
      defaultExpanded={true}
    >
      <BufferedTextArea
        value={JSON.stringify(params)}
        onChange={e => {
          setParamsString(e.target.value);
          try {
            JSON.parse(e.target.value);
            setIsJsonEdited(true);
            setErrorMessage('');
          } catch (e) {
            setIsJsonEdited(false);
            if (e instanceof SyntaxError) {
              setErrorMessage(e.message);
            } else {
              alert(e);
            }
          }
        }}
      />
      {isJsonEdited && (
        <IconButton
          onClick={() => {
            try {
              const editedParams: KeyedParams[] = JSON.parse(paramsString);
              editedParams.forEach((param, idx) => {
                for (const key of Object.keys(param)) {
                  onUpdateParams(idx, key, param[key] ?? '');
                }
              });
              while (params.length > editedParams.length) {
                onDeleteParam(editedParams.length, null);
              }
            } catch (e) {
              setErrorMessage(`Unable to save new parameters: ${e}`);
            }

            setIsJsonEdited(false);
          }}
          icon={<SaveIcon />}
          label={t('button.save')}
        />
      )}
      {errorMessage ? (
        <Alert
          sx={{ marginTop: 2 }}
          severity="error"
          onClose={() => {
            setErrorMessage('');
          }}
        >
          <AlertTitle>{t('error')}</AlertTitle>
          {errorMessage}
        </Alert>
      ) : null}
    </DetailPanelSection>
  );

  return (
    <DetailPanelPortal>
      {paramEditors}
      {jsonParamsEditor}
    </DetailPanelPortal>
  );
};
