import * as React from 'react';
import { css } from '@emotion/core';
import { I18n } from '@lingui/react';
import { t } from '@lingui/macro';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faCopy } from '@fortawesome/free-solid-svg-icons';
import copy from 'copy-text-to-clipboard';
import { Button } from '../elements';

export type Props = {
  copyText: string;
} & Pick<React.HTMLProps<HTMLButtonElement>, 'className'>;

const CopyButton = ({ copyText, className }: Props) => {
  const handleCopy = React.useCallback(() => {
    copy(copyText);
  }, [copyText]);

  return (
    <I18n>
      {({ i18n }) => (
        <Button
          className={className}
          onClick={handleCopy}
          title={i18n._(t`Copy`)}
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
  );
};

export default React.memo(CopyButton);
