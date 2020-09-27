import * as React from 'react';
import { css } from '@emotion/core';
import { clamp, sortBy } from 'lodash';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDove } from '@fortawesome/free-solid-svg-icons';
import { Trans } from '@lingui/macro';
import { Tabs, TabList, Tab, TabPanels, TabPanel } from '@reach/tabs';
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

const TranslationTab = styled(Subhead3Medium.withComponent(Tab))`
  margin-right: 5px;
  border: none;
  background: none;
  padding: 0;

  &:not([data-selected]) {
    opacity: 0.65;
  }
`;

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
  const topTranslationIdx = React.useMemo(
    () =>
      clamp(
        translationKeys.findIndex((e) => e === topTranslation),
        0,
        translationKeys.length - 1,
      ),
    [topTranslation],
  );

  const chunkGroups = React.useMemo(
    () =>
      translationKeys.map((key) => {
        const text = data[key];
        if (!text) {
          return null;
        }

        return highlightUtil(data[key], highlight);
      }),
    [data, highlight],
  );

  const highlightedChunks = React.useMemo(
    () =>
      chunkGroups.map((chunks) =>
        chunks?.map(({ text, highlight }, i) => {
          const key = `${text}-${highlight}-${i}`;

          if (highlight) {
            return <Body3Highlight key={key}>{text}</Body3Highlight>;
          }

          return <React.Fragment key={key}>{text}</React.Fragment>;
        }),
      ),
    [chunkGroups],
  );

  return (
    <Card
      className={className}
      css={css`
        width: 960px;
      `}
      tabIndex={tabIndex}
    >
      <Tabs defaultIndex={topTranslationIdx}>
        {({ selectedIndex }: { selectedIndex: number }) => (
          <React.Fragment>
            <H5
              css={css`
                margin: 0;
              `}
            >
              {title}
            </H5>
            <TabPanels>
              {highlightedChunks.map((chunk, i) => (
                <TabPanel key={translationKeys[i]}>
                  {chunk ? (
                    <Body3>{chunk}</Body3>
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
                        {translationToString(selectedIndex)} translation
                      </Trans>
                    </Body3Bold>
                  )}
                </TabPanel>
              ))}
            </TabPanels>
            <div
              css={css`
                display: flex;
                flex-direction: row;
                font-size: 16px;
              `}
            >
              <TabList>
                {translationKeys.map((key) => (
                  <TranslationTab key={key}>
                    {translationToString(key)}
                  </TranslationTab>
                ))}
              </TabList>
              <CopyButton
                copyText={`${title} ${translationToString(
                  translationKeys[selectedIndex],
                )}\n${data[translationKeys[selectedIndex]]}`}
                css={css`
                  margin-left: auto;
                  margin-right: 10px;
                `}
              />
              <OpenExternalButton
                translation={translationKeys[selectedIndex]}
                verseKey={verseKey}
              />
            </div>
          </React.Fragment>
        )}
      </Tabs>
    </Card>
  );
};

export default Verse;
