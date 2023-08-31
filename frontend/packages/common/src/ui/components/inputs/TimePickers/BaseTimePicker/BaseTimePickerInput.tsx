import React, { FC } from 'react';
import TimePicker, { TimePickerProps } from '@mui/lab/TimePicker';
import { BasicTextInput } from '../../TextInput/BasicTextInput';
import { StandardTextFieldProps, TextFieldProps } from '@mui/material';

export const BaseTimePickerInput: FC<
  Omit<TimePickerProps, 'renderInput'>
> = props => {
  return (
    <TimePicker
      disabled={props.disabled}
      renderInput={(params: TextFieldProps) => {
        const textInputProps: StandardTextFieldProps = {
          ...params,
          variant: 'standard',
        };
        return (
          <BasicTextInput disabled={!!props.disabled} {...textInputProps} />
        );
      }}
      {...props}
    />
  );
};
