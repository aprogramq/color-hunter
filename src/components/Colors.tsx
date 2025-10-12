import { type RGBA, rgbToHex } from "@opentui/core";

export function Palette({ colorsPalette, position, width, count }: { colorsPalette: RGBA[][], position:number, width:number, count:number }) {
  const colorsDisplay = [];

  for (let i = 0; i < count; i++)
    colorsDisplay.push(
      <box
        justifyContent="center"
        alignItems="center"
        backgroundColor={colorsPalette[position]![i]!}
        width="10%"
        height="60%"
      >
        <text justifyContent="center">
          {colorsPalette[position]![i] && rgbToHex(colorsPalette[position]![i]!)}
        </text>
      </box>,
    );
  return (
    <>
      <box
        flexDirection="row"
        height={10}
        width={160}
        justifyContent="center"
        alignItems="center"
      >
        {colorsDisplay}
      </box>
    </>
  );
}
