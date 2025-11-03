import { useContext } from 'react'
import type { sizeT } from '../../types.ts'
import { SizeContext } from '../../index.tsx'

export function StartButton() {
  const size = useContext<sizeT>(SizeContext)
  return (
    <box
      style={
        size.height > 13
          ? { backgroundColor: 'white' }
          : { position: 'absolute', top: size.height - 4.5, backgroundColor: 'black' }
      }
      backgroundColor={size.height > 13 ? 'white' : 'black'}
      padding={1}
      paddingLeft={4}
      paddingRight={4}
      marginTop={1}
    >
      <text fg={size.height > 13 ? '#000000' : '#ffffff'}>[S]tart</text>
    </box>
  )
}
