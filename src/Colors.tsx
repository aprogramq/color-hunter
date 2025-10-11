import { type RGBA, rgbToHex } from "@opentui/core";
import { memo, useId, useState } from "react";

export function Palette({ colorsPalette, position }: { colorsPalette: RGBA[][], position:number }) {
  const colorsDisplay = [];

  for (let i = 0; i < 3; i++)
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
        width="100%"
        justifyContent="center"
        alignItems="center"
      >
        {colorsDisplay}
      </box>
    </>
  );
}
