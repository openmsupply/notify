import { Formatter } from './formatters';

describe('Formatter', () => {
  it('is defined', () => {
    expect(Formatter.expiryDate).toBeDefined();
    expect(Formatter.expiryDateString).toBeDefined();
    expect(Formatter.naiveDate).toBeDefined();
  });

  it('expiryDate', () => {
    expect(Formatter.expiryDate(null)).toBe(null);
    expect(Formatter.expiryDate(new Date('2022/01/20'))).toBe('01/2022');
  });

  it('expiryDateString', () => {
    expect(Formatter.expiryDateString(null)).toBe('');
    expect(Formatter.expiryDateString('oops')).toBe('');
    expect(Formatter.expiryDateString('2022/01/20')).toBe('01/2022');
  });

  it('naiveDate', () => {
    expect(Formatter.naiveDate(null)).toBe(null);
    expect(Formatter.naiveDate(new Date('1984/3/13'))).toBe('1984-03-13');
  });
});
