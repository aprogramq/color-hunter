export function Modal({ activate }: { activate: boolean }) {
  if (activate)
    return (
      <box backgroundColor={'#05df72'} position="absolute" right={0} width={20} height={2}>
        <text fg={'#0d542b'}>Successfully</text>
      </box>
    )
}
