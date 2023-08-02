import React, { lazy, useState } from 'react';
import {
  BasicTextInput,
  useTranslation,
  PasswordTextInput,
  useAuthContext,
  isValidUsername,
  validateUsernameHelperText,
  UpdateOwnUserAccountInput,
  Stack,
  SaveIcon,
  LoadingButton,
} from '@notify-frontend/common';
import { useOwnUserAccountUpdate } from './api';

const PasswordStrengthMeter = lazy(
  () => import('packages/system/src/Users/Components/PasswordStrengthMeter')
);

const userIdentifyingData = (
  userAccount: UpdateOwnUserAccountInput
): string[] => {
  return [
    userAccount.displayName ?? '',
    userAccount.email ? userAccount.email : '',
  ];
};

const rootStackStyle = { width: 500, paddingTop: 2, paddingLeft: 2 };

const MyAccountPage: React.FC = () => {
  const t = useTranslation(['system']);
  const { user } = useAuthContext();
  if (!user) {
    return <div>{t('not-logged-in')}</div>;
  }

  const { id, displayName, email, username } = user;
  const [userAccount, setUserAccount] = useState<UpdateOwnUserAccountInput>({
    id,
    displayName,
    email,
    username,
  });
  const onUpdate = (patch: Partial<UpdateOwnUserAccountInput>) => {
    setUserAccount({ ...userAccount, ...patch });
  };
  const { mutate, isLoading } = useOwnUserAccountUpdate();

  return (
    <Stack spacing={2} sx={rootStackStyle}>
      <BasicTextInput
        autoFocus
        value={userAccount.username}
        error={!isValidUsername(userAccount.username ?? '')}
        helperText={validateUsernameHelperText(userAccount.username ?? '', t)}
        label={t('heading.username')}
        disabled
        InputLabelProps={{ shrink: true }}
      />
      <Stack spacing={0.5}>
        <PasswordTextInput
          value={userAccount.password || ''}
          onChange={e => onUpdate({ password: e.target.value })}
          label={t('heading.password')}
        />
        {!!userAccount.password && (
          <PasswordStrengthMeter
            password={userAccount.password || ''}
            userInfo={userIdentifyingData(userAccount)}
            includeFeedback={true}
          />
        )}
      </Stack>
      <BasicTextInput
        value={userAccount.displayName}
        onChange={e => onUpdate({ displayName: e.target.value })}
        label={t('label.name')}
        InputLabelProps={{ shrink: true }}
      />
      <BasicTextInput
        value={userAccount.email || ''}
        onChange={e => onUpdate({ email: e.target.value })}
        label={t('label.email')}
        InputLabelProps={{ shrink: true }}
      />
      <LoadingButton
        isLoading={isLoading}
        variant="text"
        startIcon={<SaveIcon />}
        onClick={() => {
          mutate(userAccount);
        }}
      >
        {t('label.save')}
      </LoadingButton>
    </Stack>
  );
};

export default MyAccountPage;
