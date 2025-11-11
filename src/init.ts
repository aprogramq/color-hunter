import fs from 'fs'
import os from 'os'

export function initialize() {
    const userName: string = os.userInfo().username
    if (!fs.existsSync(`/home/${userName}/Pictures/colors`))
        fs.mkdirSync(`/home/${userName}/Pictures/colors`, { recursive: true })

    if (!fs.existsSync(`/home/${userName}/.config/color-hunter/settings.json`)) {
        const initConf: string = `{\n"savePath":"/home/${userName}/Pictures/colors/"\n}`
        fs.mkdirSync(`/home/${userName}/.config/color-hunter/`, { recursive: true })
        fs.writeFileSync(`/home/${userName}/.config/color-hunter/settings.json`, initConf, 'utf8')
    }
}
