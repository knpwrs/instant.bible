import * as React from 'react';
import { I18n } from '@lingui/react';
import { t } from '@lingui/macro';
import { css } from '@emotion/core';
import { clamp } from 'lodash';
import copy from 'copy-text-to-clipboard';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faCopy } from '@fortawesome/free-solid-svg-icons';
import styled from '../util/styled';
import highlightUtil from '../util/highlight';
import {
  Button,
  Card,
  H5,
  Body3,
  Subhead3Medium,
  Body3Highlight,
} from '../elements';

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

  const handleCopy = React.useCallback(() => {
    copy(`${title} ${selectedKey}\n${text}`);
  }, [title, text, selectedKey]);

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
        <I18n>
          {({ i18n }) => (
            <Button
              onClick={handleCopy}
              title={i18n._(t`Copy`)}
              css={css`
                margin-left: auto;
                width: 16px;
                height: 16px;
                cursor: pointer;
              `}
            >
              <FontAwesomeIcon
                icon={faCopy}
                css={css`
                  width: 16px;
                  height: 16px;
                `}
              />
            </Button>
          )}
        </I18n>
      </div>
    </Card>
  );
};

export default Verse;
