import * as React from 'react';
import { axe } from 'jest-axe';
import { noop } from 'lodash';
import { fireEvent } from '@testing-library/react';
import Verse, { Props as VerseProps } from './verse';
import data from './__mocks__/verse-data';
import render from '../../opt/test-render';

const renderVerse = ({
  onSelectKey = noop,
  selectedKey = 'KJV',
  highlight = [],
}: Partial<VerseProps> = {}): ReturnType<typeof render> =>
  render(
    <Verse
      title="John 3:16"
      data={data}
      selectedKey={selectedKey}
      onSelectKey={onSelectKey}
      highlight={highlight}
    />,
  );

test('a11y', async () => {
  const { container } = renderVerse();
  expect(await axe(container)).toHaveNoViolations();
});

test('renders text with highlights', async () => {
  const { getByText } = renderVerse({ highlight: ['love'] });
  const title = getByText('John 3:16');
  expect(title).toBeInTheDocument();
  const mark = getByText('love');
  expect(mark).toBeInTheDocument();
  expect(mark).toHaveTextContent('love');
  expect((mark as HTMLElement).tagName).toBe('MARK');
});

test('responds to keydown', () => {
  const spy = jest.fn();
  const { getByText } = renderVerse({ onSelectKey: spy });
  const title = getByText('John 3:16');
  expect(title).toBeInTheDocument();
  if (title instanceof HTMLElement) {
    fireEvent.keyDown(title, { key: 'h' });
    fireEvent.keyDown(title, { key: 'l' });
    fireEvent.keyDown(title, { key: 'ArrowLeft' });
    fireEvent.keyDown(title, { key: 'ArrowRight' });
  }
  expect(spy).toBeCalledTimes(4);
  expect(spy).toHaveBeenNthCalledWith(1, 'KJV');
  expect(spy).toHaveBeenNthCalledWith(2, 'NET');
  expect(spy).toHaveBeenNthCalledWith(3, 'KJV');
  expect(spy).toHaveBeenNthCalledWith(4, 'NET');
});
