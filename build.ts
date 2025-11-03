import fs from 'fs'
import { exec } from 'child_process'
function compiling() {
  console.log('Compiling font header...\n')
  exec(`xxd -n iosevka_font  -i ${__dirname}/src/c/IosevkaFixedSS01-Medium.ttf > ${__dirname}/src/c/iosevka_font.h`).on(
    'close',
    () => {
      console.log('Compiling c module... \n')
      exec(
        `gcc -fPIC -shared  -o ${__dirname}/src/c/canvas.so ${__dirname}/src/c/canvas.c -I/usr/include/cairo -I/usr/include/libpng16 -I/usr/include/freetype2 -lcairo -lfreetype`
      ).on('close', () => {
        const bufModule: Buffer = fs.readFileSync(`${__dirname}/src/c/canvas.so`)
        fs.writeFileSync(`${__dirname}/src/c/canvas.ts`, `export const canvasSo = "${bufModule.toString('base64')}"`)

        console.log('Compiling bun binary... \n')
        exec('bun build ./src/index.tsx --compile --outfile color-hunter').on('close', () => console.log("Done."))
      })
    }
  )
}
compiling()
