import fs from "fs"
import os from "os"
import { exec } from "child_process"

export function initialize() {

	const compilingIosevkaHeader = exec(`xxd -n iosevka_font  -i ${__dirname}/c/IosevkaFixedSS01-Medium.ttf > ${__dirname}/c/iosevka_font.h`)
	compilingIosevkaHeader.on("close", () => {
		const compilingCModule = exec(`clang -fPIC -shared  -o ${__dirname}/c/canvas.so ${__dirname}/c/canvas.c -I/usr/include/cairo -I/usr/include/libpng16 -I/usr/include/freetype2 -lcairo -lfreetype`)
		compilingCModule.on("close", () => {
			const bufModule: Buffer = fs.readFileSync(`${__dirname}/c/canvas.so`)
			fs.writeFileSync(`${__dirname}/c/canvas.ts`, `export const canvasSo = "${bufModule.toString("base64")}"`)
		})
	})

	const userName: string = os.userInfo().username
	if (!fs.existsSync(`/home/${userName}/.config/color-hunter/`)) {

		const initConf: string = '{\nsavePath:"/home/aprogramb/Pictures/colors/"\n}'

		fs.mkdirSync(`/home/${userName}/.config/color-hunter/`)
		fs.writeFileSync(`/home/${userName}/.config/color-hunter/settings.json`, initConf, "utf8")
	}
}
initialize()
