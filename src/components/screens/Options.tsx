import { hexToRgb } from '@opentui/core'
import { useKeyboard } from '@opentui/react'
import { useContext, useState } from 'react'
import fs from 'fs'
import { Modal } from '../Modal'
import { SizeContext } from '../..'
import type { displayT, sizeT } from '../../types'
import os from 'os'
import { useKeyboardOptions } from '../../keyboard'
import { Hint } from '../Hints'

export function Options({ actionOptions }: { actionOptions: (display: displayT) => void }) {
  const [positionFocusedInput, setPositionFocusedInput] = useState<number>(1)
  const [selectionMode, setSelectionMode] = useState<boolean>(true)
  const [displayModalMessage, setDisplaySaveMessage] = useState<boolean>(false)

  const userName: string = os.userInfo().username
  const pathToSettingFile = `/home/${userName}/.config/color-hunter/settings.json`
  const jsonFile = JSON.parse(fs.readFileSync(pathToSettingFile).toString())

  const [pathInput, setPathInput] = useState<string>(jsonFile['savePath'])
  const size = useContext<sizeT>(SizeContext)

  function saveNewPathSave() {
    jsonFile['savePath'] = pathInput
    fs.writeFileSync(pathToSettingFile, JSON.stringify(jsonFile, null, 4))
    setDisplaySaveMessage(true)
    setTimeout(() => setDisplaySaveMessage(false), 2000)
    setSelectionMode(true)
  }

  useKeyboardOptions(selectionMode, setSelectionMode, setPositionFocusedInput, saveNewPathSave, actionOptions)
  return (
    <>
      <box alignItems="center" justifyContent="center" flexGrow={1} marginBottom={10}>
        <box justifyContent="center" alignItems="center" marginTop={3} height={'50%'}>
          <ascii-font font="tiny" text="Options" fg={hexToRgb('#7c86ff')} />
          <box title="Path to Save" width={50} marginTop={1} borderStyle="single">
            <input
              focusedBackgroundColor="#222222"
              value={jsonFile.savePath}
              onInput={(val) => setPathInput(val)}
              backgroundColor={positionFocusedInput === 1 ? '#111111' : '#000000'}
              padding={1}
              focused={positionFocusedInput === 1 && !selectionMode}
              textColor={selectionMode ? '#4f46e5' : '#000000'}
            />
          </box>
        </box>
        {/* <Modal activate={displayModalMessage} /> */}
      </box>
      <Hint text={selectionMode ? '[E]xit' : '[Esc] to selection mode'} />
      <Modal activate={displayModalMessage} />
    </>
  )
}
