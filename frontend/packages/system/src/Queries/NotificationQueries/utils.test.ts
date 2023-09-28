import { stringifyObjectKey } from './utils';

describe('stringifyObjectKey', () => {
  it('returns strings unchanged', () => {
    const testString = 'Hello';
    expect(stringifyObjectKey(testString)).toEqual(testString);
  });

  it('returns numbers unchanged', () => {
    const testNumber = 123;
    expect(stringifyObjectKey(testNumber)).toEqual(testNumber);
  });

  it('stringifies objects', () => {
    const testObject = { foo: 'bar' };
    expect(stringifyObjectKey(testObject)).toEqual(JSON.stringify(testObject));
  });

  it('returns true for boolean true', () => {
    expect(stringifyObjectKey(true)).toEqual('true');
  });

  it('returns false for boolean false', () => {
    expect(stringifyObjectKey(false)).toEqual('false');
  });
});
