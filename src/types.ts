  export interface ControlWheelProp  {
    pause: boolean,
    timeout: NodeJS.Timeout | null
    displayColors: boolean
  }
  export type ActionArgs = { type: "pauseColorWheel" } | { type: "startColorWheel" } | { type: "continueColorWheel" }

