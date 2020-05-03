import * as React from 'react';
import { useDispatch } from 'react-redux';
import PulseLoader from 'react-spinners/PulseLoader';
import { Trans } from '@lingui/macro';
import { css } from '@emotion/core';
import {
  useIndexBytesProgress,
  useOfflineEnabled,
  useLoading,
  doInitOffline,
} from '../../state/offline';
import Modal, { Props } from '../../components/modal';
import { Checkbox, Progress, H4, Body3 } from '../../elements';
import styled from '../../util/styled';
import { useTheme } from '../../util/theme';

const Container = styled('div')`
  width: 400px;
  height: 300px;
  border-radius: 20px;
  background: ${({ theme }) => theme.background};
  padding: 17px 20px;
`;

export default React.memo(({ onClose }: Props) => {
  const dispatch = useDispatch();
  const offlineEnabled = useOfflineEnabled();
  const loading = useLoading();
  const indexBytesProgress = useIndexBytesProgress();
  const theme = useTheme();

  const halt = React.useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
  }, []);

  const handleEnableOffline = React.useCallback(
    (o: boolean) => {
      dispatch(doInitOffline(o));
    },
    [dispatch],
  );

  return (
    <Modal onClose={onClose}>
      <Container
        onClick={halt}
        css={css`
          width: 400px;
          height: 300px;
        `}
      >
        <H4>
          <Trans>Settings</Trans>
        </H4>
        <Checkbox
          value={offlineEnabled}
          disabled={loading}
          onChange={handleEnableOffline}
        >
          <div
            css={css`
              display: flex;
              flex-direction: row;
              justify-content: space-between;
              align-items: center;
            `}
          >
            <div>
              <Trans>Enable Offline Mode</Trans>
            </div>
            {loading ? <PulseLoader color={theme.text.color} size={8} /> : null}
          </div>
        </Checkbox>
        <Body3
          secondary={true}
          css={css`
            padding-left: 44px;
            margin-top: 0;
          `}
        >
          <Trans>
            Store the search index locally and search without making web
            requests (i.e., make instant.bible even <em>instanter</em>).
          </Trans>
        </Body3>
        {offlineEnabled ? <Progress value={indexBytesProgress} /> : null}
      </Container>
    </Modal>
  );
});
