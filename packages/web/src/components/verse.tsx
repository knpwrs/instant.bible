import * as React from 'react';
import { css } from '@emotion/core';
import { clamp, sortBy } from 'lodash';
import CopyButton from './copy-button';
import OpenExternalButton from './open-external-button';
import styled from '../util/styled';
import highlightUtil from '../util/highlight';
import { Card, H5, Body3, Subhead3Medium, Body3Highlight } from '../elements';
import * as proto from '../proto';
import { translationToString } from '../util/proto';

export type OwnProps = {
  title: string;
  data: { [key: string]: string };
  selectedTranslationKey: proto.instantbible.data.Translation;
  highlight: string[];
  onSelectKey: (key: proto.instantbible.data.Translation) => unknown;
  verseKey: proto.instantbible.data.IVerseKey;
};

export type Props = Omit<React.HTMLProps<HTMLDivElement>, 'data'> & OwnProps;

const Translation = styled(Subhead3Medium.withComponent('button'))<{
  selected?: boolean;
}>`
  margin-right: 5px;
  border: none;
  background: none;
  padding: 0;

  ${({ selected }): null | ReturnType<typeof css> =>
    selected
      ? null
      : css`
          opacity: 0.65;
        `};
`;

const getNext = (
  haystack: Array<proto.instantbible.data.Translation>,
  needle: proto.instantbible.data.Translation,
): proto.instantbible.data.Translation => {
  const idx = haystack.findIndex(e => e === needle);
  const nidx = clamp(idx + 1, 0, haystack.length - 1);

  return haystack[nidx];
};

const getPrev = (
  haystack: Array<proto.instantbible.data.Translation>,
  needle: proto.instantbible.data.Translation,
): proto.instantbible.data.Translation => {
  const idx = haystack.findIndex(e => e === needle);
  const pidx = clamp(idx - 1, 0, haystack.length - 1);

  return haystack[pidx];
};

const translationKeys = sortBy(
  Object.values(proto.instantbible.data.Translation).filter(
    i => i !== proto.instantbible.data.Translation.TOTAL,
  ),
  t => translationToString(t),
) as Array<proto.instantbible.data.Translation>;

const Verse: React.FunctionComponent<Props> = ({
  title,
  data,
  selectedTranslationKey,
  highlight,
  onSelectKey,
  tabIndex = 0,
  className,
  verseKey,
}) => {
  const handleKeyDown = React.useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === 'h' || e.key === 'ArrowLeft') {
        e.preventDefault();
        onSelectKey(getPrev(translationKeys, selectedTranslationKey));
      } else if (e.key === 'l' || e.key === 'ArrowRight') {
        e.preventDefault();
        onSelectKey(getNext(translationKeys, selectedTranslationKey));
      }
    },
    [selectedTranslationKey, onSelectKey],
  );

  const text = data[selectedTranslationKey];

  const chunks = React.useMemo(() => highlightUtil(text, highlight), [
    text,
    highlight,
  ]);

  const highlightedText = React.useMemo(
    () =>
      chunks.map(({ text, highlight }, i) => {
        const key = `${text}-${highlight}-${i}`;
        if (highlight) {
          return <Body3Highlight key={key}>{text}</Body3Highlight>;
        }
        return <React.Fragment key={key}>{text}</React.Fragment>;
      }),
    [chunks],
  );

  return (
    <Card
      className={className}
      css={css`
        width: 960px;
      `}
      tabIndex={tabIndex}
      onKeyDown={handleKeyDown}
    >
      <H5
        css={css`
          margin: 0;
        `}
      >
        {title}
      </H5>
      <Body3>{highlightedText}</Body3>
      <div
        css={css`
          display: flex;
          flex-direction: row;
          font-size: 16px;
        `}
      >
        {translationKeys.map(key => (
          <Translation
            key={key}
            selected={key === selectedTranslationKey}
            onClick={(): unknown => onSelectKey(key)}
          >
            {translationToString(key)}
          </Translation>
        ))}
        <CopyButton
          copyText={`${title} ${selectedTranslationKey}\n${text}`}
          css={css`
            margin-left: auto;
            margin-right: 10px;
          `}
        />
        <OpenExternalButton
          translation={selectedTranslationKey}
          verseKey={verseKey}
        />
      </div>
    </Card>
  );
};

export default Verse;
