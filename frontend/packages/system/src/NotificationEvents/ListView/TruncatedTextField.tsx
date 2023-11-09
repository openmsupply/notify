import React from 'react';
import { Tooltip, Typography } from '@common/ui';
import { StringUtils } from '@common/utils';

interface TruncatedTextFieldProps {
  text: string;
  maxLength?: number;
}

const TruncatedTextField: React.FC<TruncatedTextFieldProps> = ({
  text,
  maxLength = 20,
}) => {
  return (
    <Tooltip title={text}>
      <Typography>{StringUtils.ellipsis(text, maxLength)}</Typography>
    </Tooltip>
  );
};

export default TruncatedTextField;
