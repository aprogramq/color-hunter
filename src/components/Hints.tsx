import { useContext } from 'react'
import { SizeContext } from '..'

export function Hint({ text}:{text:string}) {
  const size = useContext(SizeContext)
  return (
    <box padding={1}>
      <box top={size.height - 2} left={5} position="absolute" width={30}>
        <text fg="#ffffff" style={{ marginBottom: 1 }}>
          {text}
        </text>
      </box>
    </box>
  )
}
