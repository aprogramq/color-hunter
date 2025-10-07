import { hexToRgb, RGBA } from "@opentui/core";
import { sleepSync } from "bun";
import { cc } from "bun:ffi";
import randomInteger from "random-int";
import type { Dispatch, SetStateAction } from "react";

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
export function randomColor(count: number, setColorsPalette: UseState<RGBA[]>):NodeJS.Timeout {
  let newColors: RGBA[] = [];
  const colorInterval = setInterval(() => {
    for (let i = 0; i < count; i++) {
      const randomDecimal = randomInteger(0, 16777215);
      newColors.push(hexToRgb(randomDecimal.toString(16)));
    }
    setColorsPalette(newColors);
    newColors = [];
  }, 300);
  return colorInterval;
}
