import { hexToRgb, RGBA, rgbToHex, TextAttributes } from "@opentui/core";
import { render, useKeyboard } from "@opentui/react";
import { useState } from "react";
import { cc } from "bun:ffi";
import { helloName, load, randomColor } from "./fuctions";
import { sleepSync } from "bun";
import randomInteger from "random-int";

function App() {
  const [input, setInput] = useState<string>("");
  const [displayName, setDisplayName] = useState<boolean>(false);
  const [loaderValue, setLoaderValue] = useState<string>("");
  const [colorsPalette, setColorsPalette] = useState<RGBA[]>([]);
  const [timeout, setTimeout] = useState<NodeJS.Timeout>();
  const [pause, setPause] = useState<boolean>(false);
  const [displaySaveButton, setDisplaySaveButton] = useState<boolean>();
  const [countColorsPalette, setCountColorsPalette] = useState<number>(3);

  useKeyboard((key) => {
    if (key.name === "space") {
      timeout?.refresh();
    }
    if (key.name === "escape") {
      timeout?.unref();
    }
    if (key.name === "s") {

      setTimeout(randomColor(countColorsPalette, setColorsPalette));
      setDisplayName(true);
    }
  });

  return (
    <box alignItems="center" justifyContent="center" flexGrow={1}>
      <box justifyContent="center" alignItems="flex-end">
        <ascii-font font="tiny" text="Color Hunter :D" />
        <text attributes={TextAttributes.DIM} fg="#7c86ff">
          you’ll find what you need
        </text>
      </box>
      {!displayName && (
        <box width={50} justifyContent="center" alignItems="center">
          {/* <input */}
          {/*   marginTop={1} */}
          {/*   paddingTop={2} */}
          {/*   textColor="white" */}
          {/*   backgroundColor="#7c86ff" */}
          {/*   width="70%" */}
          {/*   placeholder="Descrbe feeling your will palette" */}
          {/*   focused */}
          {/*   value={input} */}
          {/*   onInput={(value) => { */}
          {/*     setInput(value); */}
          {/*     // setDisplayName(false); */}
          {/*   }} */}
          {/*   onSubmit={() => { */}
          {/*     setTimeout(randomColor(countColorsPalette, setColorsPalette)); */}
          {/*     setDisplayName(true); */}
          {/*   }} */}
          {/* /> */}
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
      {displayName && (
        <>
          <box
            flexDirection="row"
            height={10}
            width={"auto"}
            justifyContent="center"
            alignItems="center"
          >
            {colorsPalette.map((color) => (
              <box
                justifyContent="center"
                alignItems="center"
                backgroundColor={color}
                width="10%"
                height="60%"
              >
                <text justifyContent="center">{color && rgbToHex(color)}</text>
              </box>
            ))}
          </box>
        </>
      )}
    </box>
  );
}

render(<App />);
