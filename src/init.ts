import fs from "fs"
import os from "os"

const initConf: string = '{\n"savePath":"/home/aprogramb/Pictures/colors/"\n}'
const userName: string = os.userInfo().username

if (!fs.existsSync(`/home/${userName}/.config/color-hunter/`)) {
	fs.mkdirSync(`/home/${userName}/.config/color-hunter/`)
	fs.writeFileSync(`/home/${userName}/.config/color-hunter/settings.json`, initConf, "utf8")
}
