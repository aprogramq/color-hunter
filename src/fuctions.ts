import { hexToRgb, hsvToRgb, RGBA, rgbToHex } from "@opentui/core";
import { sleepSync } from "bun";
import { cc } from "bun:ffi";
import randomInteger from "random-int";
import type { Dispatch, SetStateAction } from "react";
import { createCanvas } from "canvas";
import fs from "fs";

type UseState<T> = Dispatch<SetStateAction<T>>;

export function load(setLoaderValue: UseState<string>) {
  let currentPosition: number = 0;
  const loadStadies = ["|", "/", "-"];
  const loaderSpiner = setInterval(() => {
    setLoaderValue(`Loading ${loadStadies[currentPosition]}`);
    if (currentPosition == 2) currentPosition = 0;
    else currentPosition += 1;
  }, 300);
  return loaderSpiner;

}
export function randomColor(count: number, setColorsPalette: UseState<RGBA[]>): NodeJS.Timeout {
  let newColors: RGBA[] = [];
  const colorInterval = setInterval(() => {
    for (let i = 0; i < count; i++) {
      const randomDecimal = randomInteger(0, 16777215);
      // const hueHsv = randomInteger(0,255);
      // const saturationHsv = 100;
      // const brightnessHsv = 50;
      newColors.push(hexToRgb(randomDecimal.toString(16)));
      // newColors.push(hsvToRgb(hueHsv, saturationHsv, brightnessHsv))
    }
    setColorsPalette(newColors);
    newColors = [];
  }, 300);
  return colorInterval;
}

export function savePalette(colorPalette: RGBA[], countColors: number) {
  const imagePalette = createCanvas(500, 500, "svg");
  const cntx = imagePalette.getContext("2d");

  let xPosition: number = 80;
  for (let i = 0; i < countColors; i++) {

    cntx.fillStyle = rgbToHex(colorPalette[i]!);
    cntx.fillRect(i * 170, 150, 170, 200);
    cntx.strokeStyle = "white"

    cntx.textAlign = "center"
    cntx.textBaseline = "middle"

    console.log(xPosition)
    cntx.strokeText(`${rgbToHex(colorPalette[i]!)}`, xPosition, 250, 50);
    xPosition += 170

  }

  const buffer = imagePalette.toBuffer("image/png");
  fs.writeFileSync("./image.png", buffer)


}
