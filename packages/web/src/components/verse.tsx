import * as React from 'react';
import { css } from '@emotion/core';
import { clamp } from 'lodash';
import CopyButton from './copy-button';
import OpenExternalButton from './open-external-button';
import styled from '../util/styled';
import highlightUtil from '../util/highlight';
import { Card, H5, Body3, Subhead3Medium, Body3Highlight } from '../elements';
import * as proto from '../proto';

export type OwnProps = {
  title: string;
  data: { [key: string]: string };
  selectedTranslationKey: string;
  highlight: string[];
  onSelectKey: (key: string) => unknown;
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

const getNext = (haystack: string[], needle: string): string => {
  const idx = haystack.findIndex(e => e === needle);
  const nidx = clamp(idx + 1, 0, haystack.length - 1);

  return haystack[nidx];
};

const getPrev = (haystack: string[], needle: string): string => {
  const idx = haystack.findIndex(e => e === needle);
  const pidx = clamp(idx - 1, 0, haystack.length - 1);

  return haystack[pidx];
};

const Verse: React.FunctionComponent<Props> = ({
  title,
  data,
  selectedTranslationKey: selectedKey,
  highlight,
  onSelectKey,
  tabIndex = 0,
  className,
  verseKey,
}) => {
  const translationKeys = Object.keys(data).sort();

  const handleKeyDown = React.useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === 'h' || e.key === 'ArrowLeft') {
        e.preventDefault();
        onSelectKey(getPrev(translationKeys, selectedKey));
      } else if (e.key === 'l' || e.key === 'ArrowRight') {
        e.preventDefault();
        onSelectKey(getNext(translationKeys, selectedKey));
      }
    },
    [translationKeys, selectedKey, onSelectKey],
  );

  const text = data[selectedKey];

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
        `}
      >
        {translationKeys.map(key => (
          <Translation
            key={key}
            selected={key === selectedKey}
            onClick={(): unknown => onSelectKey(key)}
          >
            {key}
          </Translation>
        ))}
        <CopyButton
          copyText={`${title} ${selectedKey}\n${text}`}
          css={css`
            margin-left: auto;
            margin-right: 10px;
          `}
        />
        <OpenExternalButton
          // @ts-ignore: TODO use more protobuf-native types?
          translation={proto.instantbible.data.Translation[selectedKey]}
          verseKey={verseKey}
        />
      </div>
    </Card>
  );
};

export default Verse;
