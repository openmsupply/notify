import React from 'react';
import { TestingProvider } from '@common/utils';
import { render, screen, within } from '@testing-library/react';
import { RecipientEditForm } from './RecipientEditForm';
import { ModalMode } from '@common/hooks';
import { NotificationTypeNode } from '@common/types';

describe('RecipientEditForm', () => {
  const draftEmailRecipient = {
    id: 'email',
    name: 'some test person',
    toAddress: 'test@email.com',
    notificationType: NotificationTypeNode.Email,
  };
  const draftTelegramRecipient = {
    id: 'telegram',
    name: 'the telegram group',
    toAddress: '-12345',
    notificationType: NotificationTypeNode.Telegram,
  };

  it('renders notification type toggle in create mode', () => {
    render(
      <TestingProvider>
        <RecipientEditForm
          draft={draftEmailRecipient}
          mode={ModalMode.Create}
          onUpdate={() => {}}
        />
      </TestingProvider>
    );

    const buttonGroup = screen.getByRole('group');
    expect(buttonGroup).toBeInTheDocument();

    const notificationTypeOptions = within(buttonGroup).getAllByRole('button');
    expect(notificationTypeOptions).toHaveLength(2);
    expect(notificationTypeOptions[0]).toHaveTextContent('Email');
  });
  it('does not render notification type options in update mode', () => {
    render(
      <TestingProvider>
        <RecipientEditForm
          draft={draftEmailRecipient}
          mode={ModalMode.Update}
          onUpdate={() => {}}
        />
      </TestingProvider>
    );

    const buttonGroup = screen.queryByRole('group');
    expect(buttonGroup).not.toBeInTheDocument();
  });
  it('renders name and email inputs for email recipients', () => {
    render(
      <TestingProvider>
        <RecipientEditForm
          draft={draftEmailRecipient}
          mode={ModalMode.Update}
          onUpdate={() => {}}
        />
      </TestingProvider>
    );

    const nameInput = screen.getByLabelText('Name *');
    expect(nameInput).toHaveValue('some test person');

    const emailInput = screen.getByLabelText('Email *');
    expect(emailInput).toHaveValue('test@email.com');
  });
  it('renders explanation text when creating a telegram recipient', () => {
    render(
      <TestingProvider>
        <RecipientEditForm
          draft={draftTelegramRecipient}
          mode={ModalMode.Create}
          onUpdate={() => {}}
        />
      </TestingProvider>
    );

    const nameInput = screen.queryByLabelText('Name *');
    expect(nameInput).not.toBeInTheDocument();

    expect(
      screen.getByText(/add your notify bot to a telegram group/i)
    ).toBeInTheDocument();
  });
  it('renders name input when updating Telegram recipient', () => {
    render(
      <TestingProvider>
        <RecipientEditForm
          draft={draftTelegramRecipient}
          mode={ModalMode.Update}
          onUpdate={() => {}}
        />
      </TestingProvider>
    );

    const nameInput = screen.getByLabelText('Name *');
    expect(nameInput).toHaveValue('the telegram group');
  });
});
