import * as React from 'react';
import { I18n } from '@lingui/react';
import { t } from '@lingui/macro';
import { css } from '@emotion/core';
import { faCog } from '@fortawesome/free-solid-svg-icons';
import SettingsModal from './modal';
import { IconButton } from '../../elements';

export default React.memo(() => {
  const [open, setOpen] = React.useState(false);

  const handleToggleOpen = React.useCallback(
    (e?: React.MouseEvent) => {
      if (e) {
        e.preventDefault();
      }
      setOpen(o => !o);
    },
    [setOpen],
  );

  return (
    <I18n>
      {({ i18n }) => (
        <>
          <IconButton
            icon={faCog}
            css={css`
              position: fixed;
              top: 1em;
              right: 1em;
              opacity: 0.5;
              font-size: 1.3em;
              background: none;
              border: none;
              cursor: pointer;
              outline: none;
              &:hover {
                opacity: 1;
              }
            `}
            onClick={handleToggleOpen}
            title={i18n._(t`Settings`)}
          />
          {open ? <SettingsModal onClose={handleToggleOpen} /> : null}
        </>
      )}
    </I18n>
  );
});
