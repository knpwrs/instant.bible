import * as React from 'react';
import { css } from '@emotion/core';
import { storiesOf } from '@storybook/react';
import Card from './card';

storiesOf('elements/card', module).add('basic', () => {
  return (
    <Card
      tabIndex={0}
      css={css`
        width: 50%;
        height: 25%;
      `}
    />
  );
});
