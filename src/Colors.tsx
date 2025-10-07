import { type RGBA, rgbToHex } from "@opentui/core";

export function Colors({ colorsPalette }: { colorsPalette: RGBA[] }) {
  return (<>
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
  )

}

