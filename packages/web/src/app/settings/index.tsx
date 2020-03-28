import * as React from 'react';
import { css } from '@emotion/core';
import SettingsModal from './modal';
import { Gear } from '../../elements';

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
    <>
      <button
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
      >
        <Gear />
      </button>
      {open ? <SettingsModal onClose={handleToggleOpen} /> : null}
    </>
  );
});
