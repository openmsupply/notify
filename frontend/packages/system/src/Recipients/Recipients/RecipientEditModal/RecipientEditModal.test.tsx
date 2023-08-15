import { NotificationTypeNode } from '@common/types';
import { checkIsInvalid } from './RecpientEditModal';
import { ModalMode } from '@common/hooks';

describe('checkIsInvalid', () => {
  it('returns true when toAddress empty', () => {
    expect(
      checkIsInvalid(
        {
          id: 'some-id',
          name: 'some-name',
          toAddress: '',
          notificationType: NotificationTypeNode.Email,
        },
        null
      )
    ).toBeTruthy();
  });
  it('returns true when name empty', () => {
    expect(
      checkIsInvalid(
        {
          id: 'some-id',
          name: '',
          toAddress: 'some-email@x.com',
          notificationType: NotificationTypeNode.Email,
        },
        null
      )
    ).toBeTruthy();
  });
  it('returns true when only whitespace provided', () => {
    expect(
      checkIsInvalid(
        {
          id: '   ',
          name: '   ',
          toAddress: 'some-email@x.com',
          notificationType: NotificationTypeNode.Email,
        },
        null
      )
    ).toBeTruthy();
  });
  it.each([
    'not an email',
    'not@email',
    'almost@email.com ',
    '.mail.com',
    'x@mail.',
  ])('returns true when email is not valid (%s)', email => {
    expect(
      checkIsInvalid(
        {
          id: 'some-id',
          name: 'some-name',
          toAddress: email,
          notificationType: NotificationTypeNode.Email,
        },
        null
      )
    ).toBeTruthy();
  });
  it('returns true when creating telegram recipient', () => {
    expect(
      checkIsInvalid(
        {
          id: 'some-id',
          name: 'some-name',
          toAddress: '-1234567',
          notificationType: NotificationTypeNode.Telegram,
        },
        ModalMode.Create
      )
    ).toBeTruthy();
  });
  it('returns false when updating telegram recipient', () => {
    expect(
      checkIsInvalid(
        {
          id: 'some-id',
          name: 'some-name',
          toAddress: '-1234567',
          notificationType: NotificationTypeNode.Telegram,
        },
        ModalMode.Update
      )
    ).toBeFalsy();
  });
});
