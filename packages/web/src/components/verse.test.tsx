import * as React from 'react';
import { axe } from 'jest-axe';
import { fireEvent } from '@testing-library/react';
import Verse, { Props as VerseProps } from './verse';
import { john316 } from './__mocks__/verse-data';
import render from '../opt/test-render';
import * as proto from '../proto';

const renderVerse = ({
  topTranslation = proto.instantbible.data.Translation.KJV,
  highlight = [],
}: Partial<VerseProps> = {}): ReturnType<typeof render> =>
  render(
    <Verse
      title="John 3:16"
      data={john316}
      topTranslation={topTranslation}
      highlight={highlight}
      verseKey={{}}
    />,
  );

test('a11y', async () => {
  const { container } = renderVerse();
  expect(await axe(container)).toHaveNoViolations();
});

test('renders text with highlights', async () => {
  const { getByText, getAllByText } = renderVerse({ highlight: ['love'] });
  const title = getByText('John 3:16');
  expect(title).toBeInTheDocument();
  const marks = getAllByText('love');
  expect(marks).toHaveLength(3);
  marks.forEach((mark) => {
    expect(mark).toBeInTheDocument();
    expect(mark).toHaveTextContent('love');
    expect((mark as HTMLElement).tagName).toBe('MARK');
  });
});

test('responds to keydown', () => {
  const { getByRole } = renderVerse();
  const tablist = getByRole('tablist');
  expect(tablist).toBeInTheDocument();

  let selected = getByRole('tab', { selected: true });
  expect(selected).toHaveTextContent('KJV');
  if (tablist instanceof HTMLElement) {
    fireEvent.keyDown(tablist, { key: 'ArrowLeft' });
    selected = getByRole('tab', { selected: true });
    expect(selected).toHaveTextContent('BSB');
    fireEvent.keyDown(tablist, { key: 'ArrowRight' });
    selected = getByRole('tab', { selected: true });
    expect(selected).toHaveTextContent('KJV');
  }
});
