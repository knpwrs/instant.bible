import * as React from 'react';
import { axe } from 'jest-axe';
import { fireEvent } from '@testing-library/react';
import Verse, { Props as VerseProps } from './verse';
import data from './__mocks__/verse-data';
import render from '../opt/test-render';
import * as proto from '../proto';

const renderVerse = ({
  topTranslation = proto.instantbible.data.Translation.KJV,
  highlight = [],
}: Partial<VerseProps> = {}): ReturnType<typeof render> =>
  render(
    <Verse
      title="John 3:16"
      data={data}
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
  const { getByText } = renderVerse({ highlight: ['love'] });
  const title = getByText('John 3:16');
  expect(title).toBeInTheDocument();
  const mark = getByText('love');
  expect(mark).toBeInTheDocument();
  expect(mark).toHaveTextContent('love');
  expect((mark as HTMLElement).tagName).toBe('MARK');
});

test('responds to keydown', () => {
  const { getByText, getByRole } = renderVerse();
  const title = getByText('John 3:16');
  expect(title).toBeInTheDocument();

  let selected = getByRole('tab', { selected: true });
  expect(selected).toHaveTextContent('KJV');
  if (title instanceof HTMLElement) {
    fireEvent.keyDown(title, { key: 'h' });
    selected = getByRole('tab', { selected: true });
    expect(selected).toHaveTextContent('BSB');
    fireEvent.keyDown(title, { key: 'l' });
    selected = getByRole('tab', { selected: true });
    expect(selected).toHaveTextContent('KJV');
    fireEvent.keyDown(title, { key: 'ArrowLeft' });
    selected = getByRole('tab', { selected: true });
    expect(selected).toHaveTextContent('BSB');
    fireEvent.keyDown(title, { key: 'ArrowRight' });
    selected = getByRole('tab', { selected: true });
    expect(selected).toHaveTextContent('KJV');
  }
});
