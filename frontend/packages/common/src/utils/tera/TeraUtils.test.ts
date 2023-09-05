import { TeraUtils } from './TeraUtils';

describe('TeraUtils', () => {
  it('Should extract a single param', () => {
    expect(TeraUtils.extractParams('Hello {{name}}!')).toEqual(['name']);
  });
  it('Should extract a single param with spaces ', () => {
    expect(TeraUtils.extractParams('Hello {{ name }}!')).toEqual(['name']);
  });
  it('Should not extract a broken parameter', () => {
    expect(TeraUtils.extractParams('Hello {{ name }!')).toEqual([]);
  });
  it('Should extract a parameter including a dot', () => {
    // In future we might want to parse this into a hierarchical structure? e.g. [{"data": [name]}]
    expect(TeraUtils.extractParams('Hello {{ data.name }}!')).toEqual([
      'data.name',
    ]);
  });
});
