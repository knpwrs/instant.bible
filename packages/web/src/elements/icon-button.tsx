import * as React from 'react';
import {
  FontAwesomeIcon,
  FontAwesomeIconProps,
} from '@fortawesome/react-fontawesome';
import { css } from '@emotion/core';
import { Button } from './typography';

export type Props = Pick<
  React.HTMLProps<HTMLButtonElement>,
  'className' | 'onClick' | 'title'
> &
  Pick<FontAwesomeIconProps, 'icon'>;

const IconButton = React.forwardRef<HTMLButtonElement, Props>(
  ({ className, onClick, title, icon }, ref) => {
    return (
      <Button
        ref={ref}
        className={className}
        onClick={onClick}
        title={title}
        css={css`
          cursor: pointer;
        `}
      >
        <FontAwesomeIcon
          icon={icon}
          css={css`
            width: 16px;
            height: 16px;
          `}
        />
      </Button>
    );
  },
);

export default React.memo(IconButton);
