import { hexToRgb, hsvToRgb, RGBA, rgbToHex } from "@opentui/core";
import randomInteger from "random-int";
import type { Dispatch, SetStateAction } from "react";
import { Canvas, createCanvas, GlobalFonts} from "@napi-rs/canvas";
import fs, { writeFileSync } from "fs";
import { isExpressionWithTypeArguments } from "typescript";

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
export function randomColor(count: number, setColorsPalette: UseState<RGBA[][]>, colorPalette: RGBA[][], setPosition:UseState<number>): NodeJS.Timeout {
  const colorInterval = setInterval(() => {
    let newColors: RGBA[] = [];
    for (let i = 0; i < count; i++) {
      let randomDecimal = randomInteger(0, 16777215);
      var isColor = /^([0-9a-f]{3}){1,2}$/i;
      let colorHex = randomDecimal.toString(16)
      let colorRgb;

      if (!isColor.test(colorHex)) {
        console.log("change")
        randomDecimal = randomInteger(0, 16777215)
        colorRgb = RGBA.fromHex(randomDecimal.toString(16))
      } else {
        colorRgb = hexToRgb(colorHex)
      }

      newColors.push(hexToRgb(randomDecimal.toString(16)));
    }

    setColorsPalette((prevColorsPalette) => [...prevColorsPalette, newColors])
    setPosition((pos:number) => pos + 1)
  }, 300);
  return colorInterval;
}

export function savePalette(colorPalette: RGBA[][], countColors: number, position: number) {
  GlobalFonts.registerFromPath("/home/aprogramb/projects/javascript/color-hunter/src/Iosevka-Medium.ttf")
  const imagePalette = createCanvas(500, 500, 0x01);
  const cntx = imagePalette.getContext("2d");

  let xPosition: number = 80;
  for (let i = 0; i < countColors; i++) {

    cntx.fillStyle = rgbToHex(colorPalette[position]![i]!);
    cntx.fillRect(i * 170, 150, 170, 200);
    cntx.strokeStyle = "white"
    cntx.font = "medium 14px Iosevka"
    cntx.imageSmoothingEnabled= false;
    cntx.textAlign = "center"

    cntx.fillStyle = "white"
    cntx.fillText(`${rgbToHex(colorPalette[position]![i]!)}`, xPosition, 250, 50);
    xPosition += 170

  }

  const buffer = imagePalette.getContent();
  fs.writeFileSync("~/Pictures/palette/image1.png", buffer)


}

