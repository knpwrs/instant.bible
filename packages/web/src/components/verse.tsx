import * as React from 'react';
import { css } from '@emotion/core';
import { clamp, sortBy } from 'lodash';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDove } from '@fortawesome/free-solid-svg-icons';
import { Trans } from '@lingui/macro';
import CopyButton from './copy-button';
import OpenExternalButton from './open-external-button';
import styled from '../util/styled';
import highlightUtil from '../util/highlight';
import {
  Card,
  H5,
  Body3,
  Body3Bold,
  Subhead3Medium,
  Body3Highlight,
} from '../elements';
import * as proto from '../proto';
import { translationToString } from '../util/proto';

export type OwnProps = {
  title: string;
  data: Record<string, string>;
  topTranslation: proto.instantbible.data.Translation;
  highlight: string[];
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
  const idx = haystack.findIndex((e) => e === needle);
  const nidx = clamp(idx + 1, 0, haystack.length - 1);

  return haystack[nidx];
};

const getPrev = (
  haystack: Array<proto.instantbible.data.Translation>,
  needle: proto.instantbible.data.Translation,
): proto.instantbible.data.Translation => {
  const idx = haystack.findIndex((e) => e === needle);
  const pidx = clamp(idx - 1, 0, haystack.length - 1);

  return haystack[pidx];
};

const translationKeys = sortBy(
  Object.values(proto.instantbible.data.Translation).filter(
    (i) => i !== proto.instantbible.data.Translation.TOTAL,
  ),
  (t) => translationToString(t),
) as Array<proto.instantbible.data.Translation>;

const Verse: React.FunctionComponent<Props> = ({
  title,
  data,
  topTranslation,
  highlight,
  tabIndex = 0,
  className,
  verseKey,
}) => {
  const [selectedTranslation, setSelectedTranslation] = React.useState(
    topTranslation,
  );

  const handleKeyDown = React.useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === 'h' || e.key === 'ArrowLeft') {
        e.preventDefault();
        setSelectedTranslation(getPrev(translationKeys, selectedTranslation));
      } else if (e.key === 'l' || e.key === 'ArrowRight') {
        e.preventDefault();
        setSelectedTranslation(getNext(translationKeys, selectedTranslation));
      }
    },
    [selectedTranslation],
  );

  const text = data[selectedTranslation];

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
      {text ? (
        <Body3>{highlightedText}</Body3>
      ) : (
        <Body3Bold
          secondary
          css={css`
            margin: 1em 0;
          `}
        >
          <FontAwesomeIcon
            icon={faDove}
            css={css`
              margin: 0 0.25em;
            `}
          />{' '}
          <Trans>
            This verse is not available in the{' '}
            {translationToString(selectedTranslation)} translation
          </Trans>
        </Body3Bold>
      )}
      <div
        css={css`
          display: flex;
          flex-direction: row;
          font-size: 16px;
        `}
      >
        <div role="tablist">
          {translationKeys.map((key) => (
            <Translation
              key={key}
              selected={key === selectedTranslation}
              aria-selected={key === selectedTranslation}
              role="tab"
              onClick={(): unknown => setSelectedTranslation(key)}
            >
              {translationToString(key)}
            </Translation>
          ))}
        </div>
        <CopyButton
          copyText={`${title} ${selectedTranslation}\n${text}`}
          css={css`
            margin-left: auto;
            margin-right: 10px;
          `}
        />
        <OpenExternalButton
          translation={selectedTranslation}
          verseKey={verseKey}
        />
      </div>
    </Card>
  );
};

export default Verse;
