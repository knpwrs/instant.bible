type SvgrComponent = React.StatelessComponent<React.SVGAttributes<SVGElement>>;

declare module '*.svg' {
  const value: SvgrComponent;
  export default value;
}

declare module '*.png' {
  const value: string;
  export default value;
}

declare module 'worker-loader!*' {
  // You need to change `Worker`, if you specified a different value for the `workerType` option
  class WebpackWorker extends Worker {
    constructor();
  }

  export default WebpackWorker;
}
