import styled, { CreateStyled } from '@emotion/styled';
import { css } from '@emotion/core';
import { Theme } from './theme';

export default styled as CreateStyled<Theme>;

export type ThemedFn<Props = {}> = (
  arg: Props & { theme: Theme },
) => string | ReturnType<typeof css>;
