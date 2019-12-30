import * as React from 'react';
import { css } from '@emotion/core';
import { findAll } from 'highlight-words-core';
import { clamp } from 'lodash';
import styled from '../util/styled';
import { Card, H5, Body3, Subhead3Medium, Body3Highlight } from '../elements';

export type OwnProps = {
  title: string;
  data: { [key: string]: string };
  selectedKey: string;
  highlight: string[];
  onSelectKey: (key: string) => unknown;
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
  selectedKey,
  highlight,
  onSelectKey,
  tabIndex = 0,
  className,
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

  const chunks = React.useMemo(
    () =>
      findAll({
        textToHighlight: text,
        searchWords: highlight,
      }),
    [text, highlight],
  );

  const highlightedText = React.useMemo(
    () =>
      chunks.map(({ highlight, start, end }) => {
        const key = `${start}-${end}-${highlight}`;
        const chunk = text.substr(start, end - start);
        if (highlight) {
          return <Body3Highlight key={key}>{chunk}</Body3Highlight>;
        }
        return <React.Fragment key={key}>{chunk}</React.Fragment>;
      }),
    [chunks, text],
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
      <div>
        {translationKeys.map(key => (
          <Translation
            key={key}
            selected={key === selectedKey}
            onClick={(): unknown => onSelectKey(key)}
          >
            {key}
          </Translation>
        ))}
      </div>
    </Card>
  );
};

export default Verse;
