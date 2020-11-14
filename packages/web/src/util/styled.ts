import styled, { CreateStyled } from '@emotion/styled';
import { css } from '@emotion/core';
import { AppTheme } from './theme';

export default styled as CreateStyled<AppTheme>;

export type ThemedFn<Props = {}> = (
  arg: Props & { theme: AppTheme },
) => string | ReturnType<typeof css>;
