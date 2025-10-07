import { hexToRgb, RGBA, rgbToHex, TextAttributes } from "@opentui/core";
import { render, useKeyboard } from "@opentui/react";
import { useReducer, useState, type ActionDispatch, type AnyActionArg } from "react";
import { randomColor } from "./fuctions";
import { Colors } from "./Colors";
import type { ControlWheelProp, ActionArgs } from "./types";

function App() {
  const [colorsPalette, setColorsPalette] = useState<RGBA[]>([]);
  const [displaySaveButton, setDisplaySaveButton] = useState<boolean>();
  const [countColorsPalette, setCountColorsPalette] = useState<number>(3);

  const [state, dispatch] = useReducer(colorReducer, { pause: true, timeout: null, displayColors: false })


  function colorReducer(state: ControlWheelProp, action: ActionArgs) {
    if (action.type === "pauseColorWheel")
      return {
        ...state,
        pause: true,
        timeout: state.timeout?.close()
      }
    else if (action.type === "startColorWheel" || action.type === "continueColorWheel")
      return {
        timeout: randomColor(countColorsPalette, setColorsPalette),
        displayColors: true,
        pause: false,
      }

  }


  useKeyboard((key) => {
    if (key.name === "space") {
      dispatch({ type: "pauseColorWheel" })
    }
    else if (key.name === "s") {
      if (!state.displayColors)
        dispatch({ type: "startColorWheel" })
    }
    else if (key.name === "c") {
      if (state.displayColors && state.pause)
        dispatch({ type: "continueColorWheel" })

    }
  });

  return (
    <box alignItems="center" justifyContent="center" flexGrow={1}>
      <box justifyContent="center" alignItems="flex-end">
        <ascii-font font="tiny" text="Color Hunter :D" />
        <text attributes={TextAttributes.DIM} fg="#7c86ff">
          You’ll find what you need
        </text>
      </box>

      {!state.displayColors && (
        <box width={50} justifyContent="center" alignItems="center">
          <box
            backgroundColor={"white"}
            padding={1}
            paddingLeft={4}
            paddingRight={4}
            marginTop={1}
          >
            <text fg="#000000">[S]tart</text>
          </box>
        </box>
      )}
      {state.displayColors && (
        <Colors colorsPalette={colorsPalette} />
      )}
      {state.pause && state.displayColors ? (
        <box flexDirection="row">
          <box
            backgroundColor="#000000"
            padding={1}
            paddingLeft={4}
            paddingRight={4}
            marginTop={1}
          >
            <text fg="#ffffff">[C]ontinue</text>
          </box>
          <box
            backgroundColor={"#000000"}
            padding={1}
            paddingLeft={4}
            paddingRight={4}
            marginTop={1}
          >
            <text fg="#ffffff">[S]ave palette</text>
          </box>

        </box>
      ) : null}
      {!state.pause && state.displayColors ? (
        <box
          backgroundColor="#000000"
          padding={1}
          paddingLeft={4}
          paddingRight={4}
          marginTop={1}
        >
          <text fg="#ffffff">Press [space] to stop</text>
        </box>
      ) : null}
    </box>
  );
}

render(<App />);
