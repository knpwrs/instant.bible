import * as React from 'react';
import { createPortal } from 'react-dom';
import styled from '@emotion/styled';

const Facade = styled.div`
  width: 100vw;
  height: 100vh;
  position: fixed;
  left: 0;
  top: 0;
  background: ${(props) => props.theme.facade};
  z-index: 3;
  display: flex;
  justify-content: center;
  align-items: center;
`;

export type Props = React.PropsWithChildren<{ onClose: () => unknown }>;

export default ({ children, onClose }: Props) => {
  const [root, setRoot] = React.useState<null | HTMLDivElement>(null);

  React.useEffect(() => {
    const div = document.createElement('div');
    document.body.appendChild(div);
    setRoot(div);

    return () => {
      document.body.removeChild(div);
    };
  }, [setRoot]);

  return (
    root && createPortal(<Facade onClick={onClose}>{children}</Facade>, root)
  );
};
