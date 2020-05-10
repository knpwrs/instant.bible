import * as React from 'react';
import { createPortal } from 'react-dom';
import { noop } from 'lodash';
import { css, keyframes } from '@emotion/core';
import { I18n } from '@lingui/react';
import { t } from '@lingui/macro';
import { faCopy } from '@fortawesome/free-solid-svg-icons';
import { usePopper } from 'react-popper';
import copy from 'copy-text-to-clipboard';
import { IconButton, Text } from '../elements';

export type Props = {
  copyText: string;
} & Pick<React.HTMLProps<HTMLButtonElement>, 'className'>;

const easeOutExpo = `cubic-bezier(0.19, 1, 0.22, 1)`;
const copiedFloat = keyframes`
  0% {
    opacity: 0;
    transform: translate3d(0, 16px, 0);
  }
  10% {
    opacity: 1;
    transform: translate3d(0, 0, 0);
  }
  100% {
    opacity: 0;
    transform: translate3d(0, -16px, 0);
  }
`;

const CopyButton = ({ copyText, className }: Props) => {
  const [btnEl, setBtnEl] = React.useState<HTMLButtonElement | null>(null);
  const [popperEl, setPopperEl] = React.useState<HTMLDivElement | null>(null);
  const [popperRoot, setPopperRoot] = React.useState<HTMLDivElement | null>(
    null,
  );
  const [copying, setCopying] = React.useState(false);

  const handleCopy = React.useCallback(() => {
    copy(copyText);
    setCopying(true);
  }, [copyText]);

  React.useEffect(() => {
    if (copying) {
      const root = document.createElement('div');
      document.body.appendChild(root);
      setPopperRoot(root);

      return () => {
        document.body.removeChild(root);
      };
    }

    return noop;
  }, [copying]);

  React.useEffect(() => {
    if (popperEl) {
      popperEl.addEventListener('animationend', () => {
        setCopying(false);
      });
    }

    return noop;
  }, [popperEl]);

  const { styles: popperStyles } = usePopper(btnEl, popperEl, {
    placement: 'top',
  });

  return (
    <I18n>
      {({ i18n }) => (
        <>
          {popperRoot
            ? createPortal(
                <div ref={setPopperEl} style={popperStyles.popper}>
                  <Text
                    css={css`
                      display: inline-block;
                      font-size: 12px;
                      animation: ${copiedFloat};
                      animation-duration: 500ms;
                      animation-iteration-count: 1;
                      animation-timing-function: ${easeOutExpo};
                      animation-fill-mode: forwards;
                    `}
                  >
                    {i18n._(t`copied!`)}
                  </Text>
                </div>,
                popperRoot,
              )
            : null}
          <IconButton
            ref={setBtnEl}
            className={className}
            icon={faCopy}
            onClick={handleCopy}
            title={i18n._(t`Copy`)}
          />
        </>
      )}
    </I18n>
  );
};

export default React.memo(CopyButton);
