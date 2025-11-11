import { ConsolePosition, hexToRgb, type ColorInput } from '@opentui/core'
import { useKeyboard } from '@opentui/react'
import { useContext, useState } from 'react'
import fs from 'fs'
import { Modal } from '../Modal'
import { SizeContext } from '../..'
import type { displayT, modalState, sizeT, UseState, modalT, optionsT } from '../../types'
import os from 'os'
import { useKeyboardOptions } from '../../keyboard'
import { Hint } from '../Hints'

export function Options({ actionOptions }: { actionOptions: (display: displayT) => void }) {
  const [positionFocusedInput, setPositionFocusedInput] = useState<number>(1)
  const [selectionMode, setSelectionMode] = useState<boolean>(true)
  const [modal, setModal] = useState<modalT>({
    display: false,
    text: '',
    state: 'success',
  })

  const userName: string = os.userInfo().username
  const pathToSettingFile = `/home/${userName}/.config/color-hunter/settings.json`
  const settings = JSON.parse(fs.readFileSync(pathToSettingFile).toString())

  const [pathInput, setPathInput] = useState<string>(settings['savePath'])
  const [sizeInput, setSizeInput] = useState<string>(settings['sizePalette'].toString())

  const size = useContext<sizeT>(SizeContext)

  function saveNewOptions() {
    let option = ''
    if (positionFocusedInput === 1) option = 'savePath'
    else if (positionFocusedInput === 2) option = 'sizePalette'

    if (parseInt(sizeInput) > 8) {
      setModal({ text: 'Error: Max palette size is 8.', state: 'error', display: true })
      setTimeout(() => setModal((p) => ({ ...p, display: false })), 2000)
      return 0
    }

    settings[option] = option === 'savePath' ? pathInput : parseInt(sizeInput)
    fs.writeFileSync(pathToSettingFile, JSON.stringify(settings, null, 4))
    setModal({ text: 'Successfully', state: 'success', display: true })
    setTimeout(() => setModal((p) => ({ ...p, display: false })), 2000)
    setSelectionMode(true)
  }

  useKeyboardOptions({ selectionMode, setSelectionMode, setPositionFocusedInput, saveNewOptions, actionOptions })

  return (
    <>
      <box alignItems="center" justifyContent="center" flexGrow={1} marginBottom={4}>
        <box justifyContent="center" alignItems="center" marginTop={3} height={'50%'}>
          <ascii-font font="tiny" text="Options" fg={hexToRgb('#7c86ff')} />
          <box
            title="Path to Save"
            width={50}
            marginTop={1}
            borderStyle="single"
            borderColor={positionFocusedInput === 1 ? '#ffffff' : '#222222'}
          >
            <input
              value={pathInput}
              onInput={(val) => setPathInput(val)}
              padding={1}
              focusedTextColor={'#4f46e5'}
              focused={positionFocusedInput === 1 && !selectionMode}
              textColor={positionFocusedInput === 1 && selectionMode ? '#4f46e5' : '#15133D'}
            />
          </box>
          <box
            title="Size of palette"
            width={50}
            marginTop={1}
            borderStyle="single"
            borderColor={positionFocusedInput === 2 ? '#ffffff' : '#222222'}
          >
            <input
              value={sizeInput}
              onInput={(val) => setSizeInput(val)}
              padding={1}
              focusedTextColor={'#4f46e5'}
              focused={positionFocusedInput === 2 && !selectionMode}
              textColor={positionFocusedInput === 2 && selectionMode ? '#4f46e5' : '#15133D'}
              placeholder="Max size: 8"
            />
          </box>
        </box>
      </box>
      <Hint text={selectionMode ? '[E]xit' : '[Esc] to selection mode'} />
      <Modal activate={modal.display} state={modal.state} text={modal.text} />
    </>
  )
}
